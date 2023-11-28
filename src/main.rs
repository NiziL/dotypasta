mod argparse;
use argparse::Commands::{App, Apply, Clear, Diff, Load, Save};
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
                // TODO dotypasta::apply();
            }
        }

        App { name, add, delete } => {
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
                    println!("Application {} is not defined", name)
                }
            }
        }

        Diff { app } => {}

        Apply { app } => {}

        Save { app } => {}

        Clear {} => dotypasta::repo::clear(),
    }
}
