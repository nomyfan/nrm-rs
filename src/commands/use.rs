use anyhow::Ok;

use crate::{
    config::{NpmRegistry, KV, NPMRC, NPMRC_HOME, NPMRC_URL},
    utils::{get_all_registries, read_npmrc, write_npmrc},
};

pub(crate) fn cmd_use(name: String) -> anyhow::Result<()> {
    let registries = get_all_registries();

    match registries.into_iter().find(|x| &x.name[..] == &name[..]) {
        None => {
            eprintln!("No registry named \"{}\" found.", &name);
            std::process::exit(1);
        }
        Some(registry) => {
            let NpmRegistry { url, home, kvs, .. } = registry;
            let mut new_npmrc: NPMRC = vec![(NPMRC_URL.to_string(), url)];

            if let Some(home) = home {
                new_npmrc.push((NPMRC_HOME.to_string(), home));
            }

            match read_npmrc()? {
                Some(npmrc) => {
                    new_npmrc.append(
                        &mut npmrc
                            .into_iter()
                            .filter(|(k, _)| &k[..] != NPMRC_URL && &k[..] != NPMRC_HOME)
                            .collect::<Vec<KV>>(),
                    );

                    // Merge KeyValues
                    if let Some(kvs) = kvs {
                        for (k, v) in kvs.into_iter() {
                            if let Some(nth) =
                                new_npmrc.iter().position(|(key, _)| &key[..] == &k[..])
                            {
                                // Overwrite
                                new_npmrc[nth] = (k, v);
                            }
                        }
                    }

                    write_npmrc(new_npmrc);
                    Ok(())
                }
                None => {
                    if let Some(mut kvs) = kvs {
                        new_npmrc.append(&mut kvs);
                    }

                    write_npmrc(new_npmrc);
                    Ok(())
                }
            }
        }
    }
}
