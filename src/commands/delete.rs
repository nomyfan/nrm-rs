use crate::{
    config::{get_preset_registries, NpmRegistry, NPMRC_HOME, NPMRC_URL},
    utils::{get_all_registries, npmrc_delete, write_nrmrc},
};

pub(crate) fn cmd_delete(name: String) {
    if get_preset_registries()
        .iter()
        .any(|x| x.name[..] == name[..])
    {
        eprintln!("Cannot delete the preset registry.");
        std::process::exit(1);
    }

    let mut registries: Vec<NpmRegistry> = get_all_registries();
    match registries.iter().position(|x| x.name[..] == name[..]) {
        None => {
            eprintln!("No registry named \"{}\" found.", &name);
            std::process::exit(1);
        }
        Some(nth) => {
            let in_use = registries[nth].in_use;
            registries.remove(nth);
            if in_use {
                npmrc_delete(NPMRC_URL);
                npmrc_delete(NPMRC_HOME);
            }

            write_nrmrc(registries);
        }
    }
}
