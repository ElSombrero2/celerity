use serde::{Deserialize, Serialize};

pub mod template;

#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    pub path: String,
    pub name: String,
}
