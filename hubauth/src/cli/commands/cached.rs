use super::get_config;
use crate::cli::model::CachedOpts;

pub fn call(opts: CachedOpts) {
    let configuration = get_config(opts.config, opts.cache_dir);

    if let Some(user) = configuration.users.get(&opts.username) {
        let result = if user.cache {
            user.source_urls_refs()
                .into_iter()
                .map(|url| {
                    format!(
                        "# {}:\n{}",
                        url,
                        configuration.get_cache_for_user(user, url)
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        } else {
            "# caching is disabled for this user".to_string()
        };
        println!("# keys for {}:\n\n{}", &opts.username, result)
    } else {
        println!("# no user {}", &opts.username);
    }
}
