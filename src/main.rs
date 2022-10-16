mod cli;
mod commands;
mod config;
mod utils;

use cli::{Cli, Command};
use commands::{add, delete, home, list, r#use, rename, show};

fn main() {
    use clap::Parser;
    let cli = Cli::parse();
    match cli.command {
        Command::List => {
            list();
        }
        Command::Show { name } => {
            show(name);
        }
        Command::Add {
            name,
            url: registry,
            home,
        } => {
            add(name, registry, home);
        }
        Command::Rename { name, new_name } => {
            rename(name, new_name);
        }
        Command::Delete { name } => {
            delete(name);
        }
        Command::Use { name } => {
            r#use(name).unwrap();
        }
        Command::Home { name, browser } => {
            home(name, browser);
        }
    }
}
