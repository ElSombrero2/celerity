use celerity_core::{
    config::types::Configuration,
    services::configuration::ConfigurationService, utils::__dirname
};
use dotenv_codegen::dotenv;

#[tauri::command]
pub fn get_configuration() -> Option<Configuration>{
    if let Ok(cfg) = ConfigurationService::get_configuration(__dirname(dotenv!("CONFIG_FILE"))) {
        return Some(cfg);
    } 
    Option::None
}