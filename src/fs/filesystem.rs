use std::env::current_dir;
use std::fs;

pub fn load_as_binary(path: String) -> Vec<u8> {
    let real_path = current_dir()
        .unwrap()
        .join("res")
        .join(path);

    fs::read(real_path).unwrap()
}

pub fn load_as_file(path: String) -> String {
    current_dir()
        .unwrap()
        .join("res")
        .join(path)
        .into_os_string()
        .into_string()
        .unwrap()
}