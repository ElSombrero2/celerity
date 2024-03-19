use dotenv_codegen::dotenv;

pub struct ApiUrl;

impl ApiUrl {
    pub fn github(path: &str) -> String {
        dotenv!("GITHUB_BASE_URL").to_owned() + path
    }
    
    pub fn github_api(path: &str) -> String{
        dotenv!("GITHUB_API_BASE_URL").to_owned() + path
    }
    
    pub fn authorization() -> String {
        ApiUrl::github("/login/oauth/authorize?")
        + &format!("client_id={}", dotenv!("GITHUB_CLIENT_ID"))
        + &format!("&redirect_uri={}", dotenv!("GITHUB_REDIRECT_URI"))
        + &format!("&state={}", dotenv!("GITHUB_STATE"))
        + &format!("&{}", dotenv!("GITHUB_EXTRA"))
    }
}
