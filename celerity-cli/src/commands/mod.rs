use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Celerity is a simple tool", long_about = None)]
pub struct Commands {
    #[arg(short = 't', long = "template-list", help = "See all templates available")]
    pub template_list: bool
}