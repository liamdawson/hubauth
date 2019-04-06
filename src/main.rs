#[macro_use]
extern crate serde;

mod cli;

use cli::{Call, CliOptions};
use gumdrop::Options;

fn main() {
    // CLI model is in cli/model.rs
    // command implementations are in cli/commands/*.rs

    CliOptions::parse_args_default_or_exit().call();
}
