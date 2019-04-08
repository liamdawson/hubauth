use crate::cli::commands::get_config;
use crate::cli::SyncOpts;
use hubauth::fetch::{get_para, Outcome};

pub fn call(opts: SyncOpts) {
    let configuration = get_config(&opts.config, opts.cache_dir);
    let cacher = configuration.get_cacher();

    let mut cached_sources: Vec<_> = configuration
        .users
        .iter()
        .flat_map(|(_, user)| {
            if user.cache {
                user.source_urls_refs()
            } else {
                vec![]
            }
        })
        .collect();

    cached_sources.sort();
    cached_sources.dedup();

    get_para(cached_sources)
        .into_iter()
        .for_each(|(url, res)| match res {
            Outcome::Success(content) => {
                cacher.set_lossy(url, &content);
            }
            Outcome::TransientError => {}
            Outcome::PermanentError => {
                cacher.set_lossy(url, "# a non-transient error occurred");
            }
        });
}
