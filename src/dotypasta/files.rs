use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub fn dotypasta_to_host(config: HashMap<String, Vec<String>>) {
    config
        .iter()
        .flat_map(|(appname, filenames)| {
            // TODO better UI
            println!("Applying {}", appname);
            filenames.iter()
        })
        .for_each(|filename| {
            let dotypasta =
                env::var("HOME").unwrap() + "/" + crate::dotypasta::REPO_PATH + "/" + filename;
            let home = env::var("HOME").unwrap() + "/" + filename;
            let src_path = Path::new(&dotypasta);
            let dst_path = Path::new(&home);
            // TODO better UI
            println!("copy {:?} to {:?}", src_path, dst_path);
            if let Some(dir) = dst_path.parent() {
                if let Ok(false) = dir.try_exists() {
                    let _ = fs::create_dir_all(dir);
                }
            }
            let _ = fs::copy(src_path, dst_path);
        });
}

pub fn host_to_dotypasta(config: HashMap<String, Vec<String>>) {
    config
        .iter()
        .flat_map(|(appname, filenames)| {
            // TODO better UI
            println!("Saving {}", appname);
            filenames.iter()
        })
        .for_each(|filename| {
            let dotypasta =
                env::var("HOME").unwrap() + "/" + crate::dotypasta::REPO_PATH + "/" + filename;
            let home = env::var("HOME").unwrap() + "/" + filename;
            let src_path = Path::new(&home);
            let dst_path = Path::new(&dotypasta);
            // TODO better UI
            println!("copy {:?} to {:?}", src_path, dst_path);
            if let Some(dir) = dst_path.parent() {
                if let Ok(false) = dir.try_exists() {
                    let _ = fs::create_dir_all(dir);
                }
            }
            let _ = fs::copy(src_path, dst_path);
        });
}
