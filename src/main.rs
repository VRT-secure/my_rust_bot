mod parse_site;
mod bot_logic;
mod convert_currencyes;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use crate::convert_currencyes::*;


#[tokio::main]
async fn main() -> HandlerResult {
    pretty_env_logger::init();
    log::info!("Starting buttons bot...");

    let bot = Bot::from_env();


    Dispatcher::builder(bot, bot_logic::schema())
    .dependencies(dptree::deps![InMemStorage::<State>::new()]) // Добавьте эту строку для предоставления InMemStorage<State>
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
    Ok(())
}
