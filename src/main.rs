mod parse_xml;
use parse_xml::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.cbr-xml-daily.ru/daily_eng_utf8.xml";
    let vec_code = get_char_codes_of_currencies(url)?;
    let tup_currencie = get_dayli_crb_money("JPY", url)?;
    println!("{:?}", vec_code);
    println!("{:?}", tup_currencie);
    Ok(())
}

