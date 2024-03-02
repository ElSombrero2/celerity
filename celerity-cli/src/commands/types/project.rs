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
    },
    AddRow {
        #[arg(long)]
        id: String,
        #[arg(long)]
        row: String,
    },
    RemoveRow {
        #[arg(long)]
        id: String,
        #[arg(long)]
        row: String,
    },
    RemoveTask {
        #[arg(long)]
        project_id: String,
        #[arg(long)]
        row: String,
        #[arg(long)]
        task_id: String,
    },
    MoveTask {
        #[arg(long)]
        project_id: String,
        #[arg(long)]
        task_id: String,
        #[arg(long)]
        origin_row: String,
        #[arg(long)]
        target_row: String,
    },
    Open {
        id: String,
    }
}