use scraper::{Html, Selector};
use reqwest::get;
use std::io;
use anyhow::{Result, Error};


async fn get_currencyes(url: &str) -> Result<Vec<String>>{
    let page = get(url).await?.text().await?;
    let document = Html::parse_document(&page);
    let mut unswer_vec = Vec::new();
    let selector = Selector::parse("tr").unwrap();
    for element in document.select(&selector).into_iter() {
        let mut tmp_vec = element.text().collect::<Vec<_>>();
        tmp_vec.remove(0);
        tmp_vec.remove(0);
        for elem in tmp_vec {
            let tmp_elem = elem.trim().to_string();
            if !tmp_elem.is_empty(){
                unswer_vec.push(tmp_elem);
            }
        }
    }
    unswer_vec.remove(0);
    unswer_vec.remove(0);
    unswer_vec.remove(0);
    unswer_vec.remove(0);
    Ok(unswer_vec)
} 




pub struct Currency {
    pub char_code: String,
    pub unit: String,
    pub curr: String,
    pub rate: String,
}

// pub async fn get_currency_struct(url: &str, charcode: &str) -> Result<Currency, Box<dyn Error>>{
//     let vec_currencyes = get_currencyes(url).await?;
//     if let Some(index) = vec_currencyes.iter().position(|x| x == charcode) {
//         let unswer = Currency {
//             char_code: vec_currencyes[index].clone(),
//             unit: vec_currencyes[index + 1].clone(),
//             curr: vec_currencyes[index + 2].clone(),
//             rate: vec_currencyes[index + 3].clone(),
//         };      
//         Ok(unswer)      
//     } else {
//         Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Currency with char code {} not found", charcode))))
//     }
// }

pub async fn get_currency_struct(url: &str, charcode: &str) -> Result<Currency> {
    let vec_currencyes = get_currencyes(url).await?;
    if let Some(index) = vec_currencyes.iter().position(|x| x == charcode) {
        let unswer = Currency {
            char_code: vec_currencyes[index].clone(),
            unit: vec_currencyes[index + 1].clone(),
            curr: vec_currencyes[index + 2].clone(),
            rate: vec_currencyes[index + 3].clone(),
        };
        Ok(unswer)
    } else {
        Err(Error::msg(format!("Currency with char code {} not found", charcode))) // Используйте `Error::msg` для создания ошибки
    }
}


pub async fn get_currencyes_codes(url: &str) -> Result<Vec<String>>{
    let vec_currencyes = get_currencyes(url).await?;
    let mut vec_char_codes = Vec::new();
    let mut i = 0;
    while i < vec_currencyes.len() {
        vec_char_codes.push(vec_currencyes[i].clone());
        i += 4;
    }
    Ok(vec_char_codes)
}
