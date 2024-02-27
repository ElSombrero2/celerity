use core::{config::types::Configuration, git::github::{ping, server::start_server}, utils::json::read_json};
use clap::Parser;
use comfy_table::Table;
use commands::{projects::init_project, templates::show_template_list};
use texts::{config_error, expiration_message, lib_description};

mod commands;
mod texts;

#[tokio::main]
async fn main(){
    dotenvy::dotenv().expect("Error while loading env");
    let cmd = commands::Commands::parse();
    let mut table = Table::new();
    if cmd.github_login { start_server().await; }
    if let Some(config) = read_json::<Configuration>(".config/configuration.json".to_owned()) {
        if !ping(config.github_token.unwrap_or_default()){
            expiration_message();
            start_server().await;
            println!("Reexecute your command");
        }
        match cmd {
            commands::Commands {templates: true, ..} => show_template_list(&mut table),
            commands::Commands {init: Some(init), ..} => init_project(init),
            _ => lib_description()
        }
    }else {
        lib_description(); 
        config_error(); 
    }
}
