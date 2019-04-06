test-cli:
    cargo test cli -- --test-threads=8

pr: pr-fmt pr-test pr-clippy

pr-test:
    cargo test

pr-fmt:
    cargo fmt

pr-clippy:
    cargo clippy -- -D clippy::pedantic -D clippy::nursery
