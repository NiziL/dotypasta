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
            let url = format!(
                "{protocol}{host}{sep}{name}/dotfiles.git",
                protocol = if *ssh { "git@" } else { "https://" },
                host = match hub {
                    Github => "github.com",
                    Gitlab => "gitlab.com",
                    Bitbucket => "bitbucket.org",
                },
                sep = if *ssh { ":" } else { "/" },
                name = name
            );

            // clone repo
            match (ssh, tag) {
                (false, None) => dotypasta::repository::clone_https(url.as_str()),
                (true, None) => dotypasta::repository::clone_ssh(url.as_str()),
                (false, Some(refname)) => {
                    dotypasta::repository::clone_https_with_ref(url.as_str(), refname)
                }
                (true, Some(refname)) => {
                    dotypasta::repository::clone_ssh_with_ref(url.as_str(), refname)
                }
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

        Clear {} => dotypasta::repository::clear(),
    }
}
