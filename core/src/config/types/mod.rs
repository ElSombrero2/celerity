use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Configuration{
    pub path: String,
    pub github_token: Option<String>,
}
