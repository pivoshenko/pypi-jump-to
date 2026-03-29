format:
  cargo fmt --all

lint:
  cargo fmt --all -- --check
  cargo clippy -- -D warnings
  cargo check --verbose --workspace --all-targets

test:
  cargo test --verbose --workspace --all-targets

update:
  cargo update
