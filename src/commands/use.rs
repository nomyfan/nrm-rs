use anyhow::Ok;

use crate::{
    config::NPMRC,
    utils::{get_all_registries, read_npmrc, write_npmrc},
};

pub(crate) fn cmd_use(name: String) -> anyhow::Result<()> {
    let registries = get_all_registries();

    match registries.into_iter().find(|x| x.name[..] == name[..]) {
        None => {
            eprintln!("No registry named \"{}\" found.", &name);
            std::process::exit(1);
        }
        Some(registry) => {
            let mut new_npmrc: NPMRC = registry.into();

            match read_npmrc()? {
                Some(npmrc) => {
                    let mut npmrc_pending = npmrc
                        .into_iter()
                        .filter(|(kx, _)| !new_npmrc.iter().any(|(ky, _)| ky[..] == kx[..]))
                        .collect::<NPMRC>();
                    new_npmrc.append(&mut npmrc_pending);

                    write_npmrc(new_npmrc);
                    Ok(())
                }
                None => {
                    write_npmrc(new_npmrc);
                    Ok(())
                }
            }
        }
    }
}
