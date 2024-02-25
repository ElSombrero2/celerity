use ansi_term::Colour::{Blue, Green};
use figlet_rs::FIGfont;
use indoc::indoc;

pub fn lib_description() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Celerity.io");
    println!("{}", Blue.bold().paint(figure.unwrap().to_string()));

    let text = format!( 
        indoc!{"
            Welcome to {} ✨,
            a simple tool that help you to
            init your project {}.\n
            See the list of all avalaible commands
            in celerity --help."
        }, 
    Blue.bold().paint("Celerity.io"), Green.bold().underline().paint("easily"));
    println!("{}", text)
}