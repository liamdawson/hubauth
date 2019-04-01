use super::constants::{default_config, default_cache, default_username};
use std::path::PathBuf;
use structopt::StructOpt;

const USERNAME_VALIDATION_FAILED_MESSAGE: &str = "Usernames should be a maximum of 31 characters, start with an underscore or lowercase letter, and contain only lowercase letters, numerals, underscores or periods.";
const USERNAME_REGEX_PATTERN: &str = "^[a-z_][a-z0-9_.]{0,30}$";

fn username_validator() -> regex::Regex {
    regex::Regex::new(USERNAME_REGEX_PATTERN).unwrap()
}

fn validate_username(username: String) -> Result<(), String> {
    if username_validator().is_match(&username) {
        Ok(())
    } else {
        Err(String::from(USERNAME_VALIDATION_FAILED_MESSAGE))
    }
}

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct CommandLineArguments {
    #[structopt(
        short = "c",
        long = "config-file",
        help = "Path to the config file",
        raw(default_value = "default_config()"),
        parse(from_os_str)
    )]
    pub config_file_path: PathBuf,
    #[structopt(subcommand)]
    pub command: Subcommand,
}

#[derive(Debug, StructOpt)]
pub struct UserArguments {
    #[structopt(
        name = "USERNAME",
        required = true,
        help = "A username to lookup to find key sources.",
        raw(validator = "validate_username")
    )]
    pub username: String,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    #[structopt(name = "fetch", about = "Retrieve the remote keys for a user")]
    Fetch {
        #[structopt(flatten)]
        user_args: UserArguments,
    },
    #[structopt(name = "cached", about = "Retrieve the cached keys for a user")]
    Cached {
        #[structopt(flatten)]
        user_args: UserArguments,
    },
    #[structopt(
        name = "sync",
        about = "Update the cached keys for all sources belonging to users with enabled caching"
    )]
    Sync,
    #[structopt(
        name = "list",
        about = "List the remote keys for a user, falling back to cache when appropriate"
    )]
    List {
        #[structopt(flatten)]
        user_args: UserArguments,
    },
    /// Add the appropriate configuration to sshd_config to use hubauth
    /// (does not use the config file)
    #[structopt(name = "init")]
    Init {
        /// Path to the sshd_config file
        #[structopt(short = "f", long = "sshd-config", default_value = "/etc/ssh/sshd_config")]
        sshd_config_path: String,
        /// Path to the hubauth cache
        #[structopt(short = "x", long = "command", default_value = "list", raw(possible_values = "&[\"list\", \"cached\", \"fetch\"]"))]
        command: String,
        /// User with access to the hubauth cache and config file
        #[structopt(short = "u", long = "user", raw(default_value = "default_username()"))]
        username: String,
    },
}
