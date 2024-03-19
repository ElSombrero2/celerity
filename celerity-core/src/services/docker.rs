use std::{io::{BufRead, BufReader}, os::windows::process::CommandExt, process::{Command, Stdio}};

use crate::{config::types::Configuration, docker::{DockerCompose, DockerComposeCommand, DockerServices}, errors::CelerityError, utils::yaml::Yaml};

use super::project::ProjectService;

pub struct DockerService;

impl DockerService {

    fn parse_services_from_json(services: &mut Vec<DockerServices>, content: String){
        if !content.is_empty() {
            let jsons = content.split('\n').collect::<Vec<&str>>();
            for json in jsons {
                if let Ok(service) = serde_json::from_str::<DockerComposeCommand>(json) {
                    services.push(DockerServices {
                        name: service.Service,
                        state: service.State,
                        project: Some(service.Project),
                        ports: Some(service.Ports),
                        size: service.Size,
                        id: Some(service.ID),
                        image: service.Image
                    })
                }
                
            }                   
        }
    }

    fn parse_services_from_yaml(services: &mut Vec<DockerServices>, path: String){
        if let Some(yml) = Yaml::read::<DockerCompose>(path) {
            for key in yml.services.keys() {
                if services.iter().any(|e| e.name.eq(key)) { continue }
                services.push(DockerServices::new(
                    key.to_owned(),
                    String::from("never_started"),
                    String::from("0B"),
                    String::from('-')
                ));
            }
        }
    }

    pub fn get_services(config: &Configuration, id: String) -> Result<Vec<DockerServices>, CelerityError>{
        if let Some((project, project_cfg)) = ProjectService::get_project(config, id) {
            if project.docker {
                if let Ok(output) = Command::new("docker-compose")
                .args(["-f", &format!("{}docker-compose.yml", project_cfg.path), "ps", "-a", "--format", "json"])
                .output() {
                    if let Ok(mut content) = String::from_utf8(output.stdout) {
                        let mut services = Vec::<DockerServices>::new();
                        content = content.trim().to_string();
                        Self::parse_services_from_json(&mut services, content);
                        Self::parse_services_from_yaml(&mut services, format!("{}docker-compose.yml", project_cfg.path));
                        return Ok(services)
                    }
                }
            }
        }
        Err(CelerityError::NotFound)
    }

    pub fn exec<F>(config: &Configuration, id: String, command: String, on_exec: F) where F: Fn(String) -> bool {
        if let Some((_, project_cfg)) = ProjectService::get_project(config, id) {
            if let Ok(mut cmd) = Command::new("docker-compose").args([
                "-f",
                &format!("{}docker-compose.yml", project_cfg.path)
            ])
            .raw_arg(command)
            .stdout(Stdio::piped())
            .spawn() {
                let stdout = cmd.stdout.as_mut().unwrap();
                let stdout_reader = BufReader::new(stdout);
                let stdout_lines = stdout_reader.lines();

                for line in stdout_lines.into_iter().map_while(Result::ok) {
                    if on_exec(line.to_owned()) {
                        break;
                    }
                }
            }
        }
    }
}