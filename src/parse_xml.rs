use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Отправить GET-запрос на API и получить ответ
    let response = reqwest::blocking::get("https://www.cbr-xml-daily.ru/daily_eng_utf8.xml")?;

    // Создать новый XML reader из ответа
    let parser = EventReader::new(BufReader::new(response));

    // Итерировать по событиям XML
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                println!("Найден элемент: {}", name.local_name);
                for attr in attributes {
                    println!("Атрибут: {}={}", attr.name.local_name, attr.value);
                }
            }
            Ok(XmlEvent::Characters(value)) => {
                println!("Значение элемента: {}", value);
            }
            Ok(XmlEvent::EndElement { name }) => {
                println!("Закрыт элемент: {}", name.local_name);
            }
            Err(e) => {
                println!("Ошибка: {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
