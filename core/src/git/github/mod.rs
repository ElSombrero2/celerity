use std::{thread, time::Duration};

use crate::{config::types::{Configuration, User}, git::github::types::{GithubClient, OAuth2Response}, utils::url::{github_api_url, github_url}};

pub mod server;
pub mod types;

fn kill(millis: u64){
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(millis));
        std::process::exit(0);
    });
}

async fn open_browser(oauth2_url: String){
    open::that_detached(oauth2_url).unwrap()
}

fn authenticate(github_client: GithubClient) -> bool{
    println!("Start authentication");
    let res = reqwest::blocking::Client::new()
    .post(&github_url("/login/oauth/access_token"))
    .header("Accept", "application/json")
    .form(&[
        ("client_id", github_client.client_id),
        ("client_secret", github_client.client_secret),
        ("code", github_client.code),
        ("redirect_uri", github_client.redirect_uri)
    ])
    .send();
    match res {
        Ok(res) => {
            if let Ok(oauth2_response) = res.json::<OAuth2Response>() {
                return Configuration::register(oauth2_response);
            }
        },
        Err(err) => println!("{}", err)
    }
    false
}

pub fn find_me(access_token: String) -> Option<User>{
    let res = reqwest::blocking::Client::new()
    .get(&github_api_url("/user"))
    .header("User-Agent", "Celerity.io/0.1.0")
    .header("Authorization", "Bearer ".to_owned() + access_token.as_str())
    .send();
    match res {
        Ok(res) => {
            if let Ok(user) = res.json::<User>() {
                return Some(user);
            }
        },
        Err(err) => { dbg!("{}", err); }
    }
    Option::None
}

pub async fn create_repository(_name: String) -> bool {
    false
}