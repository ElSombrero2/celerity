#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;
use app::controllers::{
    configuration::get_configuration,
    template::get_templates,
    project::{get_project, get_documentation},
    docker::{get_services, exec}
};
mod app;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_configuration,
            get_templates,
            get_project,
            get_documentation,
            get_services,
            exec
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
