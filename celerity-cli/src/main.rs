use clap::Parser;
use comfy_table::Table;
use commands::{projects::init_project, templates::show_template_list};
use texts::lib_description;

mod commands;
mod texts;

#[tokio::main]
async fn main(){
    let cmd = commands::Commands::parse();
    let mut table = Table::new();

    match cmd {
        commands::Commands {templates: true, ..} => show_template_list(&mut table),
        commands::Commands {init: Some(init), ..} => init_project(init),
        _ => lib_description()
    }
}