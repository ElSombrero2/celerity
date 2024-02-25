use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Celerity is a simple tool", long_about = None)]
pub struct Commands {
    #[arg(short = 'n', long)]
    pub name: Option<String>,
    // See all templates available
    #[arg(short = 't', long = "template-list")]
    pub template_list: bool
}