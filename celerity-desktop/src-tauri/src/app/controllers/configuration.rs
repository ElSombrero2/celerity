use std::env;
use celerity_core::{
    config::types::Configuration,
    services::configuration::ConfigurationService
};

#[tauri::command]
pub fn get_configuration() -> Option<Configuration>{
    if let Ok(cfg) = ConfigurationService::get_configuration(env::var("CONFIG_FILE").unwrap_or_default()) {
        return Some(cfg);
    } 
    Option::None
}