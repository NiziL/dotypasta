use std::collections::HashMap;
use std::env;
use std::fs;
use toml;

fn open_config() -> HashMap<String, Vec<String>> {
    let rcpath: String = env::var("HOME").unwrap() + "/.dotypastarc";
    let contents = match fs::read_to_string(rcpath) {
        Ok(contents) => contents,
        Err(_) => "".to_string(),
    };
    let data: HashMap<String, Vec<String>> = toml::from_str(&contents).unwrap();
    return data;
}

fn write_config(config: HashMap<String, Vec<String>>) {
    let rcpath: String = env::var("HOME").unwrap() + "/.dotypastarc";
    let data = toml::to_string(&config).unwrap();
    fs::write(rcpath, data).expect("Could not write ~/.dotypastarc");
}

pub fn read(appname: &String) -> Option<Vec<String>> {
    let data = open_config();
    data.get(appname).cloned()
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
