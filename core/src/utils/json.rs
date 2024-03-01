use std::{fs::{self, File}, io::Read, path::Path};
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

    pub fn save(content: String, path: String, filename: String) -> bool {
        if !Path::new(&path).exists() {
            fs::create_dir(&path).unwrap_or_default();   
        }
        println!("{path}/{filename}");
        fs::write(format!("{path}/{filename}"), content).is_ok()
    }   
}
