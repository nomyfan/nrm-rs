use crate::config::NpmRegistry;
use crate::config::{KEY_HOME, KEY_REGISTRY};

pub(crate) fn npmrc_path() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().unwrap();

    home_dir.join(".npmrc")
}

pub(crate) fn nrmrc_path() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().unwrap();

    home_dir.join(".nrmrc")
}

pub(crate) fn get_nrmrc() -> Vec<NpmRegistry> {
    let mut registries = vec![];
    let nrmrc = ini::Ini::load_from_file(nrmrc_path()).unwrap();

    for (name, props) in &nrmrc {
        registries.push(NpmRegistry::new(
            name.unwrap(),
            props.get(KEY_REGISTRY).unwrap(),
            props.get(KEY_HOME),
        ));
    }

    registries
}

pub(crate) fn set_in_use(mut registries: Vec<NpmRegistry>) -> Vec<NpmRegistry> {
    let mut set = false;
    if let Ok(npmrc) = ini::Ini::load_from_file(npmrc_path()) {
        if let Some(global_section) = npmrc.section(None::<String>) {
            if let Some(registry) = global_section.get(KEY_REGISTRY) {
                if let Some(target) = registries.iter_mut().find(|x| &x.registry == registry) {
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
