use spinoff::{spinners, Color, Spinner};

pub struct SpinnerPrinter{ spinner: Spinner }

impl SpinnerPrinter {
    pub fn new_dot8(message: String) -> Self {
        SpinnerPrinter { spinner: Spinner::new(spinners::Dots8, message, Color::White) }
    }

    pub fn print(&mut self, message: String){
        println!();
        self.spinner.update_text(message);
    }

    pub fn stop(mut self, message: String){
        self.spinner.stop_with_message(&message);
    }
}
