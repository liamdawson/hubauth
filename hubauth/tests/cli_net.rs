mod cli_helper;
use cli_helper::*;

#[test]
pub fn cli_loads_from_http_source() {
    let key_mock = mockito::mock("GET", "/test_user.keys")
        .expect(1)
        .with_body(EXAMPLE_SSH_KEY)
        .create();

    let config_file_path = fixture("simple_url", "hubauth.yml");

    subject()
        .args(&[
            "fetch",
            "--config",
            &config_file_path.to_str().unwrap(),
            "test_user",
        ])
        .assert()
        .code(0)
        .stdout(matches_fixture("simple_url", "stdout.txt"))
        .stderr(matches_fixture("simple_url", "stderr.txt"));

    key_mock.assert();
}

#[test]
pub fn cli_fetch_validates_username() {
    let config_file_path = fixture("simple_url", "hubauth.yml");

    subject()
        .args(&[
            "fetch",
            "--config",
            &config_file_path.to_str().unwrap(),
            "INVALID_username",
        ])
        .assert()
        .code(2)
        .stdout(matches_fixture("invalid_username", "stdout.txt"))
        .stderr(ends_fixture("invalid_username", "stderr.txt"));
}
