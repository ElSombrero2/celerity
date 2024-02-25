use std::fs;

use serde::{Deserialize, Serialize};

use crate::utils::json::read_json;

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

pub fn list(folder: String) -> Vec<Option<Template>>{
    match fs::read_dir( std::path::Path::new(&folder)) {
        Ok(dirs) => {
            let res = dirs.map::<Option<Template>, _>(|dir| {
                if let Ok(dir) = dir {
                    return read_json::<Template>(dir.path().to_string_lossy().to_string());
                }
                Option::None
            });
            return res.collect::<Vec<Option<Template>>>();
        },
        _ => {}
    }
    vec![]
}
