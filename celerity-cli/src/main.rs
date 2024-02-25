use core::projects::template::list;
use clap::Parser;
use comfy_table::Table;

mod commands;
mod texts;

#[tokio::main]
async fn main(){
    let cmd = commands::Commands::parse();
    let mut table = Table::new();

    match cmd {
        commands::Commands {template_list: true, ..} => {
            let templates = list(String::from("examples/templates"));
            let template_length = templates.len();
            table.set_header(vec!["Name", "Author"]);
            for template in templates {
                if let Some(template) = template {
                    table.add_row(vec![
                        template.name,
                        template.author
                    ]);
                }
            }
            println!("Templates list ({})\n{}", template_length, table)
        },
        _ => {
            println!("{}", texts::lib_description());
        }
    }
}