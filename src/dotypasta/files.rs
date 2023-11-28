use std::collections::HashMap;
use std::env;
use std::fs;

pub fn dotypasta_to_host(config: HashMap<String, Vec<String>>) {
    config
        .iter()
        .flat_map(|(appname, filenames)| {
            println!("Applying {}", appname);
            filenames.iter()
        })
        .for_each(|filename| {
            let home = env::var("HOME").unwrap() + "/";
            let dotypasta = home.clone() + crate::dotypasta::REPO_PATH + "/";
            println!("copy {}{} to {}{}", home, filename, dotypasta, filename);
            let _ = fs::copy(dotypasta + filename, home + filename);
        });
}

pub fn host_to_dotypasta(config: HashMap<String, Vec<String>>) {
    config
        .iter()
        .flat_map(|(appname, filenames)| {
            println!("Saving {}", appname);
            filenames.iter()
        })
        .for_each(|filename| {
            let home = env::var("HOME").unwrap() + "/";
            let dotypasta = home.clone() + crate::dotypasta::REPO_PATH + "/";
            println!("copy {}{} to {}{}", home, filename, dotypasta, filename);
            let _ = fs::copy(home + filename, dotypasta + filename);
        });
}
