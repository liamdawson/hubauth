#![allow(dead_code)]

pub use assert_cmd::prelude::*;
pub use predicates::prelude::*;
pub use std::process::Command;

pub const PKG_NAME: &str = "hubauth";
pub const EXAMPLE_SSH_KEY: &str = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQCve8gcxEQKaue8XynCD8rG4KowiW3RGFJzOWhAvCi+WTRlujUE8zRLBBtsXcHCC85+N6g+ZjMLJql1JIhCncov7Lp5jBAiBi847RegvbALeux774PHGpc6A+xPMpEDbfMWjVXwbPG/B/A+hA1G/wMEYmfzkRi4DUyCS7wPBNWM9510WxNYZzjk0zA3o+/ezCuaeX4xzBFXkX84Z3J8bI79yuRBhqSm0MVGyh2R+w75YZbaSGhqgeWXhKtV0ZtVKOP6/nDaJ4kx2f2RguqF/E2yp/liyiggDGz53kGfJ5nizsr6SwB1qIh85m/rEiYmWib+ZrFBc1KyefV9Tpztc1dr0RcVRuXAfb+nxAVuZbDTL1A2nY9+g0byEVX6jm6uEaJ2yascaqyw0NXjhTsXR9v4H50z4wMp0l1vtCXHF3dawKdMIHjia+feFmopT8QZJm5omK3xemTqwRrdiWp6IdADwN3q1nqg1wKm5D702hzAchRJ6BOMINRBtkMn+2mQLjbmKiXoEm6Yxq1RKCG7w89wrm9tGdxrikJ/dxcdWQt2gY9YubrjsW3BpLqA+Y73KX3dv3STwKgr2kEscZO1OxE2kqHYYCGBWWR6BmXLtQ2FqRtehTRQD2QR0aIF1l/7S06HqfwC4KExV1bGyIiq9JCfAzp3KfS/H4VeRVln7WhduQ==";

pub fn subject() -> Command {
    Command::cargo_bin(PKG_NAME).unwrap()
}

pub fn fixture(subdir: &str, name: &str) -> std::path::PathBuf {
    std::env::current_dir()
        .expect("could not obtain working directory")
        .join("tests")
        .join("fixtures")
        .join(subdir)
        .join(name)
}

pub fn fixture_str(subdir: &str, name: &str) -> String {
    std::fs::read_to_string(fixture(subdir, name)).expect("could not read fixture file")
}

pub fn matches_fixture(subdir: &str, name: &str) -> predicates::str::DifferencePredicate {
    predicates::str::similar(fixture_str(subdir, name))
}

pub fn ends_fixture(subdir: &str, name: &str) -> predicates::str::EndsWithPredicate {
    predicates::str::ends_with(fixture_str(subdir, name))
}
