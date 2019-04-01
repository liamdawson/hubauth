pub fn default_config() -> &'static str {
    option_env!("BIN_DEFAULT_CONFIG_PATH").unwrap_or("/etc/hubauth.yml")
}

pub fn default_cache() -> &'static str {
    option_env!("BIN_DEFAULT_CACHE_DIR").unwrap_or("/var/cache/hubauth")
}

pub fn default_username() -> &'static str {
    option_env!("BIN_DEFAULT_USERNAME").unwrap_or("root")
}
