use std::{fs::{self, File}, io::Read, path::Path};
use serde::de::DeserializeOwned;

pub struct Yaml;

impl Yaml {
    pub fn read<T>(path: String) -> Option<T> where T: DeserializeOwned {
        if let Ok(mut file) = File::open(path) {
            let mut buff = String::new();
            let copy_buffer_result = file.read_to_string(&mut buff);
            if copy_buffer_result.is_ok() {
                if let Ok(result) = serde_yaml::from_str::<T>(buff.as_str()){
                    return Some(result);
                }
            }
            return Option::None;
        }
        Option::None
    }

    pub fn save(content: String, path: String) -> bool {
        let mut strs = path.split('/').collect::<Vec<&str>>();
        strs.remove(strs.len() - 1);
        let binding = strs.join("/");
        let folder_path = binding.as_str();
        if !Path::new(folder_path).exists() {
            fs::create_dir(folder_path).unwrap_or_default();   
        }
        fs::write(path, content).is_ok()
    }   
}
