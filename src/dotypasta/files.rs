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
            let dotypasta = format!(
                "{}/{}/{}",
                env::var("HOME").unwrap(),
                crate::dotypasta::REPO_PATH,
                filename
            );
            let home = format!("{}/{}", env::var("HOME").unwrap(), filename);
            let src_path = Path::new(&dotypasta);
            let dst_path = Path::new(&home);
            if let Some(dir) = dst_path.parent() {
                if let Ok(false) = dir.try_exists() {
                    let _ = fs::create_dir_all(dir);
                }
            }
            // TODO better UI
            if let Ok(_) = fs::copy(src_path, dst_path) {
                println!("copy {:?} to {:?}", src_path, dst_path);
            } else {
                println!("error copying {:?} to {:?}", src_path, dst_path);
            }
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
            let dotypasta = format!(
                "{}/{}/{}",
                env::var("HOME").unwrap(),
                crate::dotypasta::REPO_PATH,
                filename
            );
            let home = format!("{}/{}", env::var("HOME").unwrap(), filename);
            let src_path = Path::new(&home);
            let dst_path = Path::new(&dotypasta);
            if let Some(dir) = dst_path.parent() {
                if let Ok(false) = dir.try_exists() {
                    let _ = fs::create_dir_all(dir);
                }
            }
            // TODO better UI
            if let Ok(_) = fs::copy(src_path, dst_path) {
                println!("copy {:?} to {:?}", src_path, dst_path);
            } else {
                println!("error copying {:?} to {:?}", src_path, dst_path);
            }
        });
}
