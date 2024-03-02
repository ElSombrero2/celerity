use core::{config::types::Configuration, projects::types::{Project, Todo}, utils::json::Json};
use std::collections::HashMap;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use super::project::ProjectAction;
use ansi_term::Color::{Blue, Red};

pub struct TodoAction;

impl TodoAction {

    fn get_max_length(boards: &HashMap<String, Vec<Todo>>) -> usize {
        let keys = boards.keys();
        let mut max = 0;
        for key in keys {
            max = if max < boards.get(key).unwrap().len() {boards.get(key).unwrap().len()} else { max };
        }
        max
    }

    pub fn list(config: &Configuration, id: String, table: &mut Table){
        table.load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_width(2);
        if let Some(configuration_project) = ProjectAction::find_one(config, id) {
            let project = Json::read::<Project>(configuration_project.path.to_string() + ".celerity/project.json");
            if let Some(project) = project {
                if project.board.is_empty() {
                    println!("<Your Project Board is Empty>")
                }else {
                    let keys = project.board.keys().collect::<Vec<&String>>();
                    table.set_header(&keys).set_width(5);
                    let max_lenght = Self::get_max_length(&project.board);
                    for i in 0..max_lenght {
                        let mut rows = Vec::<String>::new();
                        for key in &keys {
                            if let Some(todo) = project.board.get(key.to_owned()) {
                                if let Some(found_todo) = todo.get(i) {
                                    rows.push(format!("Id: {}\nTitle: {}", found_todo.id, found_todo.title));
                                }else {
                                    rows.push(" ".to_string());
                                }
                            }
                        }
                        table.add_row(rows);
                    }
                    println!("{}", table);
                }
            }
        }
    }

    pub fn add(config: &Configuration, id: String, row: String, title: String){
        let todo = Todo::new(title.to_owned());
        if let Some(configuration_project) = ProjectAction::find_one(config, id) {
            let project = Json::read::<Project>(configuration_project.path.to_string() + ".celerity/project.json");
            if let Some(mut project) = project {
                if let Some(board) = project.board.get_mut(&row) {
                    board.push(todo);
                    Project::save(project, configuration_project.path.to_string() + ".celerity");
                    println!("Your {} is add {}", 
                        Blue.bold().paint(title), 
                        Blue.bold().paint(row)
                    );
                    return;
                }
            }
        }
        println!(
            "{} to add your task to {} does not exist, please add this row with the {} command",
            Red.bold().paint("Failed"),
            Blue.bold().paint(title),
            Blue.bold().paint("row")
        );
    }

    pub fn add_row(config: &Configuration, id: String, row: String){
        if let Some(configuration_project) = ProjectAction::find_one(config, id) {
            let project = Json::read::<Project>(configuration_project.path.to_string() + ".celerity/project.json");
            if let Some(mut project) = project {
                if project.board.get(&row).is_none() {
                    project.board.insert(row.to_owned(), vec![]);
                    Project::save(project, configuration_project.path.to_string() + ".celerity");
                    println!("{} was added to your board", Blue.bold().paint(row));
                    return;
                }
            }
        }
        println!(
            "{}, try again or try with another row name (Maybe this row already exists)!",
            Red.bold().paint("An error occured"),
        );
    }

    pub fn remove_row(config: &Configuration, id: String, row: String){
        if let Some(configuration_project) = ProjectAction::find_one(config, id) {
            let project = Json::read::<Project>(configuration_project.path.to_string() + ".celerity/project.json");
            if let Some(mut project) = project {
                project.board.remove(&row);
                Project::save(project, configuration_project.path.to_string() + ".celerity");
                println!("{} was removed from your board", Blue.bold().paint(row));
            }
        }
    }

    pub fn move_task(config: &Configuration, project_id: String, task_id: String, from: String, to: String){
        if let Some(configuration_project) = ProjectAction::find_one(config, project_id) {
            let project = Json::read::<Project>(configuration_project.path.to_string() + ".celerity/project.json");
            if let Some(mut project) = project {
                let mut todo: Option<Todo> = Option::None;
                if let Some(origin) = project.board.get_mut(&from){
                    if let Some(index) = origin.iter().position(|todo|{ todo.id.eq(&task_id) }) {
                        todo = Some(origin.remove(index));
                    }
                }
                if let Some(target) = project.board.get_mut(&to) {
                    if let Some(todo) = todo {
                        let title = todo.title.to_owned();
                        target.push(todo);
                        Project::save(project, configuration_project.path.to_string() + ".celerity");
                        println!("Your task {} was successfully moved to {}!", 
                            Blue.bold().paint(title),
                            Blue.bold().paint(to)
                        );
                        return;
                    }
                }
                
            }
        }
        println!(
            "{}, try again or try with another row name (Maybe this row does not exist)!",
            Red.bold().paint("An error occured"),
        );
    }

    pub fn remove_task(config: &Configuration, project_id: String, task_id: String, from: String){
        if let Some(configuration_project) = ProjectAction::find_one(config, project_id) {
            let project = Json::read::<Project>(configuration_project.path.to_string() + ".celerity/project.json");
            if let Some(mut project) = project {
                if let Some(origin) = project.board.get_mut(&from){
                    if let Some(index) = origin.iter().position(|todo|{ todo.id.eq(&task_id) }) {
                        let todo = origin.remove(index);
                        Project::save(project, configuration_project.path.to_string() + ".celerity");
                        println!("Your task {} was successfully removed!", 
                            Blue.bold().paint(todo.title)
                        );
                    }
                }
            }
        }
        println!(
            "{}, can't remove this task, maybe it does not exists!",
            Red.bold().paint("An error occured"),
        );
    }

}