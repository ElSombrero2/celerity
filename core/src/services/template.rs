use std::fs;
use crate::{
    projects::types::template::Template,
    utils::json::Json
};

pub struct TemplateService;

impl TemplateService {
    pub fn list(template_path: String) -> Vec<Option<Template>>{
        if let Ok(dirs) = fs::read_dir( std::path::Path::new(&template_path)) {
            let res = dirs.map::<Option<Template>, _>(|dir| {
                if let Ok(dir) = dir {
                    return Json::read::<Template>(dir.path().to_string_lossy().to_string());
                }
                Option::None
            });
            return res.collect::<Vec<Option<Template>>>();
        }
        vec![]
    }
}