#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;
use app::controllers::{configuration::get_configuration, template::get_templates};
mod app;

fn main() {
    dotenvy::from_filename(".env.desktop").unwrap_or_default();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_configuration,
            get_templates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
