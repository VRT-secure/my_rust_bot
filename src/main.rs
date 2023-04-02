use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Me, Currency,
    },
    utils::command::BotCommands,
};
mod parse_site;
use anyhow::{Result};

static URL: &str = "https://www.cbr.ru/eng/currency_base/daily/";



#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "THe bot show currency prices. These commands are supported:")]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Start")]
    Start,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting buttons bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
    Ok(())
}

/// Creates a keyboard made by buttons in a big column.
async fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let vec_currency = match parse_site::get_currencyes_codes(URL).await{
        Ok(v) => {v},
        Err(e) => {
            eprintln!("Error downloading XML file: {}", e);
            vec!["Error".to_string()]
        },
    };
    for versions in vec_currency.chunks(3) {
        let row = versions
            .iter()
            .map(|version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

/// Parse the text wrote on Telegram and check if that text is a valid command
/// or not, then match the command. If the command is `/start` it writes a
/// markup with the `InlineKeyboardMarkup`.
async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<()> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                // Just send the description of all commands.
                bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            }
            Ok(Command::Start) => {
                // Create a list of buttons and send them.
                let keyboard = make_keyboard().await;
                bot.send_message(msg.chat.id, "Choice currency:").reply_markup(keyboard).await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }

    Ok(())
}

async fn inline_query_handler(
    bot: Bot,
    q: InlineQuery,
) -> Result<()> {
    let choose_cyrrency = InlineQueryResultArticle::new(
        "0",
        "Chose currency",
        InputMessageContent::Text(InputMessageContentText::new("Currencyes:")),
    )
    .reply_markup(make_keyboard().await);

    bot.answer_inline_query(q.id, vec![choose_cyrrency.into()]).await?;

    Ok(())
}

/// When it receives a callback from a button it edits the message with all
/// those buttons writing a text with the selected Debian version.
///
/// **IMPORTANT**: do not send privacy-sensitive data this way!!!
/// Anyone can read data stored in the callback button.
async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<()> {
    if let Some(char_code) = q.data {
        let currency = parse_site::get_currency_struct(URL, char_code.as_str()).await?;
        // Tell telegram that we've seen this query, to remove 🕑 icons from the
        // clients. You could also use `answer_callback_query`'s optional
        // parameters to tweak what happens on the client side.
        bot.answer_callback_query(q.id).await?;
       
        let text = format!("Char code: {}\nunit: {}\ncurrency: {}\nrate: {}", currency.char_code, currency.unit, currency.curr, currency.rate);
       
       
        // Edit text of the message to which the buttons were attached
        if let Some(Message { id, chat, .. }) = q.message {
            bot.edit_message_text(chat.id, id, text).await?;
        } else if let Some(id) = q.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }

        log::info!("You chose: {}", char_code);
    }

    Ok(())
}