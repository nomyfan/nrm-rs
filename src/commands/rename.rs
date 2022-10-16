use crate::{
    config::get_preset_registries,
    utils::{get_all_registries, write_nrmrc},
};

pub(crate) fn cmd_rename(name: String, new_name: String) {
    let embed_registries = get_preset_registries();

    if embed_registries.iter().any(|x| x.name == name[..]) {
        eprintln!("Only custom registries can be renamed.");
        std::process::exit(1);
    }

    let mut registries = get_all_registries();
    match registries.iter().position(|x| x.name[..] == name[..]) {
        None => {
            eprintln!("Cannot find the registry named \"{}\"", &name[..]);
            std::process::exit(1);
        }
        Some(nth) => {
            let element = &mut registries[nth];
            element.name = new_name;
            write_nrmrc(registries);
        }
    };
}
