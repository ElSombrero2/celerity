use clap::Parser;
use self::types::project::Project;

pub mod types;
pub mod templates;
pub mod projects;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Celerity is a simple tool to init your project based on templates", long_about = None)]
pub struct Commands {
    #[arg(short, long, help = "See all templates available")]
    pub templates: bool,
    #[command(subcommand, help = "Initialize your project" )]
    pub init: Option<Project>
}