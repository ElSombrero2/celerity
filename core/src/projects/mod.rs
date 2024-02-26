use std::fs;

use crate::utils::json::read_json;

use self::types::Template;

pub mod types;

pub fn template_list(folder: String) -> Vec<Option<Template>>{
    if let Ok(dirs) = fs::read_dir( std::path::Path::new(&folder)) {
        let res = dirs.map::<Option<Template>, _>(|dir| {
            if let Ok(dir) = dir {
                return read_json::<Template>(dir.path().to_string_lossy().to_string());
            }
            Option::None
        });
        return res.collect::<Vec<Option<Template>>>();
    }
    vec![]
}