use core::{git::{clone_project, reinit}, projects::types::Template, utils::json::read_json};
use super::types::project::Project;
use ansi_term::Colour::{Red, Green, Blue, White};
use spinoff::{spinners, Color, Spinner};

fn show_message(message: String, spinner: &mut Spinner){
    println!();
    spinner.update_text(message);
}

pub fn init_project(init: Project){
    let mut sp = Spinner::new(spinners::Dots8, "Initialize your project", Color::White);
    match init {
        Project::Init { name, template, path } => {
            show_message(format!("🔎 Searching for template {}", Blue.bold().paint(&template)), &mut sp);
            let filepath = "./examples/templates/".to_owned() + template.as_str() + ".json";
            if let Some(template) = read_json::<Template>(filepath) {
                if let Some (dir) =  clone_project(
                    template.path.uri,
                    template.path.branch,
                    path.to_owned() + (if path.ends_with('/') {""} else { "/" }) + name.as_str()
                ) {
                    show_message("📝 Init and commit your project, almost done!".to_owned(), &mut sp);
                    let to_remove = dir.to_owned();
                    let to_init = dir.replace(".git/", "");
                    if reinit(to_remove, to_init.to_owned()) {
                        sp.stop_with_message(
                            &format!("Your project was {} init at {}", 
                            Green.bold().paint("successfully"),
                            White.bold().paint(to_init)
                        ));
                        return;
                    }
                }
            }
            sp.stop_with_message(&format!("An {} occured while initialize your project", Red.bold().paint("error")));
        }
    }
}