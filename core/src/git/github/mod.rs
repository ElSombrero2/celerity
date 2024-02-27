use std::{thread, time::Duration};
use ansi_term::Color::{Red, Blue};
use spinoff::{spinners, Color, Spinner};
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
    let mut sp = Spinner::new(spinners::Dots9, format!("Start authentication {}", Blue.bold().paint("(Timout: 15s)")), Color::White);
    open::that(oauth2_url).unwrap();
    thread::sleep(Duration::from_millis(15000));
    sp.stop_with_message(&format!("\n❌ Authentication {}!", Red.bold().paint("timeout")));
    kill(0);
}

fn authenticate(github_client: GithubClient) -> bool{
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
    if let Ok(res) = res {
        if let Ok(oauth2_response) = res.json::<OAuth2Response>() {
            return Configuration::register(oauth2_response);
        }
    }
    false
}

pub fn find_me(access_token: String) -> Option<User>{
    let res = reqwest::blocking::Client::new()
    .get(&github_api_url("/user"))
    .header("User-Agent", "Celerity.io/0.1.0")
    .header("Authorization", "Bearer ".to_owned() + access_token.as_str())
    .send();
    if let Ok(res) = res {
        if let Ok(user) = res.json::<User>() {
            return Some(user);
        }
    }
    Option::None
}

pub async fn create_repository(_name: String) -> bool {
    false
}

pub fn ping(access_token: String) -> bool {
    let res = reqwest::blocking::Client::new()
    .get(&github_api_url("/octocat"))
    .header("User-Agent", "Celerity.io/0.1.0")
    .header("Authorization", "Bearer ".to_owned() + access_token.as_str())
    .send();
    if let Ok(res) = res {
        return res.status() == 200;
    }
    false
}