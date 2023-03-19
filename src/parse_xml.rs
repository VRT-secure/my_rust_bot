use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub fn get_dayli_crb_money(char_code_to_find: &str, url: &str) -> Result<(String, String, String, String, String), Box<dyn std::error::Error>> {
    // Отправить GET-запрос на API и получить ответ
    let response = reqwest::blocking::get(url)?;

    // Создать новый XML reader из ответа
    let parser = EventReader::new(BufReader::new(response));
    let mut num_code = "".to_string();
    let mut char_code = "".to_string();
    let mut nominal = "".to_string();
    let mut name_element = "".to_string();
    let mut value = "".to_string();
    let mut characters_v = "".to_string();


    let mut tuple_money = (num_code.clone(), char_code.clone(), nominal.clone(), name_element.clone(), value.clone());
    let mut foud_char_code = false;
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
                    "nominal" => nominal = format!("{}: {}", nominal, characters_v),
                    "name" => name_element = format!("{}: {}", name_element, characters_v),
                    "value" => value = format!("{}: {}", value, characters_v),
                    "valute" => {
                        if foud_char_code{
                            tuple_money = (num_code.clone(), char_code.clone(), nominal.clone(), name_element.clone(), value.clone());
                            return Ok(tuple_money)
                        }
                    },
                    _ => {}
                }
            }

            Err(e) => {
                println!("Ошибка: {}", e);
                break;
            }
            _ => {}
        }

        if char_code_to_find == characters_v{
            foud_char_code = true;
        }

    }

    Ok(tuple_money)
}



pub fn get_char_codes_of_currencies(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    // Отправить GET-запрос на API и получить ответ
    let response = reqwest::blocking::get(url)?;
    
    // Создать новый XML reader из ответа
    let parser = EventReader::new(BufReader::new(response));

    let mut vec_char_code = Vec::new();
    let mut characters_v = "".to_string();


    for e in parser {
        match e {
            Ok(XmlEvent::Characters(value)) => {
                characters_v = value;
            }
            
            Ok(XmlEvent::EndElement { name, .. }) => {
                match  name.local_name.to_lowercase().as_str(){
                    "charcode" => vec_char_code.push(characters_v.clone()),
                    _ => {}
                }
            }

            Err(e) => {
                println!("Ошибка: {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(vec_char_code)
}