use crate::{
    config::NpmRegistry,
    utils::{get_all_registries, write_nrmrc},
};

pub(crate) fn cmd_add(name: String, url: String, home: Option<String>) {
    let mut registries = get_all_registries();

    let normalized_url = if url.ends_with('/') { url } else { url + "/" };

    if registries.iter().any(|x| x.url[..] == normalized_url[..]) {
        eprintln!("The registry url is already included in the nrm registries. Please make sure that the url are unique.");
        std::process::exit(1);
    }

    match registries.iter().position(|x| x.name[..] == name[..]) {
        Some(nth) => {
            let mut registry = &mut registries[nth];
            registry.name = name;
            registry.url = normalized_url;
            registry.home = home;
        }
        None => {
            registries.push(NpmRegistry::new(name, normalized_url, home));
        }
    };

    write_nrmrc(registries);
}
