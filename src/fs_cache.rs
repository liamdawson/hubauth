use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Converts a key (which may be an invalid filename) to a deterministic,
/// filename-safe (hexadecimal) string
fn hash_key(key: &str) -> String {
    format!("{:x}", Sha256::digest(key.as_bytes()))
}

pub struct FilesystemCacher<'a> {
    pub destination: &'a Path,
}

impl<'a> FilesystemCacher<'a> {
    pub fn new(path: &'a Path) -> Self {
        FilesystemCacher { destination: path }
    }

    fn path_for_key(&self, key: &str) -> PathBuf {
        self.destination.join(hash_key(key))
    }

    fn key_modified_time(&self, key: &str) -> std::io::Result<SystemTime> {
        File::open(self.path_for_key(key))?.metadata()?.modified()
    }

    fn key_age(&self, key: &str) -> Option<Duration> {
        if let Ok(modified_time) = self.key_modified_time(key) {
            // failure means the file is future dated, so say no time difference
            Some(
                SystemTime::now()
                    .duration_since(modified_time)
                    .unwrap_or_else(|_| Duration::new(0, 0)),
            )
        } else {
            None
        }
    }

    pub fn key_older_than(&self, key: &str, max_age: u64) -> Option<bool> {
        self.key_age(key)
            .map(|age| age > Duration::from_secs(max_age))
    }

    pub fn key_newer_than(&self, key: &str, max_age: u64) -> Option<bool> {
        self.key_age(key)
            .map(|age| age < Duration::from_secs(max_age))
    }

    /// Indicates if the given key has a valid cache entry, with a maximum age of
    /// `max_age_option` (if supplied).
    ///
    /// Will fail if the file modified time metadata was inaccessible.
    pub fn contains_key(&self, key: &str, max_age: u64) -> bool {
        self.key_newer_than(key, max_age) == Some(true)
    }

    pub fn get(&self, key: &str, max_age: u64) -> std::io::Result<String> {
        if self.contains_key(key, max_age) {
            std::fs::read_to_string(self.path_for_key(key))
        } else {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        }
    }

    pub fn get_lossy(&self, key: &str, max_age: u64) -> String {
        self.get(key, max_age).unwrap_or_else(|_| String::new())
    }

    pub fn set(&self, key: &str, content: &str) -> std::io::Result<()> {
        std::fs::write(self.path_for_key(key), content)
    }

    pub fn set_lossy(&self, key: &str, content: &str) {
        self.set(key, content).is_ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn it_deterministically_hashes_key_names() {
        // if this value changes, it's a breaking version change
        assert_eq!(
            "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3",
            hash_key("Hello, world!")
        );
    }

    #[test]
    fn it_generates_the_expected_file_path() {
        let cacher = FilesystemCacher::new(&Path::new("foo"));

        assert_eq!(
            PathBuf::from("foo/315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"),
            cacher.path_for_key("Hello, world!")
        );
    }
}
