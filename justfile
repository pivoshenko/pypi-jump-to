default:
    @just --list

install: install-package

install-package:
    cargo fetch

format: format-package

format-package:
    cargo clippy --fix --allow-dirty --allow-staged --workspace --all-targets
    cargo fmt --all

lint: lint-package

lint-package:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings

audit: audit-package

audit-package:
    cargo audit
    uv audit --preview-features audit-command

test: test-package

test-package:
    @[ -f .no-tests ] && echo "skipping (.no-tests sentinel)" || cargo test --workspace

check: lint test build

update: update-package

update-package:
    cargo update

build: build-package

build-package:
    cargo build --release --workspace
