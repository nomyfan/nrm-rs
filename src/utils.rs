use crate::config::{get_preset_registries, NpmRegistry, KV, NPMRC};
use crate::config::{NPMRC_HOME, NPMRC_URL};
use anyhow::{bail, Result};

pub(crate) fn npmrc_path() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().unwrap();

    home_dir.join(".npmrc")
}

pub(crate) fn nrmrc_path() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().unwrap();

    home_dir.join(".nrmrc")
}

pub(crate) fn read_npmrc() -> Result<Option<NPMRC>> {
    match ini::Ini::load_from_file(npmrc_path()) {
        Ok(npmrc_ini) => {
            let props = npmrc_ini.general_section();
            let kvs = props
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<Vec<(String, String)>>();

            Ok(Some(kvs))
        }
        Err(ini::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => {
            bail!(err)
        }
    }
}

pub(crate) fn write_npmrc(npmrc: NPMRC) {
    let mut npmrc_ini = ini::Ini::new();
    let mut section = &mut npmrc_ini.with_general_section();

    for (k, v) in npmrc.into_iter() {
        section = section.set(k, v);
    }

    npmrc_ini.write_to_file(npmrc_path()).unwrap();
}

pub(crate) fn read_nrmrc() -> Vec<NpmRegistry> {
    let mut registries = vec![];
    let nrmrc = ini::Ini::load_from_file(nrmrc_path()).unwrap(); // TODO Handle the case where the file is not created yet.

    for (name, props) in &nrmrc {
        let kvs = props
            .iter()
            .filter(|(k, _)| &k[..] != NPMRC_URL && &k[..] != NPMRC_HOME)
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<KV>>();
        let mut registry = NpmRegistry::new(
            name.unwrap(),
            props.get(NPMRC_URL).unwrap(),
            props.get(NPMRC_HOME),
        );
        registry.kvs = Some(kvs);
        registries.push(registry);
    }

    registries
}

pub(crate) fn write_nrmrc(registries: &Vec<NpmRegistry>) {
    let mut nrmrc_ini = ini::Ini::new();
    for registry in registries.iter() {
        let name = &registry.name[..];
        let mut section = nrmrc_ini.with_section(Some(name));
        let mut section = section.set(NPMRC_URL, &registry.url[..]);
        if let Some(home) = &registry.home {
            section = section.set(NPMRC_HOME, &home[..]);
        }

        if registry.kvs.is_some() {
            for (k, v) in registry.kvs.as_ref().unwrap().iter() {
                section = section.set(&k[..], &v[..]);
            }
        }
    }

    nrmrc_ini.write_to_file(nrmrc_path()).unwrap()
}

pub(crate) fn set_in_use(mut registries: Vec<NpmRegistry>) -> Vec<NpmRegistry> {
    let mut set = false;
    if let Ok(npmrc) = ini::Ini::load_from_file(npmrc_path()) {
        if let Some(global_section) = npmrc.section(None::<String>) {
            if let Some(registry) = global_section.get(NPMRC_URL) {
                if let Some(target) = registries.iter_mut().find(|x| &x.url == registry) {
                    target.in_use = true;
                    set = true;
                }
            }
        }
    }

    if !set {
        if let Some(npm) = registries.iter_mut().find(|x| x.name == "npm") {
            npm.in_use = true;
        }
    }

    registries
}

pub(crate) fn get_all_registries() -> Vec<NpmRegistry> {
    let nrmrc = read_nrmrc();
    let embed_registries = get_preset_registries()
        .into_iter()
        .filter(|x| !nrmrc.iter().any(|y| y.name == x.name || y.url == x.url))
        .collect();

    set_in_use([embed_registries, nrmrc].concat())
}
