use comfy_table::{presets::UTF8_FULL_CONDENSED, Table};
use dotenv_codegen::dotenv;
use messages::Messages;
use clap::Parser;
use commands::{
    controllers::{
        docker::DockerController, project::ProjectController, template::TemplateController, todo::TodoControllers, user::UserController
    },
    types::project::ProjectCommand
};
use celerity_core::{
    auth::server::Server,
    git::github::types::Github,
    services::configuration::ConfigurationService, utils::__dirname
};

mod commands;
mod messages;

#[tokio::main]
async fn main(){
    let cmd = commands::Commands::parse();
    let mut table = Table::new();
    table.set_width(3);
    table.apply_modifier(UTF8_FULL_CONDENSED);

    if cmd.login { Server::start().await; }
    if let Ok(mut config) = ConfigurationService::get_configuration(__dirname(dotenv!("CONFIG_FILE"))) {
        if !Github::ping(config.github_token.to_owned().unwrap_or_default()){
            Messages::expiration_message();
            Server::start().await;
            println!("Reexecute your command");
        }
        match cmd {
            commands::Commands { templates: true, .. } => {
                TemplateController::show_templates(
                    __dirname(dotenv!("TEMPLATE_FOLDER")), 
                    &mut table
                )
            },
            commands::Commands { projects: true, .. } => ProjectController::show_all(&config, &mut table),
            commands::Commands { project: Some(project), ..  } => ProjectController::find_one(&config, project, &mut table),
            commands::Commands { me: true, .. } => UserController::show_user(&config),
            commands::Commands { kanban: Some(project_id), .. } => TodoControllers::show_board(&config, project_id, &mut table),
            
            commands::Commands { subcommand: Some(subcommand), ..} => {
                match subcommand {
                    ProjectCommand::Init { name, path, template } => ProjectController::init_project(&mut config, template, name, path),
                    ProjectCommand::AddTodo { project, row, title } => TodoControllers::add(&config, project, row, title),
                    ProjectCommand::AddRow { project, row } => TodoControllers::add_row(&config, project, row),
                    ProjectCommand::RemoveRow { project, row } => TodoControllers::remove_row(&config, project, row),
                    ProjectCommand::RemoveTask { project, row, task_id } => TodoControllers::remove_task(&config, project, task_id, row),
                    ProjectCommand::MoveTask { project, task_id, origin_row, target_row } => TodoControllers::move_task(&config, project, task_id, origin_row, target_row),
                    ProjectCommand::Open { project } => ProjectController::open(&config, project),
                    ProjectCommand::Services { project } => DockerController::get_services(&config, project, &mut table),
                    ProjectCommand::Cmd { project, command } => DockerController::exec(config, project, command),
                }
            }
            _ => Messages::lib_description()
        }
    }else {
        Messages::lib_description(); 
        Messages::config_error(); 
    }

}
