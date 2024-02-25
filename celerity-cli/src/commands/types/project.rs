use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Project{
    Init {
        name: String,
        path: String,
        #[arg(short, long)]
        template: String,
    }
}