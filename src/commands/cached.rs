use hubauth::models::State;

pub fn call(configuration: &State, username: &str) {
    if let Some(user) = configuration.users.get(username) {
        let result = if user.cache {
            user.source_urls_refs()
                .into_iter()
                .map(|url| {
                    format!(
                        "# {}:\n{}",
                        url,
                        configuration.get_cache_for_user(user, &url)
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        } else {
            "# caching is disabled for this user".to_string()
        };
        println!("# keys for {}:\n\n{}", username, result)
    } else {
        println!("# no user {}", username);
    }
}
