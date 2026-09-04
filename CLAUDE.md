# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Project Is

`pypi-jump-to` (binary: `pjt`) is a small Rust CLI that resolves and opens the right URL for a PyPI package in the browser — PyPI homepage, GitHub repo, docs, changelog, issues, pulls, releases, tags, or version history. Despite being pure Rust, it is distributed **on PyPI**: maturin builds the binary into a wheel (`[tool.maturin] bindings = "bin"` in `pyproject.toml`), installed via `uv tool install pypi-jump-to` / `pipx install pypi-jump-to` / `pip install pypi-jump-to`.

## Commands

Everything goes through the `justfile`; CI runs these in order install → lint → audit → test → build:

```shell
just install   # cargo fetch
just format    # cargo fmt + cargo clippy --fix --allow-dirty
just lint      # cargo fmt --check + cargo clippy -- -D warnings
just audit     # cargo audit (requires cargo-audit installed)
just test      # cargo test --verbose --workspace --all-targets
just build     # cargo build --release --verbose --workspace --all-targets
just check     # lint + test + build
just update    # cargo update
```

Gotcha: `just test` skips entirely (prints a message) if a `.no-tests` sentinel file exists in the repo root. Make sure that file is absent when you actually want tests to run.

Running subsets / the CLI itself:

```shell
cargo test --test metadata_tests                                        # one test file
cargo test --test metadata_tests test_extract_github_url_with_source_key  # one test
cargo test -- --ignored                                                 # live-network PyPI tests
cargo run -- httpx d                                                    # run the CLI locally
```

## Architecture

Flow: `handlers::args` (clap parse) → `commands::jump::execute`/`build_url` (dispatch) → `handlers::metadata` (PyPI fetch + URL extraction) → `open::that(url)`. `src/main.rs` is a 12-line entrypoint that prints a red `Error:` and exits 1 on failure; `src/lib.rs` exposes `commands` and `handlers` publicly so the `tests/` integration crates can reach the URL logic without spawning the binary.

- **`src/handlers/args.rs`** — `JumpCommand` (clap derive) with positional `package_name` and a `Destination` value-enum defaulting to `Homepage`. Each variant carries a single-letter `#[value(alias = ...)]` (`c d g h i p r t v`); those aliases are the real public CLI surface documented in the README. Help text is styled at runtime with `console` (per-variant `help = format!(...)`, `build_examples_section` as `after_help`), so `--help` output contains ANSI codes.
- **`src/handlers/metadata.rs`** — the PyPI JSON API client (`ureq` with rustls, blocking — no async runtime) plus all extraction logic. `fetch_pypi_metadata` hits `https://pypi.org/pypi/<pkg>/json` and deserializes only `info.project_urls` and `info.home_page`. Error classification string-matches `ureq`'s error text (`"404"` → package not found, `"http status:"` → API error, else connection error).
- **`src/commands/jump.rs`** — maps `Destination` to a URL. `Homepage` and `Versions` are built from the package name alone with **no network call**; all other destinations fetch metadata once, then dispatch in an inner `match` ending in `unreachable!()`. A new destination that needs no network must be handled in the *outer* match arm, not the inner one.

Extraction rules to know before changing them:

- `extract_github_url`: `project_urls["Source"]` wins unconditionally (no github.com check); fallback keys `Repository` / `Source Code` are used only when the value contains `github.com`.
- `extract_documentation_url` / `extract_changelog_url` walk ordered key lists (`Documentation, Docs, Document`; `Changelog, Change Log, Changes, History, Release Notes`) via `extract_url_by_keys`. PyPI metadata key naming varies wildly across packages — fix coverage gaps by extending these lists, not adding special cases.
- GitHub sub-pages (`issues`, `pulls`, `releases`, `tags`) all go through `extract_github_path_url`, which trims a trailing `.git` and `/` from the repo URL before appending the path.

## Tests

All tests live in `tests/` as integration crates (no `#[cfg(test)]` modules in `src/`), importing `pypi_jump_to::handlers::*`:

- `tests/metadata_tests.rs` — extractor behavior against hand-built `PypiResponse` fixtures
- `tests/integration_tests.rs` — URL building end-to-end (everything except the browser open)
- `tests/pypi_api_tests.rs` — URL construction plus two `#[ignore]`d tests that hit the live PyPI API; excluded from CI's `just test`

`mockito` and `tokio-test` sit in `[dev-dependencies]` but are currently unused by any test.

## Conventions

- Rust edition **2024**; `metadata.rs` uses let-chains (`if let Some(url) = ... && url.contains(...)`), which require it.
- Every `.rs` file opens with a `//!` doc comment: leaf modules and test files start `Module that contains ...`, `lib.rs` and each `mod.rs` start `Package that contains ...`.
- `.editorconfig`: 4-space indent for `.rs`, 2-space elsewhere, max line length 120, LF endings.
- Conventional Commits are load-bearing: `cliff.toml` (`filter_unconventional = true`) generates the changelog and `git-cliff --bumped-version` computes the next release version from commit types (`feat` → minor, breaking → major). Branch names: `<type>/<kebab-description>`. Full type table in `CONTRIBUTING.md`.

## Versioning & Release

The version lives in **`Cargo.toml` only**. `pyproject.toml` declares `dynamic = ["version"]` and maturin reads it from the crate — never add a Python-side version field.

`.github/workflows/release.yaml` is `workflow_dispatch`-only and does everything: resolve version via `git-cliff --bumped-version` (or the manual `version` input) → `sed` it into `Cargo.toml` → `cargo update --workspace` → regenerate `CHANGELOG.md` → commit `release: vX.Y.Z` + tag + push to main → GitHub Release with the changelog body → maturin wheels (linux x86_64/aarch64 manylinux 2_28, macOS x86_64/aarch64, windows x64) + sdist → `uv publish --trusted-publishing always` (PyPI OIDC, no token secret).

CI (`.github/workflows/ci.yaml`) is a single job on `ubuntu-24.04-arm` running `just install`, `just lint`, `just audit`, `just test`, `just build` — all must pass on PRs and pushes to main.
