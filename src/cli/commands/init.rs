use crate::cli::constants::{default_sshd_config, default_username};
use crate::cli::model::{InitOptCommand, InitOpts};
use std::borrow::Cow;

pub fn bin_path() -> String {
    std::env::current_exe()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

pub fn call(opts: InitOpts) {
    match inner(
        &opts.sshd_config.unwrap_or_else(|| default_sshd_config()),
        &opts.command.unwrap_or(InitOptCommand::List).to_string(),
        &opts.username.unwrap_or_else(|| default_username()),
        !opts.no_backup,
    ) {
        Ok(msg) => eprintln!("{}", msg),
        Err(err) => {
            eprintln!("unable to configure the sshd_config: {}", err);
            std::process::exit(101);
        }
    }
}

fn inner(
    sshd_config_path: &str,
    command: &str,
    username: &str,
    make_backup: bool,
) -> std::io::Result<String> {
    let current_config = std::fs::read_to_string(sshd_config_path)?;

    let configured_check = regex::RegexSet::new(&[
        r"(?i)(?m)^\s*AuthorizedKeysCommand\s.*$",
        r"(?i)(?m)^\s*AuthorizedKeysCommandUser\s.*$",
    ])
    .unwrap();

    let command_comment = regex::Regex::new(r"(?i)(?m)^\s*#\s*AuthorizedKeysCommand\s.*$").unwrap();
    let user_comment =
        regex::Regex::new(r"(?i)(?m)^\s*#\s*AuthorizedKeysCommandUser\s.*$").unwrap();

    if configured_check.is_match(&current_config) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "AuthorizedKeysCommand or AuthorizedKeysCommandUser is already configured",
        ));
    }

    if make_backup {
        std::fs::write(format!("{}.bak", sshd_config_path), &current_config)?;
    }

    let command_text: &str = &format!("AuthorizedKeysCommand {} {}\n", bin_path(), command);
    let current_config = if command_comment.is_match(&current_config) {
        command_comment.replace(&current_config, command_text)
    } else {
        Cow::from(format!("{}\n{}", &current_config, command_text))
    };

    let user_text: &str = &format!("AuthorizedKeysCommandUser {}\n", username);
    let current_config = if user_comment.is_match(&current_config) {
        user_comment.replace(&current_config, user_text)
    } else {
        Cow::from(format!("{}\n{}", &current_config, user_text))
    };

    std::fs::write(sshd_config_path, current_config.as_bytes())?;

    Ok(String::from("successfully initialized sshd_config"))
}
