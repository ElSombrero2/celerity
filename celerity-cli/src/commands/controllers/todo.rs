use crate::messages::Messages;
use std::collections::HashMap;
use ansi_term::Color::Blue;
use comfy_table::Table;
use core::{
    config::types::Configuration,
    projects::types::Todo,
    services::todo::TodoService
};

pub struct TodoControllers;

impl TodoControllers {
    /* --------------------------- PRIVATE ----------------------------- */
    fn get_max_length(boards: &HashMap<String, Vec<Todo>>) -> usize {
        let keys = boards.keys();
        let mut max = 0;
        for key in keys {
            max = if max < boards.get(key).unwrap().len() {boards.get(key).unwrap().len()} else { max };
        }
        max
    }

    /* --------------------------- PRIVATE ----------------------------- */

    pub fn show_board(config: &Configuration, id: String, table: &mut Table) {
        if let Ok(board) = TodoService::get_board(config, id) {
            if board.is_empty() {
                println!("Your board is empty, please add rows and tasks");
                return;
            }else {
                let keys = board.keys().collect::<Vec<&String>>();
                table.set_header(&keys).set_width(5);
                let max_lenght = Self::get_max_length(&board);
                for i in 0..max_lenght {
                    let mut rows = Vec::<String>::new();
                    for key in &keys {
                        if let Some(todo) = board.get(key.to_owned()) {
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
                return;
            }
        }
        Messages::error_message("in getting your kanban".to_string());
    }

    pub fn add(config: &Configuration, id: String, row: String, title: String) {
        if TodoService::add(config, id, row.to_owned(), title.to_owned()).is_ok() {
            Messages::success_message(
                format!(
                    "Your task \"{}\" to was added to {}", 
                    Blue.bold().paint(title),
                    Blue.bold().paint(row)
                )
            );
            return;
        }
        Messages::error_message(
            "in adding your your task, please check if your row exists"
            .to_string()
        )
    }

    pub fn add_row(config: &Configuration, id: String, row: String) {
        if TodoService::add_row(config, id, row.to_owned()).is_ok() {
            Messages::success_message(
                format!(
                    "Your row \"{}\" was created!", 
                    Blue.bold().paint(row),
                )
            );
            return;
        }
        Messages::error_message(
            "in adding your your row, maybe this name was already taken"
            .to_string()
        )
    }

    pub fn remove_row(config: &Configuration, id: String, row: String) {
        if TodoService::remove_row(config, id, row.to_owned()).is_ok() {
            Messages::success_message(
                format!(
                    "Your row {} was removed",
                    Blue.bold().paint(row),
                )
            );
            return;
        }
        Messages::error_message(
            "in removing your row"
            .to_string()
        )
    }

    pub fn move_task(config: &Configuration, project_id: String, task_id: String, from: String, to: String) {
        if TodoService::move_task(config, project_id, task_id, from.to_owned(), to.to_owned()).is_ok() {
            Messages::success_message(
                format!(
                    "Your task moved from {} to {}",
                    Blue.bold().paint(from),
                    Blue.bold().paint(to),
                )
            );
            return;
        }
        Messages::error_message(
            "moving your task"
            .to_string()
        )
    }

    pub fn remove_task(config: &Configuration, project_id: String, task_id: String, from: String) {
        if TodoService::remove_task(config, project_id, task_id, from).is_ok() {
            Messages::success_message("Your task was removed from the board!".to_string());
            return;
        }
        Messages::error_message(
            "moving your task"
            .to_string()
        )
    }


}