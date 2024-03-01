use core::config::types::Configuration;
use std::{fs, io::{Cursor, Read, copy}, path::Path};
use clearscreen::clear;
use viuer::{print_from_file, Config};
use ansi_term::Color::Green;

pub struct UserAction;

impl UserAction {

    fn download_if_not_exists(url: String){
        let path = "./.config/avatar.png";
        if Path::new(path).exists() {
            return;
        }
        let res = reqwest::blocking::get(&url);
        if let Ok(mut res) = res {
            let mut file = fs::File::create(path).unwrap();
            let mut bytes = vec![];
            res.read_to_end(&mut bytes).unwrap_or_default();
            let mut content = Cursor::new(bytes);
            copy(&mut content, &mut file).unwrap_or_default();
        }
    }

    pub async fn show(config: &Configuration) {
        let user = &config.user;
        clear().unwrap_or_default();
        if let Some(user) = user {
            println!("Hello {} ({})\t", 
                Green.bold().paint(user.name.to_owned().unwrap_or_default()),
                Green.bold().paint(user.login.to_owned().unwrap_or_default()),
            );
            
            let img_conf: Config = Config {
                width: Some(8),
                height: Some(4),
                x: 0,
                y: 2,
                ..Default::default()
            };
            Self::download_if_not_exists(user.avatar_url.to_owned().unwrap_or_default());
            print_from_file("./.config/avatar.png", &img_conf).unwrap();
        }else {
            println!("User is not signed, please try to call -g command!");
        }
    }
}
