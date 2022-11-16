use crate::utils::{get_all_registries, npmrc_delete, npmrc_set};

pub(crate) fn cmd_set_scope(scope: String, registry: String) {
    let registries = get_all_registries();

    match registries.into_iter().find(|x| x.name[..] == registry[..]) {
        Some(registry) => {
            npmrc_set(format!("@{scope}:registry"), registry.url);
        }
        None => {
            eprintln!("No registry named {} found.", registry);
            std::process::exit(1);
        }
    }
}

pub(crate) fn cmd_delete_scope(scope: String) {
    npmrc_delete(format!("@{scope}:registry"));
}
