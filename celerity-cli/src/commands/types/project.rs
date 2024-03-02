use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum ProjectCommand{
    Init {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        path: String,
        #[arg(short, long)]
        template: String,
    },
    AddTodo {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        row: String,
        #[arg(short, long)]
        title: String,
    },
    AddRow {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        row: String,
    },
    RemoveRow {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        row: String,
    },
    RemoveTask {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        row: String,
        #[arg(short, long)]
        task_id: String,
    },
    MoveTask {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        task_id: String,
        #[arg(long)]
        origin_row: String,
        #[arg(long)]
        target_row: String,
    },
    Open {
        project: String,
    }
}