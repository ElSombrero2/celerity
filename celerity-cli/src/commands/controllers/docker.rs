use celerity_core::{config::types::Configuration, services::docker::DockerService};
use comfy_table::Table;

pub struct DockerController;

impl DockerController {
    pub fn get_services(config: &Configuration, id: String, table: &mut Table){
        if let Ok(services) = DockerService::get_services(config, id) {
            table.set_header(vec![
                "Id",
                "Name",
                "Project",
                "Image",
                "State",
                "Ports",
                "Size",
            ]);
            for service in services {
                table.add_row(vec![
                    service.id.unwrap_or_default(),
                    service.name,
                    service.project.unwrap_or_default(),
                    service.image,
                    service.state,
                    service.ports.unwrap_or_default(),
                    service.size,
                ]);
            }
            println!("{}", table);
        }
    }

    pub fn exec(config: &Configuration, id: String, command: String){
        DockerService::exec(config, id, command, |output| {
            println!("{}", output);
        });
    }
}