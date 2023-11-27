mod argparse;
use argparse::Commands::{App, Apply, Clear, Diff, Load, Save};
use argparse::GitHosts::{Bitbucket, Github, Gitlab};
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
            let url = format!("{}", argparse::GitURL::new(*ssh, hub.clone(), name.clone()));

            // clone repo
            match (ssh, tag) {
                (false, None) => dotypasta::clone::from_https(&url),
                (true, None) => dotypasta::clone::from_ssh(&url),
                (false, Some(refname)) => dotypasta::clone::from_https_with_ref(&url, refname),
                (true, Some(refname)) => dotypasta::clone::from_ssh_with_ref(&url, refname),
            }

            // apply configuration
            if *apply {
                // TODO dotypasta::apply();
            }
        }

        App { name, add, delete } => {}

        Diff { app } => {}

        Apply { app } => {}

        Save { app, tag } => {}

        Clear {} => dotypasta::clone::clear(),
    }
}
