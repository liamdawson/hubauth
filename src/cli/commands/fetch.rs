use hubauth::fetch::{fetch_parallel, FetchResult};
use hubauth::models::State;

pub fn call(configuration: &State, username: &str) {
    if let Some(user) = configuration.users.get(username) {
        let results = fetch_parallel(user.source_urls_refs())
            .into_iter()
            .map(|(url, res)| {
                format!(
                    "# {}\n{}",
                    url,
                    match res {
                        FetchResult::Success(val) => val,
                        FetchResult::TransientError => "# a transient error occurred".to_string(),
                        FetchResult::PermanentError => "# a permanent error occurred".to_string(),
                    }
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        println!("# keys for {}:\n\n{}", username, results);
    } else {
        eprintln!("# no user {}", username);
    }
}
