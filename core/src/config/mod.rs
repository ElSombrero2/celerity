use std::fs;
use crate::{git::github::{find_me, types::OAuth2Response}, utils::json::read_json};
use self::types::Configuration;

pub mod types;

impl Configuration {

    pub fn new(github_token: Option<String>) -> Self {
        Configuration{
            github_token,
            user: Option::None,
        }
    }

    pub fn register(oauth2_response: OAuth2Response) -> bool {
        if let Some(mut configuration) = read_json::<Configuration>(".config/configuration.json".to_string()) {
            configuration.github_token = Some(oauth2_response.access_token);
            return Configuration::save(configuration);
        }
        let mut configuration = Configuration::new(Some(oauth2_response.access_token.clone()));
        configuration.user = find_me(configuration.github_token.to_owned().unwrap_or_default());
        Configuration::save(configuration)
    }
    
    pub fn save(configuration: Configuration) -> bool {
        let content = serde_json::to_string_pretty(&configuration).unwrap_or_default();
        fs::create_dir(".config").unwrap_or_default();
        fs::write(".config/configuration.json", content).is_ok()
    }
    
}
