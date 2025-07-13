//! pypi-jump-to (pjt) - a quick navigation tool for the PyPI packages.
//!
//! This library provides functionality to navigate to various PyPI package-related URLs
//! such as GitHub repositories, documentation, changelogs, and more.

pub mod commands;
pub mod handlers;

pub use handlers::*;
