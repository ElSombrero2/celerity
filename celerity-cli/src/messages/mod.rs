use ansi_term::Colour::{Blue, Green, Yellow};
use figlet_rs::FIGfont;
use indoc::indoc;

pub struct Messages;

impl Messages {
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
    
    pub fn config_error(){
        println!(indoc! {"
            \n{} It seems like Configuration file is not found!
            Please try to login with github using 
            the command {} or short command {} 
            to initialize your configuration file
        "}, 
            Yellow.bold().paint("[WARN]"),
            Blue.bold().paint("--github-login"),
            Blue.bold().paint("-g")
        );
    }
    
    pub fn expiration_message(){
        print!(indoc! {
            "Your {} has expired,
            {} your session...
        "},
            Yellow.bold().paint("session"),
            Green.bold().paint("Refeshing"),
        );
    }   
}