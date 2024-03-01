use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum ProjectCommand{
    Init {
        name: String,
        path: String,
        #[arg(short, long)]
        template: String,
    },
    AddTodo {
        #[arg(long)]
        id: String,
        #[arg(long)]
        row: String,
        #[arg(long)]
        title: String,
    }
}