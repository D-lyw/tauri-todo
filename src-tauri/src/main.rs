// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod service;
mod utils;
mod setup;
mod error;
pub use commands::todo;
use std::{env, sync::OnceLock};

// use anyhow::Result;
use dotenv::dotenv;
use tauri::{
    CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, SystemTray, SystemTrayMenu, SystemTrayMenuItem
};

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    dotenv().ok();

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");

    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::default()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu); // configure the menu

    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    // let tray_menu = SystemTrayMenu::new()
    //     .add_item(quit)
    //     .add_native_item(SystemTrayMenuItem::Separator)
    //     .add_item(hide); // insert the menu items here
    // let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .menu(menu)
        // .system_tray(system_tray)
        .plugin(tauri_plugin_store::Builder::default().build())
        // .plugin(tauri_plugin_authenticator::init())
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
