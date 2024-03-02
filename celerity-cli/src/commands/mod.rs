use clap::Parser;
use self::types::project::ProjectCommand;

pub mod types;
pub mod controllers;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Celerity is a simple tool to init your project based on templates", long_about = None)]
pub struct Commands {
    #[command(subcommand, help = "Subcommand help")]
    pub subcommand: Option<ProjectCommand>,
    
    #[arg(long, help = "Show all Todos")]
    pub kanban: Option<String>,
    #[arg(short, long, help = "See all templates available")]
    pub templates: bool,
    #[arg(long, help = "Login with Github")]
    pub login: bool,
    #[arg(long, help = "Show my projects")]
    pub projects: bool,
    #[arg(long, help = "Find my project")]
    pub project: Option<String>,
    #[arg(long, help = "Show my Github account information")]
    pub me: bool,
}