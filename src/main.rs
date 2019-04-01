#[macro_use]
extern crate serde;

mod bin_constants;
mod cli;
mod commands;
mod configuration;

use config::{Config, File};
use configuration::Configuration;
use structopt::StructOpt;

fn main() {
    let args = cli::CommandLineArguments::from_args();

    let mut config = Config::new();
    if let Err(err) = config.merge(File::with_name(args.config_file_path.to_str().unwrap())) {
        eprintln!("error: {:?}", err);
        std::process::exit(2);
    }

    let configuration = match config.try_into::<Configuration>() {
        Ok(configuration) => configuration.into(),
        Err(err) => {
            eprintln!("error: {:?}", err);
            std::process::exit(3);
        }
    };

    match &args.command {
        cli::Subcommand::Fetch { user_args } => {
            commands::fetch::call(&configuration, &user_args.username)
        }
        cli::Subcommand::Sync => commands::sync::call(&configuration),
        cli::Subcommand::Cached { user_args } => {
            commands::cached::call(&configuration, &user_args.username)
        }
        cli::Subcommand::List { user_args } => {
            commands::list::call(&configuration, &user_args.username)
        }
    }
}
