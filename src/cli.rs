use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "nrm", version)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    #[clap(about = "List all registries")]
    List,
    #[clap(about = "Show registry detail")]
    Show {
        #[clap(short = 'n', long = "name")]
        name: Option<String>,
    },
    #[clap(about = "Add or update a registry")]
    Add {
        #[clap(help = "Registry name")]
        name: String,
        #[clap(help = "Registry URL")]
        url: String,
        #[clap(short = 'H', long = "home", help = "Registry homepage URL")]
        home: Option<String>,
    },
    #[clap(about = "Rename a registry")]
    Rename { name: String, new_name: String },
    #[clap(about = "Delete a registry")]
    Delete { name: String },
    #[clap(about = "Use a registry")]
    Use { name: String },
    #[clap(about = "Open a registry's homepage in browser")]
    Home {
        name: Option<String>,
        #[clap(short = 'b', long = "browser")]
        browser: Option<String>,
    },
}
