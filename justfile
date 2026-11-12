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

audit:
    cargo audit
    uv audit --preview-features audit-command

test:
    @[ -f .no-tests ] && echo "skipping (.no-tests sentinel)" || cargo test --workspace

check: lint test build

update:
    cargo update

build:
    cargo build --release --workspace
