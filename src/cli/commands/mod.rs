pub mod cached;
pub mod fetch;
pub mod init;
pub mod list;
pub mod sync;

use super::configuration::Configuration;
use super::constants::default_config;
use super::model::{Call, CliCommands, CliOptions};
use config::{Config, File};
use hubauth::models::State;
use gumdrop::Options;

impl Call for CliOptions {
    fn call(self) {
        if self.version {
            println!("{}", env!("CARGO_PKG_VERSION"));
        } else {
            if let Some(cmd) = self.command {
                cmd.call();
            } else {
                eprintln!("error: a subcommand was expected: \n\n{})", CliOptions::command_list().unwrap());
                std::process::exit(2);
            }
        }
    }
}

impl Call for CliCommands {
    fn call(self) {
        match self {
            CliCommands::Init(opts) => init::call(opts),
            CliCommands::Fetch(opts) => fetch::call(opts),
            CliCommands::Sync(opts) => sync::call(opts),
            CliCommands::Cached(opts) => cached::call(opts),
            CliCommands::List(opts) => list::call(opts),
        }
    }
}

fn get_config(config_path: Option<String>, cache_directory: Option<String>) -> State {
    let mut config = Config::new();
    if let Err(err) = config.merge(File::with_name(
        &config_path.unwrap_or_else(|| default_config().to_owned()),
    )) {
        eprintln!("config error: {:?}", err);
        std::process::exit(2);
    }

    let mut configuration: State = match config.try_into::<Configuration>() {
        Ok(configuration) => configuration.into(),
        Err(err) => {
            eprintln!("config error: {:?}", err);
            std::process::exit(3);
        }
    };

    if let Some(cache_path) = cache_directory {
        configuration.caching.destination = cache_path;
    }

    configuration
}
