use std::sync::mpsc;

use celerity_core::{
    config::types::Configuration,
    docker::DockerServices,
    services::docker::DockerService
};
use tauri::Window;

use super::types::CommandsOutput;

#[tauri::command]
pub fn get_services(config: Configuration, project: String) -> Option<Vec<DockerServices>>{
    if let Ok(services) = DockerService::get_services(&config, project) {
        return Some(services);
    }
    Option::None
}

#[tauri::command]
pub async fn exec(window: Window, config: Configuration, project: String, command: String){
    let (tx, rx) = mpsc::channel::<bool>();

    window.listen("end", move |_| { let _ = tx.send(true); });

    DockerService::exec(&config, project, command, |output| {
        println!("{}", &output);
        let _ = window.emit("message", CommandsOutput{ output });
        if rx.try_recv().is_ok() { return true; }
        false
    });

    let _ = window.emit("ended", ());
}
