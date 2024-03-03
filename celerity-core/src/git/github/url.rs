use std::env;

pub struct ApiUrl;

impl ApiUrl {
    pub fn github(path: &str) -> String {
        env::var("GITHUB_BASE_URL").unwrap_or_default().to_owned() + path
    }
    
    pub fn github_api(path: &str) -> String{
        env::var("GITHUB_API_BASE_URL").unwrap_or_default().to_owned() + path
    }
    
    pub fn authorization() -> String {
        ApiUrl::github("/login/oauth/authorize?")
        + &format!("client_id={}", env::var("GITHUB_CLIENT_ID").unwrap_or_default())
        + &format!("&redirect_uri={}", env::var("GITHUB_REDIRECT_URI").unwrap_or_default())
        + &format!("&state={}", env::var("GITHUB_STATE").unwrap_or_default())
        + &format!("&{}", env::var("GITHUB_EXTRA").unwrap_or_default())
    }
}
