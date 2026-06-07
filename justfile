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
    @[ -f .no-tests ] && echo "skipping (.no-tests sentinel)" || cargo test --verbose --workspace --all-targets

build:
    cargo build --release --verbose --workspace --all-targets

check: lint test build

audit:
    cargo audit

update:
    cargo update
