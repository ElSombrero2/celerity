use crate::messages::Messages;
use std::collections::HashMap;
use ansi_term::Color::Blue;
use comfy_table::Table;
use celerity_core::{
    config::types::Configuration,
    projects::types::Todos,
    services::todo::TodoService
};

pub struct TodoControllers;

impl TodoControllers {
    /* --------------------------- PRIVATE ----------------------------- */
    fn get_max_length(boards: &HashMap<String, Todos>) -> usize {
        let keys = boards.keys();
        let mut max = 0;
        for key in keys {
            max = if max < boards.get(key).unwrap().todos.len() {boards.get(key).unwrap().todos.len()} else { max };
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
                let max_lenght = Self::get_max_length(&board);
                let mut sorted_board: Vec<_> = board.iter().collect();
                
                sorted_board.sort_by(|a, b|{ a.1.id.partial_cmp(&b.1.id).unwrap() });

                table.set_header(sorted_board.iter().map(|(s, _)|{ s })).set_width(5);
                for i in 0..max_lenght {
                    let mut rows = Vec::<String>::new();
                    for (_, todo) in &sorted_board {
                        if let Some(found_todo) = todo.todos.get(i) {
                            rows.push(format!("Id: {}\nTitle: {}", found_todo.id, found_todo.title));
                        }else {
                            rows.push(" ".to_string());
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