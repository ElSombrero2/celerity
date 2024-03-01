use core::{config::types::Configuration, projects::types::{Project, Todo}, utils::json::Json};
use std::{collections::HashMap, vec};
use comfy_table::Table;
use super::project::ProjectAction;
use ansi_term::Color::Blue;

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
        if let Some(configuration_project) = ProjectAction::find_one(config, id) {
            let project = Json::read::<Project>(configuration_project.path.to_string() + ".celerity/project.json");
            if let Some(project) = project {
                if project.board.is_empty() {
                    println!("<Your Project Board is Empty>")
                }else {
                    let keys = project.board.keys().collect::<Vec<&String>>();
                    table.set_header(&keys);
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
                }else {
                    project.board.insert(row.to_owned(), vec![todo]);
                }
                Project::save(project, configuration_project.path.to_string() + ".celerity");
                println!("Your {} is add {}", 
                    Blue.bold().paint(title), 
                    Blue.bold().paint(row)
                );
            }
        }
    }



}