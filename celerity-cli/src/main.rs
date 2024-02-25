use clap::Parser;


mod commands;

#[tokio::main]
async fn main(){
    let cmd = commands::Commands::parse();

    match cmd {
        commands::Commands {template_list: true, ..} => {

        },
        _ => {
            println!("Hello world")
        }
    }
}