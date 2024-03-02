use std::{collections::HashMap, env, fs, process::Command};
use chrono::Local;
use uuid::Uuid;
use crate::{
    config::types::Configuration, errors::CelerityError, git::Git, projects::types::{
        template::Template,
        ConfigurationProject,
        Project, 
        Todo
    }, utils::json::Json
};


pub struct ProjectService;

impl ProjectService {

    // --------------------------- PRIVATE -----------------------------

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
        Project::save(Project {
            id, name, board, based_template,
            created_at: created_at.to_rfc3339(),
            docker: Self::is_docker(path.to_owned())
        }, path.to_owned() + &env::var("CELERITY_FILE").unwrap_or_default());
    }

     // --------------------------- PRIVATE -----------------------------

    pub fn init<A, F, G>(
        config: &mut Configuration,
        template: String,
        name: String,
        path: String, 
        on_clone: F,
        on_init: G,
        arg: &mut A
    ) -> Result<(), CelerityError> where F: FnOnce(&mut A), G: FnOnce(&mut A)
    {
        let filepath = env::var("TEMPLATE_FOLDER").unwrap_or_default() + template.as_str() + ".json";
        if let Some(template) = Json::read::<Template>(filepath) {
            on_clone(arg);
            if let Some (dir) =  Git::clone (
                template.path.uri,
                template.path.branch,
                path.to_owned() + (if path.ends_with('/') {""} else { "/" }) + name.as_str()
            ) {
                on_init(arg);
                let (to_remove, to_init, id) = Self::add_projects_and_save(dir, name.to_owned(), config);
                Self::init_project(name, &to_init, id, template.name);
                if Git::reinit(to_remove, to_init.to_owned()) {
                    return Ok(());
                }
                return Err(CelerityError::IOError)
            }
            return Err(CelerityError::NotFound);
        }
        Err(CelerityError::IOError)
    }

    pub fn find_one(config: &Configuration, id: String) -> Result<&ConfigurationProject, CelerityError> {
        if let Some(project) = config.projects.iter().find(|&project| project.id.eq(&id) || project.name.eq(&id)) {
            return Ok(project);
        }
        Err(CelerityError::NotFound)
    }

    pub fn get_project(configuration: &Configuration, id: String) -> Option<(Project, &ConfigurationProject)> {
        if let Ok(configuration_project) = ProjectService::find_one(configuration, id) {
            if let Some(project) = Json::read::<Project>(
                configuration_project.path.to_string() + 
                &env::var("CELERITY_FILE").unwrap_or_default()
            ) {
                return Some((project, configuration_project));
            }
        }
        Option::None
    }

    pub fn open(config: &Configuration, id: String) -> Result<(), CelerityError>{
        if let Ok(project) = Self::find_one(config, id) {
            if cfg!(target_os = "windows") {
                if Command::new("code.cmd").args([&project.path]).spawn().is_ok() {
                    return Ok(())
                }   
            }else if Command::new("code").args([&project.path]).spawn().is_ok() {
                return Ok(())
            }
        } 
        Err(CelerityError::NotFound)
    }

    pub fn get_all(config: &Configuration) -> &Vec<ConfigurationProject> {
        return &config.projects;
    }
}