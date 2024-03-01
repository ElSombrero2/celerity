use std::{fs::File, io::Read};
use serde::de::DeserializeOwned;

pub struct Json;

impl Json {
    pub fn read<T>(path: String) -> Option<T> where T: DeserializeOwned {
        if let Ok(mut file) = File::open(path) {
            let mut buff = String::new();
            let copy_buffer_result = file.read_to_string(&mut buff);
            if copy_buffer_result.is_ok() {
                if let Ok(result) = serde_json::from_str::<T>(buff.as_str()){
                    return Some(result);
                }
            }
            return Option::None;
        }
        Option::None
    }   
}
