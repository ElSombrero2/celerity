use core::{config::types::Configuration, git::Git, projects::types::{template::Template, ConfigurationProject, Project, Todo}, utils::{json::Json, printer::SpinnerPrinter}};
use std::{collections::HashMap, fs};
use ansi_term::Colour::{Red, Green, Blue, White};
use chrono::Local;
use comfy_table::Table;
use uuid::Uuid;

pub struct ProjectAction;

impl ProjectAction {
    fn add_projects_and_save(dir: String, name: String, config: &mut Configuration) -> (String, String, String) {
        let to_remove = dir.to_owned();
        let to_init = dir.replace(".git/", "");
        let id = Uuid::new_v4().to_string();
        config.projects.push(ConfigurationProject { id: id.to_owned(), name, path: to_init.to_owned() });
        Configuration::save(config);
        (to_remove, to_init, id)
    }

    fn is_docker(path: String) -> bool{
        let dirs = fs::read_dir(path);
        if let Ok(dirs) = dirs {
            for dir in dirs.flatten() {
                let filename = String::from(dir.file_name().to_str().unwrap_or_default());
                if filename.contains("Dockerfile") || filename.contains("docker-compose"){
                    return true;
                }
            }
        }
        false
    }

    fn init_project(name: String, path: &String, id: String, based_template: String){
        let created_at = Local::now();
        let board: HashMap<String, Vec<Todo>> = HashMap::new();
        println!("{}", path.to_owned() + ".celerity");
        Project::save(Project {
            id, name, board, based_template,
            create_at: created_at.to_string(),
            docker: Self::is_docker(path.to_owned())
        }, path.to_owned() + ".celerity");
    }

    pub fn init(config: &mut Configuration, template: String, name: String, path: String){
        let mut printer = SpinnerPrinter::new_dot8("Initialize your project".to_owned());
        printer.print(format!("🔎 Searching for template {}", Blue.bold().paint(&template)));
                let filepath = "./examples/templates/".to_owned() + template.as_str() + ".json";
                if let Some(template) = Json::read::<Template>(filepath) {
                    if let Some (dir) =  Git::clone (
                        template.path.uri,
                        template.path.branch,
                        path.to_owned() + (if path.ends_with('/') {""} else { "/" }) + name.as_str()
                    ) {
                        printer.print("📝 Init and commit your project, almost done!".to_owned());
                        let (to_remove, to_init, id) = Self::add_projects_and_save(dir, name.to_owned(), config);
                        Self::init_project(name, &to_init, id, template.name);
                        if Git::reinit(to_remove, to_init.to_owned()) {
                            printer.stop(
                                format!("Your project was {} init at {}", 
                                Green.bold().paint("successfully"),
                                White.bold().paint(to_init)
                            ));
                            return;
                        }
                    }
                }
                printer.stop(format!("An {} occured while initialize your project", Red.bold().paint("error")));
    }

    pub fn find_one(config: &Configuration, id: String) -> Option<&ConfigurationProject> {
        config.projects.iter().find(|&project| project.id.eq(&id))
    }

    pub fn list(config: &Configuration, table: &mut Table){
        table.set_header(vec!["Project Name", "Path", "Id"]);
        for project in &config.projects {
            table.add_row(vec![&project.id, &project.name, &project.path ]);
        }
        println!("Project list ({})\n{}", config.projects.len(), table);
    }

}
