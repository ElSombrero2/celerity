use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod template;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationProject {
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    create_at: String,
    docker: bool,
    based_template: String,
    board: HashMap<String, Vec<Todo>>
}
