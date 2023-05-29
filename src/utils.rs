use std::ffi::OsStr;
use std::fs::{read_dir, read_to_string, write};
use std::io::Error;
use std::env::current_dir;
use std::vec::Vec;


const FILE_EXTENSION: &str = "grconf";


pub fn save_to_path(name: &String, data: &String) -> Result<(), Error> {
    let mut path = current_dir()?;
    let filename: String = format!("{}.{}", name, FILE_EXTENSION);
    path.push(filename);
    write(path, data)
}

pub fn read_from_path(name: &String) -> Result<String, Error> {
    let mut path = current_dir()?;
    let filename: String = format!("{}.{}", name, FILE_EXTENSION);
    path.push(filename);
    read_to_string(path)
}

pub fn get_config_names() -> Result<Vec<String>, Error> {
    let path = current_dir()?;
    let mut names = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.extension().and_then(OsStr::to_str) == Some(FILE_EXTENSION) {
            names.push(
                entry_path
                    .file_stem().unwrap()
                    .to_str().unwrap()
                    .to_string()
            );
        }
    }
    Ok(names)
}