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
        + &format!("client_id={}", env::var("CLIENT_ID").unwrap_or_default())
        + &format!("&redirect_uri={}", env::var("REDIRECT_URI").unwrap_or_default())
        + &format!("&state={}", env::var("STATE").unwrap_or_default())
        + &format!("&{}", env::var("EXTRA").unwrap_or_default())
    }
}
