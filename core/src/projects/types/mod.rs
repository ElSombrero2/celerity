use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::json::Json;

pub mod template;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationProject {
    pub id: String,
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: String,
    pub title: String,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Todo {
            id: Uuid::new_v4().to_u128_le().to_string(),
            title
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub create_at: String,
    pub docker: bool,
    pub based_template: String,
    pub board: HashMap<String, Vec<Todo>>
}

impl Project {
    pub fn save(project: Project, path: String) -> bool {
        let content = serde_json::to_string_pretty(&project).unwrap_or_default();
        Json::save(content, path, "project.json".to_owned())
    }
}