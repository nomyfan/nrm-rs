use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    List,
    Current,
    Add {
        name: String,
        url: String,
        home: Option<String>,
    },
    Rename {
        name: String,
        new_name: String,
    },
    Delete {
        name: String,
    },
    Use {
        name: String,
    },
    Home {
        name: String,
        browser: Option<String>,
    },
}
