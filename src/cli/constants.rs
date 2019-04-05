pub fn default_config() -> String {
    option_env!("BIN_DEFAULT_CONFIG_PATH")
        .unwrap_or("/etc/hubauth.yml")
        .to_owned()
}

pub fn default_sshd_config() -> String {
    option_env!("BIN_DEFAULT_SSHD_CONFIG_PATH")
        .unwrap_or("/etc/ssh/sshd_config")
        .to_owned()
}

pub fn default_cache() -> String {
    option_env!("BIN_DEFAULT_CACHE_DIR")
        .unwrap_or("/var/cache/hubauth")
        .to_owned()
}

pub fn default_username() -> String {
    option_env!("BIN_DEFAULT_USERNAME")
        .unwrap_or("root")
        .to_owned()
}
