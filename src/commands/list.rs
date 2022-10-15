use crate::config::get_embed_registries;
use crate::utils::{get_nrmrc, set_in_use};

pub(crate) fn cmd_list() {
    let embed_registries = get_embed_registries();
    let nrmrc = get_nrmrc();

    let registries = set_in_use([embed_registries, nrmrc].concat());

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
