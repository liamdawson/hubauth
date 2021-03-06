use super::constants::cache_path;
use hubauth::models::*;
use std::collections::HashMap;
use std::iter::FromIterator;

// 15 seconds
const DEFAULT_MINIMUM_AGE: u64 = 15;
// 30 days
const DEFAULT_MAXIMUM_AGE: u64 = 60 * 60 * 24 * 30;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub caching: Option<CachingRaw>,
    pub users: HashMap<String, UserRaw>,
}

#[derive(Debug, Deserialize)]
pub struct CachingRaw {
    pub destination: Option<String>,
    pub minimum_age: Option<u64>,
    pub maximum_age: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct UserRaw {
    pub cache: Option<bool>,
    pub key_sources: Vec<UserKeySourceRaw>,
}

#[derive(Debug, Deserialize)]
pub struct UserKeySourceRaw {
    pub url: Option<String>,
    pub github: Option<String>,
}

impl Into<User> for UserRaw {
    fn into(self) -> User {
        User {
            cache: self.cache.unwrap_or(true),
            source_urls: self
                .key_sources
                .into_iter()
                .flat_map(|src| {
                    if let Some(url) = src.url {
                        Some(url)
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

impl Into<Caching> for CachingRaw {
    fn into(self) -> Caching {
        Caching {
            destination: self.destination.unwrap_or_else(cache_path),
            min_age: self.minimum_age.unwrap_or(DEFAULT_MINIMUM_AGE),
            max_age: self.maximum_age.unwrap_or(DEFAULT_MAXIMUM_AGE),
        }
    }
}

pub fn default_caching() -> Caching {
    Caching {
        destination: cache_path(),
        min_age: DEFAULT_MINIMUM_AGE,
        max_age: DEFAULT_MAXIMUM_AGE,
    }
}

impl Into<State> for Configuration {
    fn into(self) -> State {
        State {
            caching: self
                .caching
                .map_or_else(default_caching, |caching| caching.into()),
            users: HashMap::from_iter(
                self.users
                    .into_iter()
                    .map(|(username, config)| (username, config.into())),
            ),
        }
    }
}
