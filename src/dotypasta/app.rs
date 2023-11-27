use std::collections::HashMap;
use std::env;
use std::fs;
use toml;

fn open_config() -> HashMap<String, Vec<String>> {
    let fpath: String = env::var("HOME").unwrap() + "/.local/share/dotypasta/dotypasta.toml";
    let contents =
        fs::read_to_string(fpath).expect("Could not read ~/.local/share/dotypasta/dotypasta.toml");
    let data: HashMap<String, Vec<String>> = toml::from_str(&contents).unwrap();
    return data;
}

fn write_config(config: HashMap<String, Vec<String>>) {
    let fpath: String = env::var("HOME").unwrap() + "/.local/share/dotypasta/dotypasta.toml";
    let data = toml::to_string(&config).unwrap();
    fs::write(fpath, data).expect("Could not write ~/.local/share/dotypasta/dotypasta.toml");
}

pub fn read(appname: &String) -> Option<Vec<String>> {
    let data = open_config();
    if data.contains_key(appname) {
        Some(data[appname].clone())
    } else {
        None
    }
}

pub fn add(appname: String, filenames: Vec<String>) {
    let mut data = open_config();
    if data.contains_key(&appname) {
        data.entry(appname).and_modify(|fpaths| {
            for filename in filenames {
                if !fpaths.contains(&filename) {
                    fpaths.push(filename)
                }
            }
        });
    } else {
        data.insert(appname, filenames);
    }
    write_config(data)
}

pub fn rm(appname: String, filenames: Vec<String>) {
    let mut data = open_config();
    if data.contains_key(&appname) {
        data.entry(appname)
            .and_modify(|e| e.retain(|e| !filenames.contains(e)));
        write_config(data);
    }
}
