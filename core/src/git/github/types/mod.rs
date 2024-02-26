use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Debug)]
pub struct GithubClient{
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub code: String,
}