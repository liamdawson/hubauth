use rayon::iter::{IntoParallelIterator, ParallelIterator};
use retry::retry;
use mio_httpc::{CallBuilder, Result, Response};

const TIMEOUT_LENGTH: u64 = 2500u64;
const MS_BETWEEN_RETRIES: u64 = 500;
const MAX_RETRIES: u64 = 2;
const PERMANENT_ERROR_CODES: &[u16] = &[401, 403, 404, 405, 406, 410, 451];

#[derive(PartialEq, Clone, Debug)]
pub enum FetchResult {
    Success(String),
    TransientError,
    PermanentError,
}

fn success_status(status: u16) -> bool {
    status >= 200 && status < 300
}

fn error_response_type(result: &Result<(Response, Vec<u8>)>) -> Option<FetchResult> {
    if let Ok((response, _)) = result {
        if success_status(response.status) {
            return None;
        } else if PERMANENT_ERROR_CODES.contains(&response.status) {
            return Some(FetchResult::PermanentError);
        }
    }

    Some(FetchResult::TransientError)
}

fn try_fetch(url: &str, timeout: u64) -> Result<(Response, Vec<u8>)> {
    CallBuilder::get()
        .timeout_ms(timeout)
        .url(url)?
        .exec()
}

pub fn fetch(url: &str) -> FetchResult {
    let retry_result = retry(
        MAX_RETRIES,
        MS_BETWEEN_RETRIES,
        || try_fetch(url, TIMEOUT_LENGTH),
        |result| error_response_type(&result) != Some(FetchResult::TransientError),
    );

    if retry_result.is_err() {
        return FetchResult::TransientError;
    }

    // retry condition should make the second unwrap safe
    let (response, raw_body) = retry_result.unwrap().unwrap();

    if success_status(response.status) {
        if let Ok(response_text) = String::from_utf8(raw_body) {
            return FetchResult::Success(response_text);
        }
    }

    // treat a decoding error as a permanent error, to be safe
    FetchResult::PermanentError
}

pub fn fetch_parallel<'a>(urls: Vec<&'a str>) -> Vec<(&'a str, FetchResult)> {
    urls.into_par_iter().map(|url| (url, fetch(url))).collect()
}

#[cfg(test)]
mod tests {
    use super::{fetch, FetchResult};
    use mockito;

    #[test]
    fn it_returns_body() {
        let test_string = "I'm a happy little vegemite!";

        let url = &mockito::server_url();
        let request_mock = mockito::mock("GET", "/")
            .with_status(200)
            .with_body(test_string)
            .create();

        assert_eq!(FetchResult::Success(String::from(test_string)), fetch(url));

        request_mock.assert();
    }

    #[test]
    fn it_tries_twice_before_failing() {
        let url = &mockito::server_url();
        let request_mock = mockito::mock("GET", "/")
            .with_status(500)
            .expect(2)
            .create();

        assert_eq!(FetchResult::TransientError, fetch(url));

        request_mock.assert();
    }

    #[test]
    fn it_respects_the_list_of_permanent_errors() {
        let url = &mockito::server_url();
        for error_code in super::PERMANENT_ERROR_CODES {
            let request_mock = mockito::mock("GET", "/")
                .with_status(usize::from(*error_code))
                .with_body("This is a permanent error.")
                .expect(1)
                .create();

            assert_eq!(FetchResult::PermanentError, fetch(url));

            request_mock.assert();
        }
    }

    // TODO: test to ensure proper paralellism, when possible?
    // see: https://github.com/lipanski/mockito/issues/64#issuecomment-474682193
    //
    // though it should be fine if nothing weird is going on:
    // https://github.com/rayon-rs/rayon/issues/551
}
