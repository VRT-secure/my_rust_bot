use teloxide::prelude::*;

async fn my_function() {
    pretty_env_logger::init();
    log::debug!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_message(msg.chat.id, "hello").await?;
        Ok(())
    }).await;
}

