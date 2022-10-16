use crate::utils::get_all_registries;

pub(crate) fn cmd_show(name: Option<String>) {
    let registries = get_all_registries();

    let registry = if let Some(name) = &name {
        registries.into_iter().find(|x| x.name[..] == name[..])
    } else {
        registries.into_iter().find(|x| x.in_use)
    };

    if name.is_some() && registry.is_none() {
        eprintln!("No registry named \"{}\" found.", name.as_ref().unwrap());
        std::process::exit(2);
    }

    if let Some(current_registry) = registry {
        println!("{}:", &current_registry.name);
        println!("  Registry: {}", &current_registry.url);
        if let Some(home) = &current_registry.home {
            println!("  Home: {}", home);
        }
    }
}
