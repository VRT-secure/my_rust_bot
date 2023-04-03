use anyhow::{Result};
mod parse_site;
mod bot_logic;
mod convert_currencyes;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use crate::convert_currencyes::*;


#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting buttons bot...");

    let bot = Bot::from_env();

    let convert_currencyes_handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(start))
        .branch(dptree::case![State::ChooseFirstCurrency].endpoint(receive_first_currency))
        .branch(dptree::case![State::ChooseSecondCurrency { first_currency }].endpoint(receive_second_currency))
        .branch(dptree::case![State::Amount { first_currency, second_currency }].endpoint(receive_amount));

    let command_handler = Update::filter_message().endpoint(bot_logic::message_handler);
    let callback_handler = Update::filter_callback_query().endpoint(bot_logic::callback_handler);
    let inline_query_handler = Update::filter_callback_query().endpoint(bot_logic::inline_query_handler);
    

    let handler = dptree::entry()
        .branch(convert_currencyes_handler)
        .branch(command_handler)
        .branch(callback_handler)
        .branch(inline_query_handler);

    Dispatcher::builder(bot, handler)
    .dependencies(dptree::deps![InMemStorage::<State>::new()]) // Добавьте эту строку для предоставления InMemStorage<State>
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
    Ok(())
}
