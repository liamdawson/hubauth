use man::prelude::*;

pub fn generate() -> Vec<(&'static str, String)> {
    vec![intro(), cached(), sync(), list(), fetch()]
}

fn intro() -> (&'static str, String) {
    ("hubauth.1", Manual::new("hubauth").render())
}

fn cached() -> (&'static str, String) {
    ("hubauth-cached.1", Manual::new("hubauth-cached").render())
}

fn sync() -> (&'static str, String) {
    ("hubauth-sync.1", Manual::new("hubauth-sync").render())
}

fn list() -> (&'static str, String) {
    ("hubauth-list.1", Manual::new("hubauth-list").render())
}

fn fetch() -> (&'static str, String) {
    ("hubauth-fetch.1", Manual::new("hubauth-fetch").render())
}
