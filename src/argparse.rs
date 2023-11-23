use clap::{Parser, Subcommand, ValueEnum};

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
        #[arg(long, default_value_t=GitHosts::Github)]
        hub: GitHosts,
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
    /// Configure applications for dotopy
    App {
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
        app: Option<String>,
    },
    /// Apply pulled dotfiles
    Apply {
        /// for this application (default: all)
        #[arg(long)]
        app: Option<String>,
    },
    /// Save dotfiles from your computer to the local repository
    Save {
        /// for this application (default: all)
        #[arg(long)]
        app: Option<String>,
        /// git tag to add on this save
        #[arg(short, long)]
        tag: Option<String>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum GitHosts {
    Github,
    Gitlab,
    Bitbucket,
}
