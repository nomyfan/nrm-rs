use crate::{
    config::NpmRegistry,
    utils::{get_all_registries, write_nrmrc},
};

pub(crate) fn cmd_add(name: String, url: String, home: Option<String>) {
    let mut registries = get_all_registries();

    let normalized_url = if url.ends_with("/") {
        url
    } else {
        url + &"/".to_string()
    };

    if let Some(_) = registries
        .iter()
        .find(|x| x.name == name || x.url == normalized_url)
    {
        eprintln!("The registry name or url is already included in the nrm registries. Please make sure that the name and url are unique.");
        std::process::exit(1);
    }

    let registry = NpmRegistry::new(name, normalized_url, home);
    registries.push(registry);

    write_nrmrc(&registries);
}
