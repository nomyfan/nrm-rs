use crate::config::{NPMRC_HOME, UNDEFINED};
use crate::utils::{get_all_registries, npmrc_get};

fn open_in_browser<S: AsRef<str>>(home: S, browser: Option<String>) {
    if let Err(error) = match browser {
        Some(browser) => open::with(home.as_ref(), browser),
        None => open::that(home.as_ref()),
    } {
        eprintln!("Failed to open homepage because of `{:?}`", error);
    }
}

pub(crate) fn cmd_home(name: Option<String>, browser: Option<String>) {
    let registries = get_all_registries();

    match match name {
        Some(name) => registries.into_iter().find(|x| x.name[..] == name[..]),
        None => registries.into_iter().find(|x| x.in_use),
    } {
        Some(registry) => match &registry.home {
            None => {
                eprintln!("Registry \"{}\" has no homepage.", &registry.name[..]);
                std::process::exit(1);
            }
            Some(home) => {
                open_in_browser(home, browser);
            }
        },
        None => match npmrc_get(NPMRC_HOME) {
            Some(home) if home != UNDEFINED => {
                open_in_browser(home, browser);
            }
            _ => {
                eprintln!("No homepage found.");
                std::process::exit(1);
            }
        },
    }
}
