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
                (false, None) => dotypasta::load::from_https(&url),
                (true, None) => dotypasta::load::from_ssh(&url),
                (false, Some(refname)) => dotypasta::load::from_https_with_ref(&url, refname),
                (true, Some(refname)) => dotypasta::load::from_ssh_with_ref(&url, refname),
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

        Clear {} => dotypasta::load::clear(),
    }
}
