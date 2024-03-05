use std::env;
use celerity_core::{projects::types::template::Template, services::template::TemplateService};

#[tauri::command]
pub fn get_templates() -> Option<Vec<Option<Template>>>{
    Some(TemplateService::list(env::var("TEMPLATE_FOLDER").unwrap_or_default()))
}