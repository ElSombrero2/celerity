use std::env;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::{auth::server::Server, git::github::types::{Github, GithubClient}};

#[derive(Deserialize, Debug)]
pub struct OAuth2Payload {
    pub code: Option<String>,
    pub state: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OAuth2Response {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}


#[get("/login")]
async fn login(payload: web::Query<OAuth2Payload>) -> impl Responder {
    if let (Some(code), Some(state)) = (&payload.code, &payload.state) {
        if state.eq("celerity.io") {
            if Github::authenticate(GithubClient {
                code: code.to_string(),
                client_id: env::var("CLIENT_ID").unwrap_or_default(),
                client_secret: env::var("CLIENT_SECRET").unwrap_or_default(),
                redirect_uri: env::var("REDIRECT_URI").unwrap_or_default()
            }) {
                println!("\nYour {} authenticate!", ansi_term::Colour::Green.bold().paint("successfully"));
            }else {
                println!("\nAn {}, please try again or check your {}!", 
                    ansi_term::Colour::Red.bold().paint("error occured"),
                    ansi_term::Colour::Green.bold().paint("internet connection")
                );
            }
        }
    }
    Server::kill(1000);
    HttpResponse::Ok().body("Process ended, you can close this browser!")
}

pub mod server;
