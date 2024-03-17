use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum ProjectCommand{
    /// Initialize your project
    Init {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        path: String,
        #[arg(short, long)]
        template: String,
    },
    /// Add a new task to your Kanban Board
    AddTodo {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        row: String,
        #[arg(short, long)]
        title: String,
    },
    /// Add a new row to your Kanban Board
    AddRow {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        row: String,
    },
    /// Remove an existing Row to your Kanban Board
    RemoveRow {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        row: String,
    },
    /// Remove an existing Task to your Kanban Board
    RemoveTask {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        row: String,
        #[arg(short, long)]
        task_id: String,
    },
    /// Move a task from an origin row to a target row
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
    /// Open your project with Visual Studio Code (Need the code command in your environment variable Path)
    Open {
        project: String,
    },
    /// Show all the docker services of your project (docker-compose is required)
    Services{
        #[arg(short, long)]
        project: String,
    },
    /// execute a docker-compose command to your project (docker-compose is required)
    Cmd{
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        command: String,
    }
}