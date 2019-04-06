use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn cli_explains_issue_when_run_with_no_subcommand() {
    Command::cargo_bin("hubauth")
        .unwrap()
        .assert()
        .code(2)
        .stderr(predicate::str::contains("error: a subcommand was expected"));
}
