mod argparse;

use argparse::parse_args;
use argparse::Commands::{App, Apply, Diff, Load, Save};

fn main() {
    let args = parse_args();
    match &args.command {
        Load {
            name,
            hub,
            tag,
            ssh,
            apply,
        } => {}

        App { name, add, delete } => {}

        Diff { app } => {}

        Apply { app } => {}

        Save { app, tag } => {}
    }
}
