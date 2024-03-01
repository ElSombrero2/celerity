use serde::{Deserialize, Serialize};

use crate::projects::types::Projects;

#[derive(Deserialize, Serialize, Debug)]
pub struct Configuration{
    pub github_token: Option<String>,
    pub user: Option<User>,
    pub projects: Vec<Projects>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub login: Option<String>,
    pub id: Option<u32>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub public_repos: Option<u32>,
    pub public_gists: Option<u32>,
    pub followers: Option<u32>,
    pub following: Option<u32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub private_gists: Option<u32>,
    pub total_private_repos: Option<u32>,
    pub owned_private_repos: Option<u32>,
  }
