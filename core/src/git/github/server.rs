use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use crate::{git::github::{authenticate, kill, open_browser, types::{GithubClient, OAuth2Payload}}, utils::url::get_authorization_uri};

#[get("/login")]
async fn login(payload: web::Query<OAuth2Payload>) -> impl Responder {
    if let (Some(code), Some(state)) = (&payload.code, &payload.state) {
        if state.eq("celerity.io") {
            if authenticate(GithubClient {
                code: code.to_string(),
                client_id: env::var("CLIENT_ID").unwrap_or_default(),
                client_secret: env::var("CLIENT_SECRET").unwrap_or_default(),
                redirect_uri: env::var("REDIRECT_URI").unwrap_or_default()
            }) {
                println!("Your {} authenticate!", ansi_term::Colour::Green.bold().paint("successfully"));
            }else {
                println!("An {}, please try again or check your {}!", 
                    ansi_term::Colour::Red.bold().paint("error occured"),
                    ansi_term::Colour::Green.bold().paint("internet connection")
                );
            }
        }
    }
    kill(1000);
    HttpResponse::Ok().body("Process ended, you can close this browser!")
}

pub async fn start_server(){
    let server = HttpServer::new(|| {
        App::new()
            .service(login)
    });
    let run = server.shutdown_timeout(1).bind("127.0.0.1:8100").unwrap().run();
    let _ = tokio::join!(run, open_browser(get_authorization_uri()));
}
