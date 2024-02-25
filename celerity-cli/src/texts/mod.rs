use ansi_term::Colour::{Blue, Red};

pub fn lib_description() -> String {
    format!("Welcome to {},\na simple tool that help you to\ninit your project {}.\n\nSee the list of all avalaible commands\nin celerity --help.", 
    Blue.underline().bold().paint("Celerity.io"), Red.bold().underline().paint("easily"))
}