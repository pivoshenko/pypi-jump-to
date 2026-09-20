# Changelog

All notable changes to this project will be documented in this file.

## [1.1.4] - 2026-09-20

### Bug fixes

- Reject non-github.com source urls for github paths

### Build

- Raise python floor to 3.9 and add 3.14 classifier
- Pin local python version to 3.14
- Update dependencies
- Update dependencies
- Update dependencies
- **deps**: Update dependencies
- **deps**: Update dependencies

### CI/CD

- Pin setup-uv to v10.1.0
- Drop label sync in favor of terraform
- Publish to pypi via trusted publishing instead of a token
- Drop hashFiles guard; move .no-tests sentinel handling into justfile
- Flatten to one job per language
- Bump action versions to latest major
- Standardize workflow to per-language parallel pipelines on ubuntu-24.04-arm

### Documentation

- Use absolute raw url for logo
- Rewrite CLAUDE.md from scratch
- Regenerate CLAUDE.md and add AGENTS.md
- **release**: Drop trusted publishing header comments
- Note that releases publish via trusted publishing
- Add pull request template
- Regenerate CLAUDE.md
- Document the module doc comment convention
- Normalize module and package doc comments
- Strip ai tells from docs, comments, and help text

### Miscellaneous

- **assets**: Drop svg repo attribution comments
- Repository housekeeping
- Symlink AGENTS.md to CLAUDE.md
- Remove local pull request template
- **deps**: Update locked dependencies
- **deps**: Update Cargo.lock
- **deps**: Update cc to 1.4.2
- Update dependency lockfile
- Add editorconfig
- **justfile**: Standardize recipes to workspace vocabulary

### Refactor

- **justfile**: Standardize recipe names and ordering

## [1.1.3] - 2026-05-31

### CI/CD

- **release**: Use manylinux_2_28 image for wheel builds

### Release

- V1.1.3

## [1.1.2] - 2026-05-31

### CI/CD

- **release**: Build manylinux wheels via maturin-action

### Release

- V1.1.2

## [1.1.1] - 2026-05-31

### Build

- Standardize tooling on libs conventions
- Update dev dependencies
- Update dev dependencies
- Update dev dependencies
- Update dev dependencies

### CI/CD

- **workflows**: Align workflows with libs conventions

### Documentation

- Refresh project documentation

### Miscellaneous

- Add gitignore for rust artifacts and local files
- Drop legacy repo scaffolding

### Release

- V1.1.1

## [1.1.0] - 2026-03-29

### Bug fixes

- Update metadata

### Build

- Bump version to 1.1.0
- Update dependencies
- **deps**: Bump actions/attest-build-provenance from 3 to 4
- **deps**: Bump actions/download-artifact from 7 to 8
- **deps**: Bump crazy-max/ghaction-github-labeler from 5 to 6
- **deps**: Bump actions/upload-artifact from 6 to 7
- **deps**: Bump time from 0.3.44 to 0.3.47
- **deps**: Bump bytes from 1.10.1 to 1.11.1
- Update dev dependencies
- **deps**: Bump serde_json from 1.0.148 to 1.0.149
- Update dev dependencies
- Update dev dependencies
- Update dev dependencies
- Update dev dependencies
- Update dependencies and versions in Cargo.lock and Cargo.toml
- Update dependencies
- Update dependencies
- Update dependencies
- Update dependencies
- Update dependencies
- Update dependencies
- **deps**: Bump serde_json from 1.0.142 to 1.0.143
- Update dependencies
- Update dependencies
- Update dependencies
- Update dependencies
- **deps**: Bump clap from 4.5.40 to 4.5.41

### CI/CD

- Replace macos-13 with macos-latest runner
- Remove deprecated GitHub workflows and files
- Upgrade actions
- Update version of the Checkout action
- Upgrade upload-artifact action to v5 in release workflow
- Upgrade download-artifact action to v6 and adjust subject-path formatting
- Update actions version
- Update actions/checkout
- Disable cheker
- Enable ARM flags

### Documentation

- Center align license badge in README
- Update license
- Remove outdated funding link and badge from README

### Miscellaneous

- Update chore files
- Update .gitignore

### Refactor

- Run linters

## [1.0.0] - 2025-07-13

### Build

- Add maturin

### CI/CD

- Add workflows

### Documentation

- Update notes
- Update contributing guidelines

### Miscellaneous

- Initial commit

### Refactor

- Run linters
- Remove semantic release
- Improve metadata handling
- Update variable names

### Style

- Run formatters

### Testing

- Add core tests

