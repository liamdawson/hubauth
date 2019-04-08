#![allow(clippy::default_trait_access)]

use super::constants::config_path;
use gumdrop::Options;
use std::str::FromStr;
use std::string::ToString;

const USERNAME_VALIDATION_FAILED_MESSAGE: &str = "Usernames should be a maximum of 31 characters, start with an underscore or lowercase letter, and contain only lowercase letters, numerals, underscores or periods.";
const USERNAME_REGEX_PATTERN: &str = "^[a-z_][a-z0-9_.]{0,30}$";

fn username_validator() -> regex::Regex {
    regex::Regex::new(USERNAME_REGEX_PATTERN).unwrap()
}

fn parse_username(input: &str) -> Result<String, String> {
    if username_validator().is_match(input) {
        Ok(input.to_owned())
    } else {
        Err(String::from(USERNAME_VALIDATION_FAILED_MESSAGE))
    }
}

#[derive(Options)]
pub struct CliOptions {
    #[options(help_flag, help = "Print help message")]
    pub help: bool,
    #[options(short = "V", help = "Print application version")]
    pub version: bool,
    // not marked as required, or version wouldn't work
    #[options(command)]
    pub command: Option<CliCommands>,
}

#[derive(Options)]
pub enum CliCommands {
    #[options(help = "Add AuthorizedKeys configuration to sshd_config, use hubauth on SSH login")]
    Init(InitOpts),
    #[options(help = "Retrieve the remote keys for a user")]
    Fetch(FetchOpts),
    #[options(help = "Update the cached keys for all users with caching enabled")]
    Sync(SyncOpts),
    #[options(help = "Retrieve the cached keys for a user")]
    Cached(CachedOpts),
    #[options(help = "List the remote keys for a user, falling back to cache when enabled")]
    List(ListOpts),
}

#[derive(Options)]
pub struct FetchOpts {
    #[options(help = "Path to the config file", default_expr = "config_path()")]
    pub config: String,
    #[options(help = "Path to the cache directory (takes precedence)")]
    pub cache_dir: Option<String>,
    #[options(
        free,
        help = "Username to lookup to find key sources",
        parse(try_from_str = "parse_username")
    )]
    pub username: String,
}

#[derive(Options)]
pub struct SyncOpts {
    #[options(help = "Path to the config file", default_expr = "config_path()")]
    pub config: String,
    #[options(help = "Path to the cache directory (takes precedence)")]
    pub cache_dir: Option<String>,
}

#[derive(Options)]
pub struct CachedOpts {
    #[options(help = "Path to the config file", default_expr = "config_path()")]
    pub config: String,
    #[options(help = "Path to the cache directory (takes precedence)")]
    pub cache_dir: Option<String>,
    #[options(
        free,
        help = "Username to lookup to find key sources",
        parse(try_from_str = "parse_username")
    )]
    pub username: String,
}

#[derive(Options)]
pub struct ListOpts {
    #[options(help = "Path to the config file", default_expr = "config_path()")]
    pub config: String,
    #[options(help = "Path to the cache directory (takes precedence)")]
    pub cache_dir: Option<String>,
    #[options(
        free,
        help = "Username to lookup to find key sources",
        parse(try_from_str = "parse_username")
    )]
    pub username: String,
}

#[derive(Options)]
pub struct InitOpts {
    #[options(no_short, help = "Path to the sshd_config file")]
    pub sshd_config: Option<String>,
    #[options(
        help = "User with access to the hubauth cache and config file",
        parse(try_from_str = "parse_username")
    )]
    pub username: Option<String>,
    #[options(no_short, help = "Don't create a backup of the sshd_config file")]
    pub no_backup: bool,
    #[options(
        free,
        help = "Subcommand of hubauth to use on login (list, cached or fetch)"
    )]
    pub command: Option<InitOptCommand>,
}

pub enum InitOptCommand {
    List,
    Cached,
    Fetch,
}

impl FromStr for InitOptCommand {
    type Err = String;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "list" => Ok(InitOptCommand::List),
            "fetch" => Ok(InitOptCommand::Fetch),
            "cached" => Ok(InitOptCommand::Cached),
            _ => Err(format!(
                "{} was not an expected command, expected list, fetch or cached",
                name
            )),
        }
    }
}

impl ToString for InitOptCommand {
    fn to_string(&self) -> String {
        match self {
            InitOptCommand::List => "list".to_owned(),
            InitOptCommand::Fetch => "fetch".to_owned(),
            InitOptCommand::Cached => "cached".to_owned(),
        }
    }
}

pub trait Call {
    fn call(self);
}
