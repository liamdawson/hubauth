use super::get_config;
use crate::cli::model::FetchOpts;
use hubauth::fetch::{get_para, Outcome};

pub fn call(opts: FetchOpts) {
    let configuration = get_config(&opts.config, opts.cache_dir);

    if let Some(user) = configuration.users.get(&opts.username) {
        let results = get_para(user.source_urls_refs())
            .into_iter()
            .map(|(url, res)| {
                format!(
                    "# {}\n{}",
                    url,
                    match res {
                        Outcome::Success(val) => val,
                        Outcome::TransientError => "# a transient error occurred".to_string(),
                        Outcome::PermanentError => "# a permanent error occurred".to_string(),
                    }
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        println!("# keys for {}:\n\n{}", &opts.username, results);
    } else {
        eprintln!("# no user {}", &opts.username);
    }
}
