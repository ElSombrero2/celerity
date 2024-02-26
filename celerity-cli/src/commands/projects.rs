use core::{git::{clone_project, reinit}, projects::types::Template, utils::json::read_json};
use super::types::project::Project;
use ansi_term::Colour::{Red, Green};

pub fn init_project(init: Project){
    match init {
        Project::Init { name, template, path } => {
            let filepath = "./examples/templates/".to_owned() + template.as_str() + ".json";
            if let Some(template) = read_json::<Template>(filepath) {
                if let Some (dir) =  clone_project(
                    template.path.uri,
                    template.path.branch,
                    path.to_owned() + (if path.ends_with('/') {""} else { "/" }) + name.as_str()
                ) {
                    let to_remove = dir.to_owned();
                    let to_init = dir.replace(".git/", "");
                    if reinit(to_remove, to_init.to_owned()) {
                        println!("Your project was {} init at {}", Green.bold().paint("successfully"), to_init);
                        return;
                    }
                }
            }
            println!("An {} occured while initialize your project", Red.bold().paint("error"));
        }
    }
}