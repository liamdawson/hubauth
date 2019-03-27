use crate::configuration::*;
use crate::fs_cache::FilesystemCacher;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::Path;

const DEFAULT_CACHE_PATH: &str = "/var/cache/hubauth";
// 15 seconds
const DEFAULT_MINIMUM_AGE: u64 = 15;
// 30 days
const DEFAULT_MAXIMUM_AGE: u64 = 60 * 60 * 24 * 30;

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

impl From<UserConfiguration> for User {
    fn from(user: UserConfiguration) -> Self {
        User {
            cache: user.cache.unwrap_or(true),
            source_urls: user
                .key_sources
                .into_iter()
                .flat_map(|src| {
                    if let Some(url) = src.url {
                        Some(url.to_owned())
                    } else if let Some(handle) = src.github {
                        Some(format!("https://github.com/{}.keys", handle))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
}

impl From<Option<CachingConfiguration>> for Caching {
    fn from(cache_option: Option<CachingConfiguration>) -> Self {
        if let Some(cache) = cache_option {
            Caching {
                destination: cache
                    .destination
                    .unwrap_or_else(|| String::from(DEFAULT_CACHE_PATH)),
                min_age: cache.minimum_age.unwrap_or(DEFAULT_MINIMUM_AGE),
                max_age: cache.maximum_age.unwrap_or(DEFAULT_MAXIMUM_AGE),
            }
        } else {
            Caching {
                destination: String::from(DEFAULT_CACHE_PATH),
                min_age: DEFAULT_MINIMUM_AGE,
                max_age: DEFAULT_MAXIMUM_AGE,
            }
        }
    }
}

impl From<Configuration> for State {
    fn from(configuration: Configuration) -> Self {
        State {
            caching: Caching::from(configuration.caching),
            users: HashMap::from_iter(
                configuration
                    .users
                    .into_iter()
                    .map(|(username, config)| (username, User::from(config))),
            ),
        }
    }
}

impl State {
    pub fn get_cacher(&self) -> FilesystemCacher {
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
