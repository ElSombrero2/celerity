use celerity_core::{
    config::types::Configuration,
    projects::types::Project,
    services::project::ProjectService
};

#[tauri::command]
pub fn get_project(config: Configuration, id: String) -> Option<Project>{
    if let Some((project, _)) = ProjectService::get_project(&config, id) {
        return Some(project);
    }
    Option::None
}

#[tauri::command]
pub fn get_documentation(config: Configuration, id: String) -> Option<String> {
    if let Ok(content) = ProjectService::get_documentation(&config, id) {
        return Some(content);
    }
    Option::None
}