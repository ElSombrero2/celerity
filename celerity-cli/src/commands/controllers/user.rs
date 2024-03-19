use clearscreen::clear;
use dotenv_codegen::dotenv;
use viuer::{print_from_file, Config};
use ansi_term::Color::Green;
use celerity_core::{
    config::types::Configuration,
    services::user::UserService, utils::__dirname
};

pub struct UserController;

impl UserController {

    pub fn show_user(config: &Configuration){
        if let Ok(user) = UserService::get_user(config) {
            clear().unwrap_or_default();
            let config = Config { x: 5, y: 0, width: Some(8), height: Some(4), ..Default::default() };
            print_from_file(__dirname(dotenv!("AVATAR_FILE")).to_string(), &config)
            .unwrap_or_default();
            println!("\nHello {}\nAlso known as {} 😎", 
                Green.bold().paint(user.name.to_owned().unwrap_or_default()),
                Green.bold().paint(user.login.to_owned().unwrap_or_default())
            );
        }
    }
}