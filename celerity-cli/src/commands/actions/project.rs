use core::{config::types::Configuration, git::Git, projects::types::{template::Template, ConfigurationProject}, utils::{json::Json, printer::SpinnerPrinter}};
use ansi_term::Colour::{Red, Green, Blue, White};
use crate::commands::types::project::Project;

pub struct ProjectAction;

impl ProjectAction {
    fn add_projects_and_save(dir: String, name: String, config: &mut Configuration) -> (String, String) {
        let to_remove = dir.to_owned();
        let to_init = dir.replace(".git/", "");
        config.projects.push(ConfigurationProject { name, path: to_init.to_owned() });
        Configuration::save(config);
        (to_remove, to_init)
    }

    pub fn init(init: Project, config: &mut Configuration){
        let mut printer = SpinnerPrinter::new_dot8("Initialize your project".to_owned());
        match init {
            Project::Init { name, template, path } => {
                printer.print(format!("🔎 Searching for template {}", Blue.bold().paint(&template)));
                let filepath = "./examples/templates/".to_owned() + template.as_str() + ".json";
                if let Some(template) = Json::read::<Template>(filepath) {
                    if let Some (dir) =  Git::clone (
                        template.path.uri,
                        template.path.branch,
                        path.to_owned() + (if path.ends_with('/') {""} else { "/" }) + name.as_str()
                    ) {
                        printer.print("📝 Init and commit your project, almost done!".to_owned());
                        let (to_remove, to_init) = Self::add_projects_and_save(dir, name, config);
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
        }
    }
}
