# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Project Is

`pypi-jump-to` (binary: `pjt`) is a small Rust CLI that resolves and opens the right URL for a PyPI package — PyPI homepage, GitHub repo, docs, changelog, issues, pulls, releases, tags, or version history. It is distributed **on PyPI** as a maturin-built wheel wrapping the Rust binary (`[tool.maturin] bindings = "bin"`), installed with `uv tool install pypi-jump-to` / `pipx install pypi-jump-to`.

## Commands

Everything goes through the `justfile` (this is what CI runs, in this order):

```shell
just install   # cargo fetch
just format    # cargo fmt + cargo clippy --fix --allow-dirty
just lint      # cargo fmt --check + cargo clippy -- -D warnings
just audit     # cargo audit (needs: cargo install cargo-audit)
just test      # cargo test --verbose --workspace --all-targets
just build     # cargo build --release --verbose --workspace --all-targets
just check     # lint + test + build
just update    # cargo update
```

`just test` short-circuits if a `.no-tests` sentinel file exists in the repo root — it prints a skip message instead of running cargo. Delete/avoid that file when you actually want tests to run.

Running a subset:

```shell
cargo test --test metadata_tests                    # one test file
cargo test --test metadata_tests test_extract_github_url_with_source_key
cargo test -- --ignored                             # the network tests (see below)
cargo run -- httpx d                                # run the CLI locally
```

## Architecture

`main.rs` is a 12-line entrypoint: parse args, call `commands::jump::execute`, print a red `Error:` and `exit(1)` on failure. `lib.rs` exposes `commands` and `handlers` publicly (and re-exports `handlers::*`) purely so the `tests/` integration crates can reach the URL logic.

Flow: `handlers::args` (parse) → `commands::jump::build_url` (dispatch) → `handlers::metadata` (fetch + extract) → `open::that(url)`.

- **`src/handlers/args.rs`** — clap derive `JumpCommand` + `Destination` enum. Each variant declares a single-letter `#[value(alias = ...)]` (`c d g h i p r t v`); those aliases are the real public CLI surface documented in the README. Help text is styled with `console` at runtime (`build_examples_section`, per-variant `help = format!(...)`), so help output contains ANSI codes.
- **`src/handlers/metadata.rs`** — the PyPI JSON API client (`ureq` with rustls, blocking, no async runtime) and all URL extraction. `fetch_pypi_metadata` hits `https://pypi.org/pypi/<pkg>/json` and only deserializes `info.project_urls` and `info.home_page`. Error mapping is string-matching on `ureq`'s error text (`"404"`, `"http status:"`) to produce "not found on PyPI" vs. generic API vs. connection errors.
- **`src/commands/jump.rs`** — maps `Destination` to a URL. `Homepage` and `Versions` are built from the package name alone (**no network call**); every other destination fetches metadata once, then dispatches. The inner `match` ends in `unreachable!()` because those two are handled in the outer arm — adding a destination that needs no network means handling it in the *outer* match, not the inner one.

Extraction rules worth knowing before changing them:

- `extract_github_url` returns `project_urls["Source"]` unconditionally (no github.com check), then falls back to `Repository` / `Source Code` **only if** the value contains `github.com`.
- `extract_documentation_url` / `extract_changelog_url` walk ordered key lists (`Documentation, Docs, Document`; `Changelog, Change Log, Changes, History, Release Notes`) via `extract_url_by_keys`. PyPI key naming varies wildly across packages — fix coverage gaps by extending these key lists, not by adding per-destination special cases.
- GitHub sub-pages (`issues`, `pulls`, `releases`, `tags`) are all `extract_github_path_url`, which trims a trailing `.git` and `/` from the repo URL before appending the path.

## Tests

Three integration test files in `tests/` (no inline `#[cfg(test)]` modules in `src/`), all exercising the library through `pypi_jump_to::handlers::*`:

- `metadata_tests.rs` — extractor behavior against hand-built `PypiResponse` fixtures
- `integration_tests.rs` — URL building end-to-end (minus the browser open)
- `pypi_api_tests.rs` — URL construction plus two `#[ignore]`d tests that hit the live PyPI API; they do not run in CI's `just test`

`mockito` and `tokio-test` are declared in `[dev-dependencies]` but currently unused by any test.

## Conventions

- Rust edition **2024**; `metadata.rs` uses let-chains (`if let Some(url) = ... && url.contains(...)`), which require it.
- Every `.rs` file opens with a `//!` doc comment. Leaf modules and test files start `Module that contains ...`; `lib.rs` and each `mod.rs` start `Package that contains ...`.
- `.editorconfig`: 4-space indent for `.rs`, 2-space elsewhere, max line length 120.
- Conventional Commits are enforced by the changelog tooling (`cliff.toml` sets `filter_unconventional = true`). Branches: `<type>/<kebab-description>`. See `CONTRIBUTING.md` for the full type table.

## Versioning & Release

The version lives in **`Cargo.toml` only**. `pyproject.toml` declares `dynamic = ["version"]` and maturin reads it from the crate — do not add a Python-side version field.

`.github/workflows/release.yaml` is `workflow_dispatch`-only and does the whole thing: `git-cliff --bumped-version` (or a manual override input) → `sed` the version into `Cargo.toml` → `cargo update --workspace` → regenerate `CHANGELOG.md` → commit `release: vX.Y.Z`, tag, push → GitHub Release → maturin wheels (linux x86_64/aarch64, macOS x86_64/aarch64, windows x64) + sdist → `uv publish` via PyPI trusted publishing (OIDC, no token secret).

CI (`.github/workflows/ci.yaml`) is a single job on `ubuntu-24.04-arm` running `just install/lint/audit/test/build`.
