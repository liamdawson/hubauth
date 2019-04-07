use crate::fs_cache::FilesystemCacher;
use std::collections::HashMap;
use std::path::Path;

pub struct State {
    pub caching: Caching,
    pub users: HashMap<String, User>,
}

pub struct Caching {
    pub destination: String,
    pub min_age: u64,
    pub max_age: u64,
}

pub struct User {
    pub source_urls: Vec<String>,
    pub cache: bool,
}

impl User {
    pub fn source_urls_refs(&self) -> Vec<&str> {
        self.source_urls
            .iter()
            .map(|url| url.as_str())
            .collect::<Vec<_>>()
    }
}

impl State {
    pub fn get_cacher(&self) -> FilesystemCacher<'_> {
        FilesystemCacher::new(Path::new(&self.caching.destination))
    }

    pub fn get_cache_for_user(&self, user: &User, key: &str) -> String {
        if user.cache {
            self.get_cacher().get_lossy(key, self.caching.max_age)
        } else {
            String::new()
        }
    }

    pub fn set_cache_for_user(&self, user: &User, key: &str, content: &str) {
        if user.cache {
            self.get_cacher().set_lossy(key, content);
        }
    }
}
