use crate::config::{get_preset_registries, NpmRegistry, KV, NPMRC};
use crate::config::{NPMRC_HOME, NPMRC_URL, UNDEFINED};
use xshell::{cmd, Shell};

pub(crate) fn nrmrc_path() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().unwrap();

    home_dir.join(".nrmrc")
}

pub(crate) fn npmrc_get<S: AsRef<str>>(prop: S) -> Option<String> {
    let sh = Shell::new().unwrap();
    let prop = prop.as_ref();

    cmd!(sh, "npm config get {prop}").quiet().read().ok()
}

pub(crate) fn npmrc_delete<S: AsRef<str>>(prop: S) {
    let sh = Shell::new().unwrap();
    let prop = prop.as_ref();

    cmd!(sh, "npm config delete {prop}").quiet().run().unwrap();
}

pub(crate) fn npmrc_set<S: AsRef<str>, V: AsRef<str>>(prop: S, value: V) {
    let sh = Shell::new().unwrap();
    let prop = prop.as_ref();
    let value = value.as_ref();

    cmd!(sh, "npm config set {prop} {value}")
        .quiet()
        .run()
        .unwrap();
}

pub(crate) fn read_nrmrc() -> Vec<NpmRegistry> {
    let mut registries = vec![];

    match ini::Ini::load_from_file(nrmrc_path()) {
        Ok(nrmrc) => {
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
        Err(_) => vec![],
    }
}

pub(crate) fn write_nrmrc(registries: Vec<NpmRegistry>) {
    let mut nrmrc_ini = ini::Ini::new();

    for registry in registries.into_iter() {
        let name = &registry.name[..];
        let mut section = &mut nrmrc_ini.with_section(Some(name));

        let npmrc: NPMRC = registry.into();
        for (key, value) in npmrc {
            section = section.set(key, value);
        }
    }

    nrmrc_ini.write_to_file(nrmrc_path()).unwrap()
}

pub(crate) fn set_in_use(mut registries: Vec<NpmRegistry>) -> Vec<NpmRegistry> {
    let mut has_url_in_npmrc = false;

    match npmrc_get(NPMRC_URL) {
        Some(url) if &url[..] != UNDEFINED => {
            has_url_in_npmrc = true;
            if let Some(target) = registries.iter_mut().find(|x| x.url[..] == url[..]) {
                target.in_use = true;
            }
        }
        _ => {}
    };

    if !has_url_in_npmrc {
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
