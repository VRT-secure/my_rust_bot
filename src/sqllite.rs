use std::{io, env};
use tokio::task;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};


fn insert_into_table(conn: &Connection, table_name: &str, column_name: &str, data: &str) {
    let sql = format!("INSERT INTO {} ({}) VALUES (?1)", table_name, column_name);
    let result = conn.execute(
        &sql,
        params![data],
    );

    match result {
        Ok(_) => println!("Запись добавлена!"),
        Err(error) => println!("Возникла ошибка {}", error),
    }
}

fn delete_from_table(conn: &Connection, table_name: &str, id: u32) {
    let sql = format!("DELETE FROM {} WHERE id = ?", table_name);
    let result = conn.execute(sql.as_str(), params![id]);
    match result {
        Ok(_) => println!("Запись удалена!"),
        Err(error) => println!("Возникла ошибка {}", error),
    }
}

fn select_from_table(conn: &Connection, table_name: &str){
    let sql = format!("SELECT * FROM {}", table_name);
    let mut stmt = conn.prepare(&sql).unwrap();
    let rows = stmt.query_map(params![], |row| {
        Ok((row.get(0).unwrap(), row.get(1).unwrap()))
    }).unwrap();

    for row in rows {
        let (id, task): (u32, String) = row.unwrap();
        println!("id: {:?} task: {:?}", id, task);
    }
}

// pub fn create_or_connect_to_db(table_name: &str, column_name: &str) -> Connection{
//     let exe_file_path = env::current_exe().unwrap();
//     let path = exe_file_path.parent().unwrap().join(format!("{}.sqlite3", table_name));
//     println!("Путь к файлу БД: {:?}", path);
//     let conn = Connection::open(path).unwrap();

//     let sql = format!(
//         "CREATE TABLE IF NOT EXISTS {} ( 
//         id INTEGER PRIMARY KEY, 
//         {} TEXT NOT NULL )", 
//         table_name, column_name);
    
//     match conn.execute(&sql,params![],) {
//         Ok(_) => {println!("База данных подключена!")},
//         Err(error) => {println!("Ошибка создания базы данных {}", error)},
//     };
//     conn
// }


// pub fn create_or_connect_to_db(table_name: &str, column_names: &[(&str, &str)]) -> Connection {
//     let exe_file_path = env::current_exe().unwrap();
//     let path = exe_file_path.parent().unwrap().join(format!("{}.sqlite3", table_name));
//     println!("Путь к файлу БД: {:?}", path);
//     let conn = Connection::open(path).unwrap();

//     let columns = column_names
//         .iter()
//         .map(|(name, data_type)| format!("{} {}", name, data_type))
//         .collect::<Vec<String>>()
//         .join(", ");

//     let sql = format!(
//         "CREATE TABLE IF NOT EXISTS {} ( 
//         id INTEGER PRIMARY KEY, 
//         {} )",
//         table_name, columns
//     );

//     match conn.execute(&sql, params![],) {
//         Ok(_) => println!("База данных подключена!"),
//         Err(error) => println!("Ошибка создания базы данных {}", error),
//     };
//     conn
// }

pub async fn create_or_connect_to_db(
    table_name: &str,
    column_names: &[(&str, &str)],
) -> Result<SqlitePool, sqlx::Error> {
    let exe_file_path = env::current_exe().unwrap();
    let path = exe_file_path.parent().unwrap().join(format!("{}.sqlite3", table_name));
    println!("Путь к файлу БД: {:?}", path);

    let connection_options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

    let pool = SqlitePool::connect_with(connection_options).await?;

    let columns = column_names
        .iter()
        .map(|(name, data_type)| format!("{} {}", name, data_type))
        .collect::<Vec<String>>()
        .join(", ");

    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
        id INTEGER PRIMARY KEY,
        {} );",
        table_name, columns
    );

    sqlx::query(&sql).execute(&pool).await?;

    println!("База данных подключена!");
    Ok(pool)
}


fn update_in_table(conn: &Connection, table_name: &str, column_name: &str, id: u32, new_task: &str){
    let sql = format!("UPDATE {} SET {} = ?1 WHERE id = ?2", table_name, column_name);
    match conn.execute(&sql, &[new_task, &id.to_string()]) {
        Ok(_) => {println!("Запись обновлена!")},
        Err(error) => {println!("Возникла ошибка {}", error)},
    };
}

fn search_record(conn: &Connection, table_name: &str, column_name: &str, keyword: &str){
    let sql = format!("SELECT * FROM {} WHERE {} LIKE '%{}%'", table_name, column_name, keyword);
    let mut stmt = conn.prepare(&sql).unwrap();
    let rows = stmt.query_map(params![], |row| {
        Ok((row.get(0).unwrap(), row.get(1).unwrap()))
    }).unwrap();
    
    for row in rows {
        let (id, task): (u32, String) = row.unwrap();
        println!("id: {:?} task: {:?}", id, task);
    }
}
