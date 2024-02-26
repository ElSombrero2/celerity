use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Configuration{
    pub github_token: Option<String>,
    pub user: Option<User>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    login: Option<String>,
    id: Option<u32>,
    avatar_url: Option<String>,
    gravatar_id: Option<String>,
    url: Option<String>,
    name: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    public_repos: Option<u32>,
    public_gists: Option<u32>,
    followers: Option<u32>,
    following: Option<u32>,
    created_at: Option<String>,
    updated_at: Option<String>,
    private_gists: Option<u32>,
    total_private_repos: Option<u32>,
    owned_private_repos: Option<u32>,
  }
