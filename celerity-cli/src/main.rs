use core::{auth::server::Server, config::types::Configuration, git::github::types::Github, utils::json::Json};
use commands::{actions::{project::ProjectAction, template::TemplateAction, todo::TodoAction, user::UserAction}, types::project::ProjectCommand};
use messages::Messages;
use clap::Parser;
use comfy_table::Table;

mod commands;
mod messages;

#[tokio::main]
async fn main(){
    dotenvy::dotenv().expect("Error while loading env");
    let cmd = commands::Commands::parse();
    let mut table = Table::new();
    if cmd.github_login { Server::start().await; }
    if let Some(mut config) = Json::read::<Configuration>(".config/configuration.json".to_owned()) {
        if !Github::ping(config.github_token.to_owned().unwrap_or_default()){
            Messages::expiration_message();
            Server::start().await;
            println!("Reexecute your command");
        }
        match cmd {
            commands::Commands { templates: true, .. } => TemplateAction::list(&mut table),
            commands::Commands { projects: true, .. } => ProjectAction::list(&config, &mut table),
            commands::Commands { project: Some(project), ..  } => println!("Project {}", project),
            commands::Commands { me: true, .. } => UserAction::show(&config).await,
            commands::Commands { todo_list: Some(project_id), .. } => TodoAction::list(&config, project_id, &mut table),
            
            commands::Commands { subcommand: Some(subcommand), ..} => {
                match subcommand {
                    ProjectCommand::Init { name, path, template } => ProjectAction::init(&mut config, template, name, path),
                    ProjectCommand::AddTodo { id, row, title } => TodoAction::add(&config, id, row, title),
                }
            }
            _ => Messages::lib_description()
        }
    }else {
        Messages::lib_description(); 
        Messages::config_error(); 
    }
}
