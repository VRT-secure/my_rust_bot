use crate::parse_site::*;
use anyhow::{Result};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
type MyDialogue = Dialogue<State, InMemStorage<State>>;
use crate::parse_site::get_currency_struct;
use crate::bot_logic::URL;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ChooseFirstCurrency,
    ChooseSecondCurrency {
        first_currency: String,
    },
    Amount {
        first_currency: String,
        second_currency: String,
    }
}


pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, "Выберите первую валюту").await?;
    dialogue.update(State::ChooseFirstCurrency).await?;
    Ok(())
}


pub async fn receive_first_currency(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Выберите вторую валюту").await?;
            dialogue.update(State::ChooseSecondCurrency { first_currency: text.to_string() }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Отправьте мне текст").await?;
        }
    }

    Ok(())
}

pub async fn receive_second_currency(bot: Bot, dialogue: MyDialogue, first_currency: String, msg: Message) -> Result<()> {
    match msg.text() {
        Some(text) => {
            
            bot.send_message(msg.chat.id, "Выберите вторую валюту").await?;
            dialogue.update(State::Amount { first_currency, second_currency: text.to_string() }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Отправьте мне текст").await?;
        }
    }

    Ok(())
}

pub async fn receive_amount(
    bot: Bot,
    dialogue: MyDialogue,
    first_currency: String, 
    second_currency: String,
    msg: Message,
) -> Result<()> {
    match msg.text().map(|text| text.parse::<i64>()) {
        Some(Ok(amount)) => {
            let first_curr: Currency = get_currency_struct(URL, &first_currency).await?;
            let second_curr: Currency = get_currency_struct(URL, &second_currency).await?;
            let converted_curr: i64 = convert(&first_curr, &second_curr, amount)?;
            let unswer = format!("Валюта {} конвертированная в {}, в количестве {}", first_curr.char_code, second_curr.char_code, converted_curr);
            bot.send_message(msg.chat.id, unswer).await?;
            dialogue.exit().await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me a number.").await?;
        }
    }
    Ok(())
}


fn convert(first_currency: &Currency, second_currency: &Currency, amount: i64) -> Result<i64> {
    let rate_fist_currency = first_currency.rate.parse::<i64>()?;
    let rate_second_currency = second_currency.rate.parse::<i64>()?;
    let unit_fist_currency = first_currency.unit.parse::<i64>()?;
    let unit_second_currency = second_currency.unit.parse::<i64>()?;
    let unswer = rate_fist_currency / (rate_second_currency / unit_second_currency) / unit_fist_currency * amount;
    Ok(unswer)
}


