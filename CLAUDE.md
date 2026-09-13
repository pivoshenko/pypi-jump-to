# CLAUDE.md

Guidance for Claude Code when working in this repository. `AGENTS.md` is a symlink to this file.

`pypi-jump-to` is a Rust CLI that resolves a PyPI package name plus a one-letter destination into a URL and opens it in the default browser. The binary is named `pjt`, the crate and PyPI distribution are both named `pypi-jump-to`. `README.md` has the destination list and usage examples.

## Expensive To Get Wrong

- `build_url` in `src/commands/jump.rs` has two match layers. `Homepage` and `Versions` are resolved from the package name alone and are matched in the **outer** `match`, before any network call; every other destination falls into the `_` arm, which fetches PyPI metadata once and then re-matches on the same value in an inner `match` ending in `unreachable!()`. Putting a metadata-free destination in the inner match costs a pointless HTTP round trip; forgetting the arm entirely hits `unreachable!()` and panics at runtime rather than failing to compile
- `ureq` is pinned to `rustls` with default features off - do not reintroduce default features or a native-TLS backend, it is what keeps the binary dependency-free across the wheel targets
- never edit `version` in `Cargo.toml` by hand - the `Release` workflow owns it, and `pyproject.toml` declares `dynamic = ["version"]` so the Python package version follows automatically
- HTTP failures are classified by substring-matching the `ureq` error text for `"404"` and `"http status:"`. Any change to `ureq`'s `Display` output silently degrades the "Package not found on PyPI" message into the generic connection error

## Project Shape

It is a Rust project shipped through PyPI: `maturin` with `bindings = "bin"` packages the compiled `pjt` binary into wheels, so `pyproject.toml` describes packaging only and there is no Python source anywhere in the repo. `uv.lock` exists solely because the project is its own editable root, and it has no Python dependencies to lock.

Edition 2024 - let-chains are used in `src/handlers/metadata.rs`, so a recent stable toolchain is required. There is no async runtime: `ureq` is blocking and the whole program is a single synchronous pass.

Everything goes through `just`; `CONTRIBUTING.md` has the recipe table. Two things it does not cover: the network-dependent tests are `#[ignore]`d and run only under `cargo test -- --ignored`, and `cargo run -- httpx g` is the quickest way to try the CLI locally.

## Architecture

Two crates in one package: `src/lib.rs` (library, `pypi_jump_to`) and `src/main.rs` (the `pjt` binary, declared as `[[bin]]`). `main.rs` only parses args, calls `commands::jump::execute`, and prints a red `Error:` prefix to stderr with exit code 1 on failure. Everything testable lives in the library, because the files in `tests/` are integration tests that link against it - anything a test touches must be `pub`.

Module layout:

- `src/handlers/args.rs` - the `Destination` enum (a `clap::ValueEnum` where each variant carries its single-letter alias and a `console`-styled help string) and `JumpCommand`, the two-positional-argument parser. Help output is styled via a custom `clap::builder::Styles` plus an `after_help` examples block built in `build_examples_section`
- `src/handlers/metadata.rs` - fetches `https://pypi.org/pypi/<package>/json` and extracts URLs out of `info.project_urls`. `Documentation` and `Changelog` get dedicated extractors that walk an ordered list of candidate keys and error when none match; `Github` has its own `extract_github_url`, and `Issues`/`PullRequests`/`Releases`/`Tags` all share `extract_github_path_url`
- `src/commands/jump.rs` - `build_url` maps a `Destination` to a URL, then `open::that` hands it to the OS

`extract_github_url` trusts a `Source` project URL without checking that it points at github.com, so `Github` happily opens a GitLab `Source`. `extract_github_path_url` re-checks the domain itself and errors with `No GitHub repository found` for anything that is not github.com, so `Issues`/`PullRequests`/`Releases`/`Tags` never emit nonsense paths like `https://gitlab.com/x/y/pulls`.

## Adding a Destination

Adding a variant means touching every one of these, in order:

1. `Destination` in `src/handlers/args.rs` - variant, `#[value(alias = "x")]`, and a styled `#[value(help = ...)]` matching the surrounding format
2. an extractor in `src/handlers/metadata.rs`, unless it reuses `extract_github_path_url`
3. `build_url` in `src/commands/jump.rs` - the inner match if the destination needs PyPI metadata, **the outer match if it does not** (see above for what each mistake costs)
4. the `Available Destinations` list in `README.md`
5. tests in `tests/`

## Tests

Three integration test files, no unit tests inside `src/`:

- `tests/metadata_tests.rs` - per-extractor coverage of key precedence, missing keys, and `.git`/trailing-slash trimming
- `tests/integration_tests.rs` - the public `metadata` extractors called directly against a hand-built `PypiResponse`, plus edge cases
- `tests/pypi_api_tests.rs` - deserialization of PyPI payload shapes, error-message formats, and the two `#[ignore]`d tests that actually hit pypi.org

Each file groups its cases into `#[cfg(test)] mod <topic>_tests { ... }` blocks and builds `PypiResponse` values by hand - there is no HTTP mocking layer, so anything below `fetch_pypi_metadata` is tested against literal structs.

## Conventions

- every module opens with a `//!` doc comment in the form `//! Module that contains ...` (or `//! Package that contains ...` for a `mod.rs`)
- in the library modules (`src/handlers/args.rs`, `src/handlers/metadata.rs`, `src/commands/jump.rs`) imports are written as bare module paths and called fully qualified (`handlers::metadata::fetch_pypi_metadata`, `clap::ValueEnum`, `console::style`) rather than importing individual items - match that when editing those files; `src/main.rs` and everything in `tests/` import individual items instead
- error handling is `Result<T, Box<dyn std::error::Error>>` with human-readable string errors throughout; `metadata.rs` defines a local `type Result<T>` alias for it
- `cliff.toml` sets `filter_unconventional = true`, so a commit that does not follow Conventional Commits is silently dropped from the changelog instead of being rejected
