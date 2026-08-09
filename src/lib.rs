//! Package that contains pypi-jump-to (pjt), a quick navigation tool for the PyPI packages.
//!
//! Resolves the URL for a PyPI package: its GitHub repository, documentation,
//! changelog, issues, pulls, releases, tags, or version history.

pub mod commands;
pub mod handlers;

pub use handlers::*;
