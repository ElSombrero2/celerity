use std::{thread, time::Duration};
use actix_web::{App, HttpServer};
use crate::{git::github::url::ApiUrl, utils::browser::Browser};
use super::login;

pub struct Server;

impl Server {
    pub async fn start(){
        let server = HttpServer::new(|| {
            App::new()
                .service(login)
        });
        let run = server.shutdown_timeout(1).bind("127.0.0.1:8100").unwrap().run();
        let _ = tokio::join!(run, Browser::open(ApiUrl::authorization()));
    }
    
    pub fn kill(millis: u64){
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(millis));
            std::process::exit(0);
        });
    }
}