use anyhow::Ok;

use crate::utils::{get_all_registries, read_npmrc, write_npmrc};

pub(crate) fn cmd_use(name: String) -> anyhow::Result<()> {
    let registries = get_all_registries();

    match registries.into_iter().find(|x| x.name[..] == name[..]) {
        None => {
            eprintln!("No registry named \"{}\" found.", &name);
            std::process::exit(1);
        }
        Some(registry) => match read_npmrc()? {
            Some(npmrc) => {
                let mut new_npmrc = npmrc;
                new_npmrc.append(&mut registry.into());

                write_npmrc(new_npmrc);
                Ok(())
            }
            None => {
                write_npmrc(registry.into());
                Ok(())
            }
        },
    }
}
