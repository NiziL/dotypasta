use clap::{Parser, Subcommand, ValueEnum};
use std::fmt;

pub fn parse_args() -> Args {
    Args::parse()
}

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Load a dotfiles repository
    Load {
        /// username
        #[arg(help = "username")]
        name: String,
        /// host from which pull
        #[clap(value_enum)]
        #[arg(long, default_value_t=GitHost::Github)]
        hub: GitHost,
        /// git tag to pull
        #[arg(short, long)]
        tag: Option<String>,
        /// enable usage of ssh
        #[arg(long, action)]
        ssh: bool,
        /// directly apply loaded configuration
        #[arg(long, action)]
        apply: bool,
    },
    /// Configure applications for dotypasta
    Config {
        /// application name to configure
        name: String,
        /// files to add to this application
        #[arg(short, long)]
        add: Option<Vec<String>>,
        /// files to remove from this application
        #[arg(short, long)]
        delete: Option<Vec<String>>,
    },
    /// Show the differences between the loaded dotfiles and yours
    Diff {
        /// for this application (default: all)
        #[arg(long)]
        app: Option<Vec<String>>,
    },
    /// Apply pulled dotfiles
    Apply {
        /// for this application (default: all)
        #[arg(long)]
        app: Option<Vec<String>>,
    },
    /// Save dotfiles from your computer to the local repository
    Save {
        /// for this application (default: all)
        #[arg(long)]
        app: Option<Vec<String>>,
    },
    /// Delete the local repository
    Clear {},
}

#[derive(ValueEnum, Clone, Debug)]
pub enum GitHost {
    Github,
    Gitlab,
    Bitbucket,
}

impl fmt::Display for GitHost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GitHost::Github => write!(f, "github.com"),
            GitHost::Gitlab => write!(f, "gitlab.com"),
            GitHost::Bitbucket => write!(f, "bitbucket.org"),
        }
    }
}
