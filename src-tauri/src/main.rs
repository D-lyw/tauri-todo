// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod error;
mod menu;
mod service;
mod setup;
mod utils;
pub use commands::todo;
use std::env;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .menu(menu::window_menu())
        .system_tray(menu::system_tray_menu())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            todo::add_todo_item,
            todo::query_list_by_page,
            todo::delete_item_by_id,
            todo::switch_item_status
        ])
        .setup(setup::setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
