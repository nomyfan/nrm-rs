use crate::config::{get_preset_registries, NpmRegistry};
use crate::config::{KEY_HOME, KEY_REGISTRY};

pub(crate) fn npmrc_path() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().unwrap();

    home_dir.join(".npmrc")
}

pub(crate) fn nrmrc_path() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().unwrap();

    home_dir.join(".nrmrc")
}

// pub(crate) fn read_npmrc() -> NpmRegistry {
//     todo!()
// }

// pub(crate) fn write_npmrc(registry: NpmRegistry) {
//     todo!()
// }

pub(crate) fn read_nrmrc() -> Vec<NpmRegistry> {
    let mut registries = vec![];
    let nrmrc = ini::Ini::load_from_file(nrmrc_path()).unwrap();

    for (name, props) in &nrmrc {
        let mut kvs: Vec<(String, String)> = vec![];
        for (k, v) in props
            .iter()
            .filter(|(k, _)| k != &"registry" && k != &"home")
        {
            kvs.push((k.to_string(), v.to_string()));
        }
        let mut registry = NpmRegistry::new(
            name.unwrap(),
            props.get(KEY_REGISTRY).unwrap(),
            props.get(KEY_HOME),
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
        let mut section_setter = section.set(KEY_REGISTRY, &registry.url[..]);
        if let Some(home) = &registry.home {
            section_setter = section_setter.set(KEY_HOME, &home[..]);
        }

        if registry.kvs.is_some() {
            for (k, v) in registry.kvs.as_ref().unwrap().iter() {
                section_setter = section_setter.set(&k[..], &v[..]);
            }
        }
    }

    nrmrc_ini.write_to_file(nrmrc_path()).unwrap()
}

pub(crate) fn set_in_use(mut registries: Vec<NpmRegistry>) -> Vec<NpmRegistry> {
    let mut set = false;
    if let Ok(npmrc) = ini::Ini::load_from_file(npmrc_path()) {
        if let Some(global_section) = npmrc.section(None::<String>) {
            if let Some(registry) = global_section.get(KEY_REGISTRY) {
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
