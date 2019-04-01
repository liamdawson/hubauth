use std::path::PathBuf;

pub fn bin_path() -> PathBuf {
    std::env::current_exe().unwrap()
}

pub fn call(sshd_config_path: &str, command: &str, username: &str) {
    println!("Initializing for path {:?} in config file {} using command {} and username {}.", bin_path(), sshd_config_path, command, username);
    // let sshd_file =

}

