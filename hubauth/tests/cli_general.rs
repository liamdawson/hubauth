mod cli_helper;
use cli_helper::*;

#[test]
fn cli_explains_issue_when_run_with_no_subcommand() {
    subject()
        .assert()
        .code(2)
        .stderr(predicate::str::contains("error: a subcommand was expected"));
}

#[test]
fn cli_displays_help_when_requested() {
    subject()
        .arg("--help")
        .assert()
        .code(0)
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn cli_displays_version_when_requested() {
    subject()
        .arg("--version")
        .assert()
        .code(0)
        .stdout(predicate::str::contains(PKG_NAME))
        .stdout(predicate::str::is_match(r"\d+.\d+.\d+").unwrap());
}
