use crate::utils::get_all_registries;

pub(crate) fn cmd_current() {
    let registries = get_all_registries();
    if let Some(current_registry) = registries.into_iter().find(|x| x.in_use) {
        println!("{}:", &current_registry.name);
        println!("  Registry: {}", &current_registry.url);
        if let Some(home) = &current_registry.home {
            println!("  Home: {}", home);
        }
    }
}
