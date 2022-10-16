use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    List,
    Show {
        name: Option<String>,
    },
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
        name: Option<String>,
        browser: Option<String>,
    },
}
