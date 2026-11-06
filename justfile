default:
    @just --list

install:
    cargo fetch

format:
    cargo clippy --fix --allow-dirty --allow-staged --workspace --all-targets
    cargo fmt --all

lint:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings

test:
    @[ -f .no-tests ] && echo "skipping (.no-tests sentinel)" || cargo test --workspace

check: lint test build

update:
    cargo update

build:
    cargo build --release --verbose --workspace --all-targets
