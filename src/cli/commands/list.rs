use hubauth::fetch::{fetch, FetchResult};
use crate::cli::commands::get_config;
use crate::cli::ListOpts;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn call(opts: ListOpts) {
    let configuration = get_config(opts.config, opts.cache_dir);
    let cacher = configuration.get_cacher();

    if let Some(user) = configuration.users.get(&opts.username) {
        let results = user
            .source_urls_refs()
            .into_par_iter()
            .map({
                |url| {
                    (
                        url,
                        if user.cache
                            && cacher.key_newer_than(&url, configuration.caching.min_age)
                                == Some(true)
                        {
                            configuration.get_cache_for_user(user, &url)
                        } else {
                            match fetch(&url) {
                                FetchResult::Success(content) => {
                                    configuration.set_cache_for_user(user, &url, &content);
                                    content
                                }
                                FetchResult::TransientError => {
                                    configuration.get_cache_for_user(user, &url)
                                }
                                FetchResult::PermanentError => {
                                    configuration.set_cache_for_user(user, &url, "");
                                    String::from("")
                                }
                            }
                        },
                    )
                }
            })
            .map(|(url, content)| format!("# {}\n{}", url, content))
            .collect::<Vec<_>>()
            .join("\n");

        println!("{}", results);
    }
}
