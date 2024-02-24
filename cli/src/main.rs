use core::{config::configuration::Configuration, utils::json::read_json};

fn main(){
    if let Some(config) = read_json::<Configuration>("examples/config/basic-config.json".to_owned()) {
        dbg!("{}", config);
    }
}