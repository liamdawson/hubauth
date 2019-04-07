mod cli_helper;
use cli_helper::*;

fn fixtures_dir() -> std::path::PathBuf {
    std::env::current_dir().expect("could not obtain working directory").join("tests").join("fixtures")
}

fn simple_url_fixture(file: &str) -> String {
    fixtures_dir().join("simple_url_config").join(file).to_str().expect("unable to get fixture path").to_owned()
}

fn matches_file(path: &str) -> predicates::str::DifferencePredicate {
    predicates::str::similar(std::fs::read_to_string(path).expect("could not load file matching fixture"))
}

#[test]
pub fn cli_loads_from_http_source() {
    let key_mock = mockito::mock("GET", "/test_user.keys")
        .expect(1)
        .with_body(EXAMPLE_SSH_KEY)
        .create();

    let config_file_path = simple_url_fixture("hubauth.yml");
    let stdout_fixture = simple_url_fixture("stdout.txt");
    let stderr_fixture = simple_url_fixture("stderr.txt");

    subject()
        .args(&[
            "fetch",
            "--config",
            &config_file_path,
            "test_user",
        ])
        .assert()
        .code(0)
        .stdout(matches_file(&stdout_fixture))
        .stderr(matches_file(&stderr_fixture));

    key_mock.assert();
}
