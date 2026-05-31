# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this project is

`pypi-jump-to` (binary: `pjt`) is a tiny Rust CLI that opens the right URL for a PyPI package — homepage, GitHub repo, docs, changelog, issues, pulls, releases, tags, or version history — without manual URL construction. It ships as a PyPI package built by **maturin** (Rust binary wrapped in a wheel), installable via `uv tool install pypi-jump-to` / `pipx install pypi-jump-to`.

## Commands

All workflow lives in the `justfile`:

- `just install` — `cargo fetch`
- `just format` — `cargo fmt --all`
- `just lint` — `cargo fmt --check`, `cargo clippy -D warnings`, `cargo check --workspace --all-targets`
- `just test` — `cargo test --workspace --all-targets`
- `just build` — `cargo build --release --workspace --all-targets`
- `just audit` — `cargo audit` (requires `cargo install cargo-audit`)
- `just update` — `cargo update`

Single test: `cargo test --test integration_tests -- <name>` (test files live in `tests/`, not inline modules).

## Architecture

Two-module split under `src/`:

- **`handlers/args.rs`** — `clap` derive parser (`JumpCommand`) and the `Destination` enum. Each enum variant carries a single-letter `alias` (`c`/`d`/`g`/`h`/`i`/`p`/`r`/`t`/`v`) — this is the public CLI surface; adding a destination means adding a variant *and* wiring it in both `commands/jump.rs` and `handlers/metadata.rs`.
- **`handlers/metadata.rs`** — PyPI JSON API client (`ureq`, rustls) plus URL extractors. `fetch_pypi_metadata` hits `https://pypi.org/pypi/<pkg>/json`; the `extract_*` helpers walk `info.project_urls` looking for known keys (e.g. `Documentation`/`Docs`, `Changelog`/`History`/`Release Notes`, `Source`/`Repository`). When PyPI's `project_urls` keys vary across packages, extend the key list here rather than per-destination.
- **`commands/jump.rs`** — orchestrator: maps `Destination` → URL, then `open::that(url)` to launch the browser. `Homepage` and `Versions` are computed from the package name alone (no network); everything else goes through `fetch_pypi_metadata` once and then dispatches. GitHub sub-pages (`issues`, `pulls`, `releases`, `tags`) are built by `extract_github_path_url`, which strips `.git`/trailing `/` from the repo URL before appending the path.
- **`main.rs`** is a 12-line entrypoint that parses args and calls `commands::jump::execute`; **`lib.rs`** re-exports `handlers::*` so integration tests can reach into them.

## Versioning & release

`pyproject.toml` uses `dynamic = ["version"]` with `build-backend = "maturin"` and `[tool.maturin] bindings = "bin"` — **the wheel's version is read from `Cargo.toml`**. There is no Python version field to bump; `sed` `Cargo.toml`, run `cargo update`, commit both `Cargo.toml` and `Cargo.lock`. The release workflow (`.github/workflows/release.yaml`) does exactly this, then `uv build` (which invokes maturin) and `uv publish`.

`cliff.toml` drives changelog generation via `git-cliff --bumped-version` (conventional commits → semver bump).

## Sibling-repo context

This repo sits in `~/Development/sources/` — see the parent `CLAUDE.md` for the full monorepo map. `pypi-jump-to` belongs to the `libs` group; cross-cutting changes across libs are fanned out from the root `justfile` (`just <verb>-libs`). The sibling `kasetto/` is the closest reference for Rust + justfile + release-workflow conventions.
