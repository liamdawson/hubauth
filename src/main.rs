extern crate hubauth;
extern crate structopt;

mod cli;

use config::{Config, File};
use hubauth::configuration::Configuration;
use hubauth::fetch::{fetch, fetch_parallel, FetchResult};
use hubauth::models::State;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use structopt::StructOpt;

fn main() {
    let args = cli::CommandLineArguments::from_args();

    let mut config = Config::new();
    if let Err(err) = config.merge(File::with_name(args.config_file_path.to_str().unwrap())) {
        eprintln!("error: {:?}", err);
        std::process::exit(2);
    }

    let configuration = match config.try_into::<Configuration>() {
        Ok(configuration) => State::from(configuration),
        Err(err) => {
            eprintln!("error: {:?}", err);
            std::process::exit(3);
        }
    };

    match &args.command {
        cli::Subcommand::Fetch { user_args } => {
            if let Some(user) = configuration.users.get(&user_args.username) {
                let results = fetch_parallel(user.source_urls_refs())
                    .into_iter()
                    .map(|(url, res)| {
                        format!(
                            "# {}\n{}",
                            url,
                            match res {
                                FetchResult::Success(val) => val,
                                FetchResult::TransientError => {
                                    "# a transient error occurred".to_string()
                                }
                                FetchResult::PermanentError => {
                                    "# a permanent error occurred".to_string()
                                }
                            }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n");

                println!("# keys for {}:\n\n{}", &user_args.username, results);
            } else {
                eprintln!("# no user {}", user_args.username);
            }
        }
        cli::Subcommand::Sync => {
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
        cli::Subcommand::Cached { user_args } => {
            if let Some(user) = configuration.users.get(&user_args.username) {
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
                println!("# keys for {}:\n\n{}", user_args.username, result)
            } else {
                println!("# no user {}", user_args.username);
            }
        }
        cli::Subcommand::List { user_args } => {
            let cacher = configuration.get_cacher();

            if let Some(user) = configuration.users.get(&user_args.username) {
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
    }
}
