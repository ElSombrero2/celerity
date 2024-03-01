use std::fs;

use serde::{Deserialize, Serialize};

use crate::utils::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplatePath {
    pub source: String,
    pub uri: String,
    pub branch: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub author: String,
    pub path: TemplatePath,
    pub description: String
}

impl Template {
    pub fn template_list(folder: String) -> Vec<Option<Template>>{
        if let Ok(dirs) = fs::read_dir( std::path::Path::new(&folder)) {
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
