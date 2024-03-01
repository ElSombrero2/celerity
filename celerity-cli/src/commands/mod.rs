use clap::Parser;
use self::types::project::ProjectCommand;

pub mod types;
pub mod actions;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Celerity is a simple tool to init your project based on templates", long_about = None)]
pub struct Commands {
    #[command(subcommand, help = "Initialize your project" )]
    pub subcommand: Option<ProjectCommand>,
    
    #[arg(long, help = "Show all Todos")]
    pub todo_list: Option<String>,

    #[arg(short, long, help = "See all templates available")]
    pub templates: bool,
    
    #[arg(short = 'g', long = "github-login", help = "Login with Github")]
    pub github_login: bool,

    #[arg(short, long, help = "Show all projects")]
    pub projects: bool,
    #[arg(long, help = "Find one project")]
    pub project: Option<String>,
    
    #[arg(long, help = "Show all my Github account information")]
    pub me: bool,
}