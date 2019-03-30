mod cli;
mod config;

use std::collections::HashMap;
use std::iter::FromIterator;

pub fn generate() -> HashMap<&'static str, String> {
    let manuals: Vec<_> = vec![cli::generate(), config::generate()]
        .into_iter()
        .flatten()
        .collect();
    HashMap::from_iter(manuals)
}
