#[macro_use]
extern crate serde;

mod cli;

use config::{Config, File};
use structopt::StructOpt;
use cli::configuration::Configuration;
use cli::{CommandLineArguments, Subcommand};
use cli::commands;

fn main() {
    let args = CommandLineArguments::from_args();

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
        Subcommand::Fetch { user_args } => {
            commands::fetch::call(&configuration, &user_args.username)
        }
        Subcommand::Sync => commands::sync::call(&configuration),
        Subcommand::Cached { user_args } => {
            commands::cached::call(&configuration, &user_args.username)
        }
        Subcommand::List { user_args } => {
            commands::list::call(&configuration, &user_args.username)
        },
        Subcommand::Init { command, sshd_config_path, username } => {
            commands::init::call(&sshd_config_path, &command, &username)
        }
    }
}
