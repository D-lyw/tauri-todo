// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod service;
mod utils;
pub use commands::todo;
use std::{env, sync::OnceLock};
pub use utils::*;

// use anyhow::Result;
use dotenv::dotenv;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();

#[tokio::main]
async fn main() {
    dotenv().ok();

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_authenticator::init())
        .invoke_handler(tauri::generate_handler![
            todo::add_todo_item,
            todo::query_list_by_page,
            todo::delete_item_by_id,
            todo::switch_item_status
        ])
        .setup(|app| {
            APP.set(app.handle())
                .unwrap_or_else(|_| panic!("Failed to initialize APP"));

            init_db();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
