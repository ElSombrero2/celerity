use std::{thread, time::Duration};
use spinoff::{spinners, Color, Spinner};
use ansi_term::Colour::{Blue, Red};

use crate::auth::server::Server;


pub struct Browser;

impl Browser {
    pub async fn open(oauth2_url: String){
        let mut sp = Spinner::new(spinners::Dots9, format!("Start authentication {}", Blue.bold().paint("(Timout: 15s)")), Color::White);
        open::that(oauth2_url).unwrap();
        thread::sleep(Duration::from_millis(15000));
        sp.stop_with_message(&format!("\n❌ Authentication {}!", Red.bold().paint("timeout")));
        Server::kill(0);
    }
}