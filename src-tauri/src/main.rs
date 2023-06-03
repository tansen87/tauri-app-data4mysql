// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data4mysql;

use data4mysql::{download, close_splashscreen};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            close_splashscreen, 
            download
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
