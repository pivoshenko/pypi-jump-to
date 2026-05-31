default:
    @just --list

install:
    cargo fetch

format:
    cargo fmt --all

lint:
    cargo fmt --all -- --check
    cargo clippy -- -D warnings
    cargo check --verbose --workspace --all-targets

test:
    cargo test --verbose --workspace --all-targets

build:
    cargo build --release --verbose --workspace --all-targets

audit:
    cargo audit

update:
    cargo update
