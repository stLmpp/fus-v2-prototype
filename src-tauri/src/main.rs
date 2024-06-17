// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs::home_dir;
use futures::executor::block_on;
// use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DbErr};
use std::fs;
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

async fn connect_db() -> Result<(), DbErr> {
    fs::create_dir_all(
        Path::new(&home_dir().unwrap().into_os_string().into_string().unwrap())
            .join(".fus-v2")
            .join("data"),
    )
    .expect("Could not create FUS directory");
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
    // Migrator::up(&db, None).await?;
    Ok(())
}

fn main() {
    if let Err(err) = block_on(connect_db()) {
        panic!("{}", err);
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
