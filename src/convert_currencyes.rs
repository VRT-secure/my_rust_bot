use crate::parse_site::*;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use crate::parse_site::{get_currency_struct, get_currencyes_codes};
use crate::bot_logic::URL;
use teloxide::{
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};
pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

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


pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let vec_codes = get_currencyes_codes(URL).await?
        .iter()
        .map(|second_choice| InlineKeyboardButton::callback(second_choice.to_string(), 
        second_choice.to_string()))
        .collect::<Vec<_>>();

    bot.send_message(msg.chat.id, "Выберите первую валюту")
        .reply_markup(InlineKeyboardMarkup::new([vec_codes]))
        .await?;
    dialogue.update(State::ChooseFirstCurrency).await?;

    Ok(())
}

pub async fn receive_first_currency(bot: Bot, dialogue: MyDialogue, query: CallbackQuery) -> HandlerResult {
    if let Some(first_currency) = query.data.clone() {
            let vec_codes = get_currencyes_codes(URL).await?
            .iter()
            .map(|currency| InlineKeyboardButton::callback(currency.to_string(), currency.to_string()))
            .collect::<Vec<_>>();
    
            bot.send_message(query.from.id, "Выберите вторую валюту")
            .reply_markup(InlineKeyboardMarkup::new([vec_codes]))
            .await?;
            dialogue.update(State::ChooseSecondCurrency { first_currency }).await?;

    }else {
        bot.send_message(query.from.id, "Ошибка выбора").await?;
    }

    Ok(())
}


pub async fn receive_second_currency(    
    bot: Bot,
    dialogue: MyDialogue,
    first_currency: String, // Available from `State::ReceiveSecondChoice`.
    query: CallbackQuery,
) -> HandlerResult {
    if let Some(second_curr) = query.data.clone() {
        let vec_codes = get_currencyes_codes(URL).await?
        .iter()
        .map(|second_curr| InlineKeyboardButton::callback(second_curr.to_string(), second_curr.to_string()))
        .collect::<Vec<_>>();

        bot.send_message(query.from.id, "Отправьте число")
        .reply_markup(InlineKeyboardMarkup::new([vec_codes]))
        .await?;
        dialogue.update(State::Amount { first_currency, second_currency: second_curr.to_string() }).await?;
    }
    else {
        bot.send_message(query.from.id, "Отправьте мне текст").await?;
    }
    Ok(())
}

pub async fn receive_amount(
    bot: Bot,
    dialogue: MyDialogue,
    (first_currency, second_currency): (String, String), 
    msg: Message,
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<i64>()) {
        Some(Ok(amount)) => {
            let first_curr = get_currency_struct(URL, &first_currency).await?;
            let second_curr = get_currency_struct(URL, &second_currency).await?;
            let converted_curr = convert(&first_curr, &second_curr, amount)?;
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


fn convert(first_currency: &Currency, second_currency: &Currency, amount: i64) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
    let rate_fist_currency = first_currency.rate.parse::<i64>()?;
    let rate_second_currency = second_currency.rate.parse::<i64>()?;
    let unit_fist_currency = first_currency.unit.parse::<i64>()?;
    let unit_second_currency = second_currency.unit.parse::<i64>()?;
    let unswer = rate_fist_currency / (rate_second_currency / unit_second_currency) / unit_fist_currency * amount;
    Ok(unswer)
}

// async fn send_currencies_keyboard() -> Result<ReplyMarkup> {
//     let vec_codes = get_currencyes_codes(URL).await?;
//     // Создание KeyboardButtons из Vec<String>
//     let kb: Vec<KeyboardButton> = vec_codes
//         .iter()
//         .map(|code| KeyboardButton::new(code))
//         .collect();
//     // Если вы хотите разбить кнопки на несколько рядов, используйте chunks()
//     // Например, разбить на ряды по 3 кнопки
//     let kb_rows: Vec<Vec<KeyboardButton>> = kb
//         .chunks(3)
//         .map(|chunk| chunk.to_vec())
//         .collect();
//     let markup = KeyboardMarkup::new(kb_rows)
//         .resize_keyboard(true);
//     Ok(ReplyMarkup::Keyboard(markup))
// }






