mod cli;
mod commands;
mod config;
mod utils;

use cli::{Cli, Command};
use commands::{add, current, list};

fn main() {
    use clap::Parser;
    let cli = Cli::parse();
    match cli.command {
        Command::List => {
            list();
        }
        Command::Current => {
            current();
        }
        Command::Add {
            name,
            url: registry,
            home,
        } => {
            add(name, registry, home);
        }
    }
}
