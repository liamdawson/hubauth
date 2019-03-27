use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub caching: Option<CachingConfiguration>,
    pub users: HashMap<String, UserConfiguration>,
}

#[derive(Debug, Deserialize)]
pub struct CachingConfiguration {
    pub destination: Option<String>,
    pub minimum_age: Option<u64>,
    pub maximum_age: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct UserConfiguration {
    pub cache: Option<bool>,
    pub key_sources: Vec<UserKeySourceConfiguration>,
}

#[derive(Debug, Deserialize)]
pub struct UserKeySourceConfiguration {
    pub url: Option<String>,
    pub github: Option<String>,
}
