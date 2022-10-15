mod cli;
mod commands;
mod config;
mod utils;

use cli::{Cli, Command};
use commands::list;

fn main() {
    use clap::Parser;
    let cli = Cli::parse();
    match cli.command {
        Command::List => {
            list();
        }
    }
}
