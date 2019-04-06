mod cli_helper;
use cli_helper::*;
use std::io::Write;

fn single_user_config(username: &str, key_source: &str) -> Vec<u8> {
    (format!(
        "---\nusers:\n  {}:\n    key_sources:\n    - {}\n",
        username, key_source
    ))
    .as_bytes()
    .to_vec()
}

#[test]
pub fn cli_loads_from_http_source() {
    let key_mock = mockito::mock("GET", "/user.keys")
        .expect(1)
        .with_body(EXAMPLE_SSH_KEY)
        .create();

    let mock_url = format!("{}/user.keys", mockito::server_url());
    let mock_key_source = format!("url: {}", mock_url);
    let config_bytes = single_user_config("test", &mock_key_source);
    let config_file = tempfile::Builder::default()
        .suffix(".yml")
        .tempfile()
        .expect("could not create temporary config file for test");
    config_file
        .as_file()
        .write_all(config_bytes.as_slice())
        .expect("could not write to temporary config file for test");

    let path = config_file.path();

    subject()
        .args(&[
            "fetch",
            "--config",
            &path
                .to_str()
                .expect("invalid config path for temp variable"),
            "test",
        ])
        .assert()
        .code(0)
        .stdout(predicates::str::similar(format!(
            "# keys for {}:\n\n# {}\n{}\n",
            "test", mock_url, EXAMPLE_SSH_KEY
        )));

    key_mock.assert();
}

#[test]
pub fn cli_loads_from_valid_tls_source() {
    // TODO
}

#[test]
pub fn cli_fails_to_load_from_invalid_tls_source() {
    // TODO
}
