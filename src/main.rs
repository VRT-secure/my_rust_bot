use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tuple_money = get_dayli_crb_money("USD")?;
    println!("{:?}", tuple_money);
    Ok(())
}


fn get_dayli_crb_money(char_code_to_find: &str) -> Result<(String, String, String, String, String), Box<dyn std::error::Error>> {
    // Отправить GET-запрос на API и получить ответ
    let response = reqwest::blocking::get("https://www.cbr-xml-daily.ru/daily_eng_utf8.xml")?;

    // Создать новый XML reader из ответа
    let parser = EventReader::new(BufReader::new(response));
    let mut num_code = "".to_string();
    let mut char_code = "".to_string();
    let mut nominal = "".to_string();
    let mut name_element = "".to_string();
    let mut value = "".to_string();
    let mut characters_v = "".to_string();


    let mut tuple_money = (num_code.clone(), char_code.clone(), nominal.clone(), name_element.clone(), value.clone());
    // Итерировать по событиям XML
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match  name.local_name.to_lowercase().as_str(){
                    "numcode" => num_code = name.local_name,
                    "charcode" => char_code = name.local_name,
                    "nominal" => nominal = name.local_name,
                    "name" => name_element = name.local_name,
                    "value" => value = name.local_name,
                    _ => {}
                }
            }

            Ok(XmlEvent::Characters(value)) => {
                characters_v = value;
            }
            
            Ok(XmlEvent::EndElement { name, .. }) => {
                match  name.local_name.to_lowercase().as_str(){
                    "numcode" => num_code = format!("{}: {}", num_code, characters_v),
                    "charcode" => char_code = format!("{}: {}", char_code, characters_v),
                    "nominal" => nominal = format!("{}: {}", num_code, characters_v),
                    "name" => name_element = format!("{}: {}", num_code, characters_v),
                    "value" => value = format!("{}: {}", num_code, characters_v),
                    _ => {}
                }
            }

            Err(e) => {
                println!("Ошибка: {}", e);
                break;
            }
            _ => {}
        }

        if char_code_to_find == char_code{
            tuple_money = (num_code.clone(), char_code.clone(), nominal.clone(), name_element.clone(), value.clone());
        }

    }

    Ok(tuple_money)
}

