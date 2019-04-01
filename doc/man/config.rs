use crate::doc::constants::{default_cache, default_config};
use man::prelude::*;

pub fn generate() -> Vec<(&'static str, String)> {
    vec![(
        "hubauth.yml.5",
        Manual::new("hubauth.yml")
            .description(format!("{}", default_config()))
            .option(
                Opt::new("caching-destination")
                    .long("cache.destination")
                    .default_value(default_cache()),
            )
            .render(),
    )]
}
