use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Path {
    pub source: String,
    pub uri: String,
    pub branch: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub author: String,
    pub path: Path,
    pub description: String
}