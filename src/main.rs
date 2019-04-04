#[macro_use]
extern crate serde;

mod cli;

use crate::cli::model::ConfigArguments;
use cli::commands;
use cli::configuration::Configuration;
use cli::{CommandLineArguments, Subcommand};
use config::{Config, File};
use hubauth::models::State;
use structopt::StructOpt;

fn main() {
    let args = CommandLineArguments::from_args();

    match &args.command {
        Subcommand::Fetch { user_args, config } => {
            commands::fetch::call(&config.into(), &user_args.username)
        }
        Subcommand::Sync { config } => commands::sync::call(&config.into()),
        Subcommand::Cached { user_args, config } => {
            commands::cached::call(&config.into(), &user_args.username)
        }
        Subcommand::List { user_args, config } => {
            commands::list::call(&config.into(), &user_args.username)
        }
        Subcommand::Init {
            command,
            sshd_config_path,
            username,
        } => commands::init::call(&sshd_config_path, &command, &username),
    }
}

impl Into<State> for &ConfigArguments {
    fn into(self) -> State {
        let mut config = Config::new();
        if let Err(err) = config.merge(File::with_name(self.config_file_path.to_str().unwrap())) {
            eprintln!("error: {:?}", err);
            std::process::exit(2);
        }

        let mut configuration: State = match config.try_into::<Configuration>() {
            Ok(configuration) => configuration.into(),
            Err(err) => {
                eprintln!("error: {:?}", err);
                std::process::exit(3);
            }
        };

        if let Some(cache_path) = &self.cache_directory {
            configuration.caching.destination = cache_path.to_owned();
        }

        configuration
    }
}
