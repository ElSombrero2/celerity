use std::{fs, io::{copy, Cursor, Read}, path::Path};
use reqwest::blocking::get;
use crate::{config::types::{Configuration, User}, errors::CelerityError};

pub struct UserService;

impl UserService {
    fn download_if_not_exists(url: String){
        let path = "./.config/avatar.png";
        if Path::new(path).exists() {
            return;
        }
        if let Ok(mut res) = get(&url) {
            let mut file = fs::File::create(path).unwrap();
            let mut bytes = vec![];
            res.read_to_end(&mut bytes).unwrap_or_default();
            let mut content = Cursor::new(bytes);
            copy(&mut content, &mut file).unwrap_or_default();
        }
    }

    pub fn get_user(config: &Configuration) -> Result<&User, CelerityError>{
        let user = &config.user;
        if let Some(user) = user {
            Self::download_if_not_exists(user.avatar_url.to_owned().unwrap_or_default());
            return Ok(user);
        }
        Err(CelerityError::NotFound)
    }
}