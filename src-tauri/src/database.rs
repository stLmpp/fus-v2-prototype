use std::fs;
use std::path::Path;

use dirs::home_dir;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn get_database() -> Result<DatabaseConnection, DbErr> {
    let database_file_path =
        Path::new(&home_dir().unwrap().into_os_string().into_string().unwrap())
            .join(".fus-v2")
            .join("data")
            .join("database.db")
            .into_os_string()
            .into_string()
            .expect("Could not get database file path");
    let database_url = format!("sqlite:{}?mode=rwc", database_file_path);
    let mut database_options = ConnectOptions::new(&database_url);
    database_options
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);
    let db = Database::connect(database_options).await?;
    return Ok(db);
}

pub async fn setup_database() -> Result<DatabaseConnection, DbErr> {
    fs::create_dir_all(
        Path::new(&home_dir().unwrap().into_os_string().into_string().unwrap())
            .join(".fus-v2")
            .join("data"),
    )
    .expect("Could not create FUS directory");
    return get_database().await;
}
