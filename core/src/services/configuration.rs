use crate::{config::types::Configuration, errors::CelerityError, utils::json::Json};

pub struct ConfigurationService;

impl ConfigurationService {
    pub fn get_configuration(config_path: String) -> Result<Configuration, CelerityError>{
        if let Some(cfg) = Json::read::<Configuration>(config_path){ Ok(cfg) }
        else { Err(CelerityError::NotFound) }       
    }
}