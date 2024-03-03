// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;

use celerity_core::{config::types::Configuration, services::configuration::ConfigurationService};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> String {
    format!("Hello, {}! You've been greeted from Rust!", env::var("CONFIG_FILE").unwrap_or_default())
}

#[tauri::command]
fn get_configuration() -> Option<Configuration> {
    if let Ok(cfg) = ConfigurationService::get_configuration(env::var("CONFIG_FILE").unwrap_or_default()) {
        return Some(cfg);
    }
    Option::None
}

fn main() {
    dotenvy::from_filename(".env.desktop").unwrap_or_default();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_configuration,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
