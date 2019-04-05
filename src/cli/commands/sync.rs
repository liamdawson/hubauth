use crate::cli::SyncOpts;
use crate::cli::commands::get_config;
use hubauth::fetch::{fetch_parallel, FetchResult};

pub fn call(opts: SyncOpts) {
    let configuration = get_config(opts.config, opts.cache_dir);
    let cacher = configuration.get_cacher();

    let mut cached_sources: Vec<_> = configuration
        .users
        .iter()
        .filter(|(_, user)| user.cache)
        .flat_map(|(_, user)| user.source_urls_refs())
        .collect();

    cached_sources.sort();
    cached_sources.dedup();

    fetch_parallel(cached_sources)
        .into_iter()
        .for_each(|(url, res)| match res {
            FetchResult::Success(content) => {
                cacher.set_lossy(url, &content);
            }
            FetchResult::TransientError => {}
            FetchResult::PermanentError => {
                cacher.set_lossy(url, "# a non-transient error occurred");
            }
        });
}
