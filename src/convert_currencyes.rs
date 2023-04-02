use crate::parse_site::*;
use anyhow::{Result};


fn convert(first_currency: Currency, second_currency: Currency, amount: i64) -> Result<i64> {
    let rate_fist_currency = first_currency.rate.parse::<i64>()?;
    let rate_second_currency = second_currency.rate.parse::<i64>()?;
    let unit_fist_currency = first_currency.unit.parse::<i64>()?;
    let unit_second_currency = second_currency.unit.parse::<i64>()?;
    let unswer = rate_fist_currency / (rate_second_currency / unit_second_currency) / unit_fist_currency * amount;
    Ok(unswer)
}


