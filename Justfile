test-cli:
    cargo test cli -- --test-threads=8

prepare-pr:
    cargo test
    cargo fmt
    cargo clippy -- -D clippy::pedantic -D clippy::nursery
