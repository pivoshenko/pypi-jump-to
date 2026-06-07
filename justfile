default:
    @just --list

install:
    cargo fetch

format:
    cargo fmt
    cargo clippy --fix --allow-dirty

lint:
    cargo fmt --check
    cargo clippy -- -D warnings

test:
    cargo test --verbose --workspace --all-targets

build:
    cargo build --release --verbose --workspace --all-targets

check: lint test build

audit:
    cargo audit

update:
    cargo update
