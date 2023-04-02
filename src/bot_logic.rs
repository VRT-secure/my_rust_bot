
use anyhow::{Result};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Me,ReplyMarkup, KeyboardButton, KeyboardMarkup
    },
    utils::command::BotCommands,
};
use crate::parse_site;


static URL: &str = "https://www.cbr.ru/eng/currency_base/daily/";

#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "The bot show currency prices. These commands are supported:")]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Start")]
    Start,
}

/// Creates a keyboard made by buttons in a big column.
async fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let vec_currency = match parse_site::get_currencyes_codes(URL).await{
        Ok(v) => {v},
        Err(e) => {
            eprintln!("Error parse site: {}", e);
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
pub async fn message_handler(
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
                // Create keyboard buttons and send them.
                bot.send_message(msg.chat.id, "Отправьте команду").reply_markup(send_keyboard()).send().await?;
            }

            Err(_) => {
                // Handle non-command text messages here.
                text_handler(&bot, &msg, text).await?;
            }
        }

    }

    Ok(())
}

async fn text_handler(bot: &Bot, msg: &Message, text: &str) -> Result<()>{
    let keyboard = make_keyboard().await;
    match text {
        "Узнать курс фалюты" => {
            bot.send_message(msg.chat.id, "Выберите валюту").reply_markup(keyboard).send().await?;
        }
        "Калькулятор валют" => {
            bot.send_message(msg.chat.id, "Выберите первую валюту").reply_markup(keyboard).send().await?;
            let keyboard_2 = make_keyboard().await;
            bot.send_message(msg.chat.id, "Выберите вторую валюту").reply_markup(keyboard_2).send().await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Command not found!").await?;
        }
    }
    
    Ok(())
}


pub async fn inline_query_handler(
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
pub async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<()> {
    if let Some(char_code) = q.data {
        let currency = parse_site::get_currency_struct(URL, char_code.as_str()).await?;
        let text = format!("Char code: {}\nunit: {}\ncurrency: {}\nrate: {}", currency.char_code, currency.unit, currency.curr, currency.rate);
        
        
        
        // Tell telegram that we've seen this query, to remove 🕑 icons from the
        // clients. You could also use `answer_callback_query`'s optional
        // parameters to tweak what happens on the client side.
        bot.answer_callback_query(q.id).await?;
       
       
        // Edit text of the message to which the buttons were attached
        if let Some(Message {id, chat, .. }) = q.message {
            bot.edit_message_text(chat.id, id, text).await?;
        }

        log::info!("You chose: {}", char_code);
    }

    Ok(())
}

fn send_keyboard() -> ReplyMarkup {
    let kb = vec![
        KeyboardButton::new("Узнать курс фалюты"),
        KeyboardButton::new("Калькулятор валют"),
    ];
 
    let markup = KeyboardMarkup::new(vec![kb])
    .resize_keyboard(true);
 
    ReplyMarkup::Keyboard(markup)
 }

