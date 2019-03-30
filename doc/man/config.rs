use man::prelude::*;

pub fn generate() -> Vec<(&'static str, String)> {
    vec![("hubauth.yml.5", Manual::new("hubauth.yml").render())]
}
