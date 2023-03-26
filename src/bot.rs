mod parse_xml;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup, InlineKeyboardButtonKind};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init();
    log::info!("Starting currencies bot...");

    let bot = Bot::from_env();
    let yes_button = InlineKeyboardButton::new("Да".to_string(), InlineKeyboardButtonKind::CallbackData("yes".to_string()));
    let no_button = InlineKeyboardButton::new("Нет".to_string(), InlineKeyboardButtonKind::CallbackData("no".to_string()));
    
    // Создаем объект с разметкой для двух кнопок
    // let markup = InlineKeyboardMarkup::new(vec![vec![yes_button, no_button]]);
    let keyboard = InlineKeyboardMarkup::default().append_row(vec![yes_button, no_button]);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // Отправляем сообщение с кнопками в чат
        bot.send_message(msg.chat.id, "Debian versions:").reply_markup(keyboard).await?;
        bot.send_message(msg.chat.id, "hello").await?;
        Ok(())
    }).await;

    Ok(())
}

