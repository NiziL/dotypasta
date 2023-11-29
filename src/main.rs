mod argparse;
use argparse::Commands::{Apply, Clear, Config, Diff, Load, Save};
mod dotypasta;

fn main() {
    let args = argparse::parse_args();
    match &args.command {
        Load {
            name,
            hub,
            tag,
            ssh,
            apply,
        } => {
            // construct repo url
            let url = if *ssh {
                format!("git@{}:{}/dotfiles.git", hub, name)
            } else {
                format!("https://{}/{}/dotfiles.git", hub, name)
            };

            // clone repo
            match (ssh, tag) {
                (false, None) => dotypasta::repo::from_https(&url),
                (false, Some(refname)) => dotypasta::repo::from_https_with_ref(&url, refname),
                (true, None) => dotypasta::repo::from_ssh(&url),
                (true, Some(refname)) => dotypasta::repo::from_ssh_with_ref(&url, refname),
            }

            // apply configuration
            if *apply {
                dotypasta::files::dotypasta_to_host(dotypasta::config::open_config())
            }
        }

        Config { name, add, delete } => {
            // TODO 5 file access here (3r, 2w) => expose more fn in config to reduce it
            if let Some(strings) = add {
                dotypasta::config::add(name.to_string(), strings.to_vec());
            }
            if let Some(strings) = delete {
                dotypasta::config::rm(name.to_string(), strings.to_vec());
            }

            if add.is_none() && delete.is_none() {
                if let Some(filenames) = dotypasta::config::get(name) {
                    // TODO cleaner print
                    println!("{:?}", filenames);
                } else {
                    println!("Application {} is not defined", name);
                }
            }
        }

        Diff { app } => {}

        Apply { app } => {
            let mut config = dotypasta::config::open_config();
            if let Some(appnames) = app {
                config.retain(|k, v| appnames.contains(k));
            }
            dotypasta::files::dotypasta_to_host(config);
        }

        Save { app } => {
            let mut config = dotypasta::config::open_config();
            if let Some(appnames) = app {
                config.retain(|k, v| appnames.contains(k));
            }
            dotypasta::files::host_to_dotypasta(config);
        }

        Clear {} => dotypasta::repo::clear(),
    }
}
