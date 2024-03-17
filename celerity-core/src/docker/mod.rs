use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerComposeImage{}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerCompose{
    pub version: String,
    pub services: HashMap<String, DockerComposeImage>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerComposeCommand {
    pub Service: String,
    pub Size: String,
    pub Project: String,
    pub Ports: String,
    pub ID: String,
    pub State: String,
    pub Image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerServices {
    pub name: String,
    pub state: String,
    pub project: Option<String>,
    pub ports: Option<String>,
    pub size: String,
    pub id: Option<String>,
    pub image: String,
}

impl DockerServices {
    pub fn new(name: String, state: String, size: String, image: String) -> Self {
        DockerServices {
            name,
            project: Some(String::from('-')),
            size,
            state,
            ports: Some(String::from('-')),
            id: Some(String::from('-')),
            image: image,
        }
    }
}
