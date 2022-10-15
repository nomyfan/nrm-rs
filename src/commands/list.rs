use crate::utils::get_all_registries;

pub(crate) fn cmd_list() {
    let registries = get_all_registries();

    let width = registries.iter().map(|x| x.name.len()).max().unwrap() + 5;

    for r in &registries {
        println!(
            "{0}{1:-<width$}{2}",
            if r.in_use { "* " } else { "  " },
            &r.name,
            &r.registry
        );
    }
}
