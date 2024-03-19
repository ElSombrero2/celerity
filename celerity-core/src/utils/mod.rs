use std::env::current_exe;

pub mod json;
pub mod browser;
pub mod printer;
pub mod yaml;

pub fn __dirname(path: &str) -> String{
    let mut dir = current_exe().unwrap();
    if !dir.to_string_lossy().to_lowercase().contains("debug") {
        dir.pop();
        return dir.to_string_lossy().to_string() + path;
    }
    String::from('.') + path
}