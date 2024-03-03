use celerity_core::{
    config::types::Configuration,
    services::project::ProjectService,
    utils::printer::SpinnerPrinter
};
use chrono::DateTime;
use comfy_table::{Row, Table};
use indoc::indoc;
use ansi_term::Color::{Green, Red};

use crate::messages::Messages;


pub struct ProjectController;

impl ProjectController {
    pub fn init_project(config: &mut Configuration, template: String, name: String, path: String){
        let mut printer = SpinnerPrinter::new_dot8("🔎 Searching for your template...".to_string());
        match ProjectService::init::<SpinnerPrinter, _, _>(config, template, name, path,
            |printer| printer.print("✏️ Cloning your project from github".to_string()),
            |printer| printer.print("📝 Initialize your project to the target folder...".to_string()),
            &mut printer,
        ) {
            Ok(_) => {
                printer.stop(format!(
                    "Your project was {}",
                    Green.bold().paint("successfully initialized")
                ))
            }
            Err(_) => {
                printer.stop(format!(
                    "{} occured while fetching your project, please remove the created folder and re run the command",
                    Red.bold().paint("An error")
                ))
            },
        }
    }

    pub fn show_all(config: &Configuration, table: &mut Table){
        let projects = ProjectService::get_all(config);
        table.set_header(vec!["Id", "Name", "Folder"]);
        for project in projects {
            table.add_row(vec![
                &project.id,
                &project.name,
                &project.path
            ]);
        }
        println!("Projects ({}): \n{}", projects.len(), table);
    }

    pub fn find_one(config: &Configuration, id: String, table: &mut Table){
        if let Some((project, project_config)) = ProjectService::get_project(config, id) {
            table.set_header(vec!["Name", "Details"]);
            let created_at = DateTime::parse_from_rfc3339(&project.created_at);

            if let Ok(created_at) = created_at {
                table.add_rows(
                    vec![
                        Row::from(vec!["Id", &project.id]),
                        Row::from(vec!["Name", &project.name]),
                        Row::from(vec!["Folder", &project_config.path]),
                        Row::from(vec!["Created At", &created_at.format("%d/%m/%Y %H:%M").to_string()]),
                        Row::from(vec!["Docker", if project.docker {"✔️"} else {"❌"}]),
                    ]
                );
                println!("{table}")
            }

        }else { println!("Your project was not found, please try to see all your project with the --projects commands") }
    }

    pub fn open(config: &Configuration, id: String){
        if ProjectService::open(config, id).is_ok() {
            Messages::success_message("VS Code is open".to_owned());
            return;
        }
        let msg = indoc! {"opening VS Code, make sure
        Visual Studio code is installed and added
        to your environment variable PATH"};
        Messages::error_message(String::from(msg));
    }
}