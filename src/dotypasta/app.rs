use std::collections::HashMap;
use std::env;
use std::fs;
use toml;

pub fn read(appname: &String) -> Option<Vec<String>> {
    let filename = env::var("HOME").unwrap() + "/.local/share/dotypasta/dotypasta.toml";
    let contents = fs::read_to_string(filename)
        .expect("Could not read ~/.local/share/dotypasta/dotypasta.toml");
    let data: HashMap<String, Vec<String>> = toml::from_str(&contents).unwrap();

    if data.contains_key(appname) {
        Some(data[appname].clone())
    } else {
        None
    }
}

pub fn add(appname: &String, filenames: &Vec<String>) {}

pub fn rm(appname: &String, filenames: &Vec<String>) {}
