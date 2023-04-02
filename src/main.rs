use teloxide::prelude::*;
mod parse_site;
use anyhow::{Result};
mod bot_logic;



#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting buttons bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(bot_logic::message_handler))
        .branch(Update::filter_callback_query().endpoint(bot_logic::callback_handler))
        .branch(Update::filter_inline_query().endpoint(bot_logic::inline_query_handler));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
    Ok(())
}
