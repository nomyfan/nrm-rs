use crate::utils::get_all_registries;
use open;

pub(crate) fn cmd_home(name: String, browser: Option<String>) {
    let registries = get_all_registries();

    match registries.iter().find(|x| &x.name[..] == &name[..]) {
        None => {
            eprintln!("No registry named \"{}\" found.", &name[..]);
            std::process::exit(1);
        }
        Some(registry) => match &registry.home {
            None => {
                eprintln!("Registry \"{}\" has no homepage.", &name[..]);
                std::process::exit(1);
            }
            Some(home) => {
                if let Err(error) = match browser {
                    Some(browser) => open::with(home, browser),
                    None => open::that(home),
                } {
                    eprintln!("Failed to open homepage because of `{:?}`", error);
                }
            }
        },
    }
}
