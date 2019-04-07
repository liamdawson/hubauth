use chttp::{Client, Error, Options, RedirectPolicy, Response};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use retry::retry;
use std::time::Duration;

const TIMEOUT_LENGTH: u64 = 2500_u64;
const MS_BETWEEN_RETRIES: u64 = 500;
const MAX_RETRIES: u64 = 2;
const PERMANENT_ERROR_CODES: &[u16] = &[401, 403, 404, 405, 406, 410, 451];

#[derive(PartialEq, Clone, Debug)]
pub enum Outcome {
    Success(String),
    TransientError,
    PermanentError,
}

fn error_response_type(result: &Result<Response, Error>) -> Option<Outcome> {
    if let Ok(response) = result {
        if response.status().is_success() {
            return None;
        } else if PERMANENT_ERROR_CODES.contains(&response.status().as_u16()) {
            return Some(Outcome::PermanentError);
        }
    }

    Some(Outcome::TransientError)
}

fn try_fetch(url: &str, timeout: u64) -> Result<Response, Error> {
    let mut options = Options::default();

    options.timeout = Some(Duration::from_millis(timeout));
    options.redirect_policy = RedirectPolicy::Limit(3);

    let client = Client::builder().options(options).build()?;

    client.get(url)
}

pub fn get(url: &str) -> Outcome {
    let retry_result = retry(
        MAX_RETRIES,
        MS_BETWEEN_RETRIES,
        || try_fetch(url, TIMEOUT_LENGTH),
        |res| error_response_type(res) != Some(Outcome::TransientError),
    );

    if retry_result.is_err() {
        return Outcome::TransientError;
    }

    // retry condition should make the second unwrap safe
    let mut request_result = retry_result.unwrap().unwrap();

    if request_result.status().is_success() {
        if let Ok(response_text) = request_result.body_mut().text() {
            return Outcome::Success(response_text);
        }
    }

    // treat a decoding error as a permanent error, to be safe
    Outcome::PermanentError
}

pub fn get_para<'a>(urls: Vec<&'a str>) -> Vec<(&'a str, Outcome)> {
    urls.into_par_iter().map(|url| (url, get(url))).collect()
}

#[cfg(test)]
mod tests {
    use super::{get, Outcome};
    use mockito;

    #[test]
    fn it_returns_body() {
        let test_string = "I'm a happy little vegemite!";

        let url = &mockito::server_url();
        let request_mock = mockito::mock("GET", "/")
            .with_status(200)
            .with_body(test_string)
            .create();

        assert_eq!(Outcome::Success(String::from(test_string)), get(url));

        request_mock.assert();
    }

    #[test]
    fn it_follows_redirects() {
        let test_string = "Strong as I can be!";

        let url = &mockito::server_url();
        let redirect_mock = mockito::mock("GET", "/")
            .with_status(302)
            .with_header("Location", "/real")
            .create();

        let request_mock = mockito::mock("GET", "/real")
            .with_status(200)
            .with_body(test_string)
            .create();

        assert_eq!(Outcome::Success(String::from(test_string)), get(url));

        redirect_mock.assert();
        request_mock.assert();
    }

    #[test]
    fn it_tries_twice_before_failing() {
        let url = &mockito::server_url();
        let request_mock = mockito::mock("GET", "/")
            .with_status(500)
            .expect(2)
            .create();

        assert_eq!(Outcome::TransientError, get(url));

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

            assert_eq!(Outcome::PermanentError, get(url));

            request_mock.assert();
        }
    }
}
