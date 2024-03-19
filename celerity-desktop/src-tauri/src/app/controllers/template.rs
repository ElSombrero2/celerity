use celerity_core::{
    projects::types::template::Template,
    services::template::TemplateService, utils::__dirname
};
use dotenv_codegen::dotenv;

#[tauri::command]
pub fn get_templates() -> Option<Vec<Option<Template>>>{
    Some(TemplateService::list(__dirname(dotenv!("TEMPLATE_FOLDER"))))
}