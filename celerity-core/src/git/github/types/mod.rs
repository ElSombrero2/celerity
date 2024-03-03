use serde::Serialize;
use crate::{auth::OAuth2Response, config::types::{Configuration, User}};
use super::url::ApiUrl;

#[derive(Serialize, Debug)]
pub struct GithubClient{
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub code: String,
}

pub struct Github;

impl Github {
    pub fn authenticate(github_client: GithubClient) -> bool{
        let res = reqwest::blocking::Client::new()
        .post(&ApiUrl::github("/login/oauth/access_token"))
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
    
    pub fn me(access_token: String) -> Option<User>{
        let res = reqwest::blocking::Client::new()
        .get(&ApiUrl::github_api("/user"))
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
        .get(&ApiUrl::github_api("/octocat"))
        .header("User-Agent", "Celerity.io/0.1.0")
        .header("Authorization", "Bearer ".to_owned() + access_token.as_str())
        .send();
        if let Ok(res) = res {
            return res.status() == 200;
        }
        false
    }
}