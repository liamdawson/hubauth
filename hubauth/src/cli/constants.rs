pub fn config_path() -> String {
    option_env!("BIN_DEFAULT_CONFIG_PATH")
        .unwrap_or("/etc/hubauth.yml")
        .to_owned()
}

pub fn sshd_config_path() -> String {
    option_env!("BIN_DEFAULT_SSHD_CONFIG_PATH")
        .unwrap_or("/etc/ssh/sshd_config")
        .to_owned()
}

pub fn cache_path() -> String {
    option_env!("BIN_DEFAULT_CACHE_DIR")
        .unwrap_or("/var/cache/hubauth")
        .to_owned()
}

pub fn cache_user() -> String {
    option_env!("BIN_DEFAULT_USERNAME")
        .unwrap_or("root")
        .to_owned()
}

pub const EXIT_INVOCATION_ERROR: i32 = 2;
pub const EXIT_CONFIGURATION_ERROR: i32 = 3;
