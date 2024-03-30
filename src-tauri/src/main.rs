// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, CustomMenuItem, Manager, Menu,  MenuItem, Submenu};

let quit: CustomMenuItem = CustomMenuItem::new("quit".to_string(), "Quit").unwrap();
let close: CustomMenuItem = CustomMenuItem::new("close".to_string(), "Close").unwrap();
let submenu: Submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close)).unwrap();
let menu: Menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu)
    .unwrap();

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(app: AppHandle, name: &str) -> String {
    app.emit_all("event_name", "eventpayload").unwrap();
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
