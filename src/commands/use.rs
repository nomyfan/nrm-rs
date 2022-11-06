use anyhow::Ok;

use crate::config::{NPMRC_HOME, NPMRC_URL};
use crate::utils::{get_all_registries, npmrc_set};

pub(crate) fn cmd_use(name: String) -> anyhow::Result<()> {
    let registries = get_all_registries();

    match registries.into_iter().find(|x| x.name[..] == name[..]) {
        None => {
            eprintln!("No registry named \"{}\" found.", &name);
            std::process::exit(1);
        }
        Some(registry) => {
            npmrc_set(NPMRC_URL, registry.url);
            if let Some(home) = registry.home {
                npmrc_set(NPMRC_HOME, home);
            }

            Ok(())
        }
    }
}
