# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`pypi-jump-to` (binary name: `pjt`) is a CLI tool written in Rust that quickly opens PyPI package-related URLs in the browser. It fetches package metadata from the PyPI JSON API (`https://pypi.org/pypi/<package>/json`) and resolves destination URLs (GitHub repo, docs, changelog, issues, etc.) from the `project_urls` field. Distributed on PyPI as a binary wheel built with [maturin](https://github.com/PyO3/maturin).

## Build & Dev Commands

Uses `just` as task runner (see `justfile`):

```sh
just format        # cargo fmt --all
just lint          # fmt check + clippy + cargo check
just test          # cargo test --verbose --workspace --all-targets
just update        # cargo update
```

Run a single test:
```sh
cargo test <test_name> -- --exact
```

Tests marked `#[ignore]` in `tests/pypi_api_tests.rs` hit the real PyPI API. Run them explicitly:
```sh
cargo test -- --ignored
```

Rust edition is 2024 (requires Rust nightly or recent stable).

## Architecture

Two modules under `src/`:

- **`handlers`** — CLI argument parsing (`args.rs` via clap derive) and PyPI metadata fetching/extraction (`metadata.rs` via ureq + serde). The `Destination` enum defines all jump targets with single-letter aliases.
- **`commands`** — `jump.rs` orchestrates: maps `(package, destination)` → URL, then opens it with `open::that`. Homepage and Versions URLs are built locally; all other destinations require a PyPI API call.

`main.rs` is a thin entry point that parses args and calls `commands::jump::execute`.

Tests live in `tests/` (not inside `src/`): `integration_tests.rs`, `metadata_tests.rs`, `pypi_api_tests.rs`.

## Conventions

- Commits follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) (`feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`, `build`, `perf`, `style`, `revert`).
- GitHub URL extraction priority: `Source` key first, then `Repository`/`Source Code` (only if they contain `github.com`).
