use dotenv_codegen::dotenv;

use super::project::ProjectService;
use std::collections::HashMap;
use crate::{
    config::types::Configuration,
    errors::CelerityError,
    projects::types::{
        ConfigurationProject,
        Project,
        Todo, Todos
    },
};

pub struct TodoService;

impl TodoService {

    /* --------------------------- PRIVATE ----------------------------- */

    fn save(project: Project, configuration_project: &ConfigurationProject){
        Project::save(
            project, 
            configuration_project.path.to_string() +
            dotenv!("CELERITY_FILE")
        );
    }

    fn get_max(board: &HashMap<String, Todos>) -> (usize, String) {
        let mut max = 0;
        let mut current_key = String::default();
        for key in board.keys() {
            if let Some(todos) = board.get(key) {
                if max <= todos.id {
                    max = todos.id;
                    current_key = key.to_owned();
                }
            }
        }
        (max, current_key)
    }

    /* --------------------------- PRIVATE ----------------------------- */

    pub fn get_board(config: &Configuration, id: String) -> Result<HashMap<String, Todos>, CelerityError> {
        if let Some((project, ..)) = ProjectService::get_project(config, id) {
            return Ok(project.board);
        }
        Err(CelerityError::NotFound)
    }

    pub fn add(config: &Configuration, id: String, row: String, title: String) -> Result<(), CelerityError> {
        let todo = Todo::new(title.to_owned());
        if let Some((mut project, configuration_project)) = ProjectService::get_project(config, id) {
            if let Some(board) = project.board.get_mut(&row) {
                board.todos.push(todo);
                Self::save(project, configuration_project);
                return Ok(());
            }
        }
        Err(CelerityError::IOError)
    }

    pub fn add_row(config: &Configuration, id: String, row: String) -> Result<(), CelerityError> {
        if let Some((mut project, configuration_project)) = ProjectService::get_project(config, id) {
            if project.board.get(&row).is_none() {
                project.board.insert(row.to_owned(), Todos { id: Self::get_max(&project.board).0 + 1, todos: vec![] });
                Self::save(project, configuration_project);
                return Ok(());
            }
        }
        Err(CelerityError::IOError)
    }

    pub fn remove_row(config: &Configuration, id: String, row: String) -> Result<(), CelerityError>{
        if let Some((mut project, configuration_project)) = ProjectService::get_project(config, id) {
            project.board.remove(&row);
            Self::save(project, configuration_project);
            return Ok(());
        }
        Err(CelerityError::IOError)
    }

    pub fn move_task(config: &Configuration, project_id: String, task_id: String, from: String, to: String) -> Result<(), CelerityError>{
        if let Some((mut project, configuration_project)) = ProjectService::get_project(config, project_id) {
            let mut todo: Option<Todo> = Option::None;
            if let Some(origin) = project.board.get_mut(&from){
                if let Some(index) = origin.todos.iter().position(|todo|{ todo.id.eq(&task_id) }) {
                    todo = Some(origin.todos.remove(index));
                }
            }
            if let Some(target) = project.board.get_mut(&to) {
                if let Some(todo) = todo {
                    target.todos.push(todo);
                    Self::save(project, configuration_project);
                    return Ok(());
                }
            }
            
        }
        Err(CelerityError::IOError)
    }

    pub fn remove_task(config: &Configuration, project_id: String, task_id: String, from: String) -> Result<(), CelerityError>{
        if let Some((mut project, configuration_project)) = ProjectService::get_project(config, project_id) {
            if let Some(board) = project.board.get_mut(&from){
                if let Some(index) = board.todos.iter().position(|todo|{ todo.id.eq(&task_id) }) {
                    board.todos.remove(index);
                    Self::save(project, configuration_project);
                    return Ok(());
                }
            }
        }
        Err(CelerityError::IOError)
    }
}