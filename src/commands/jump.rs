//! Jump command implementation for opening a target page in browser.

use crate::handlers::{args, metadata};
use open;

pub fn execute(cmd: &args::JumpCommand) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_url(&cmd.package_name, &cmd.destination)?;
    open::that(url).map_err(|e| format!("Failed to open browser: {}", e))?;
    Ok(())
}

fn build_url(
    package_name: &str,
    destination: &args::Destination,
) -> Result<String, Box<dyn std::error::Error>> {
    match destination {
        // Direct URLs that don't require PyPI metadata
        args::Destination::Homepage => Ok(format!("https://pypi.org/project/{}/", package_name)),
        args::Destination::Versions => Ok(metadata::build_pypi_versions_url(package_name)),
        // URLs that require PyPI metadata
        _ => {
            let pypi_metadata = metadata::fetch_pypi_metadata(package_name)
                .map_err(|e| format!("Failed to fetch metadata for '{}': {}", package_name, e))?;

            match destination {
                args::Destination::Github => metadata::extract_github_url(&pypi_metadata),
                args::Destination::Issues => {
                    metadata::extract_github_path_url(&pypi_metadata, "issues")
                }
                args::Destination::PullRequests => {
                    metadata::extract_github_path_url(&pypi_metadata, "pulls")
                }
                args::Destination::Releases => {
                    metadata::extract_github_path_url(&pypi_metadata, "releases")
                }
                args::Destination::Tags => {
                    metadata::extract_github_path_url(&pypi_metadata, "tags")
                }
                args::Destination::Documentation => {
                    metadata::extract_documentation_url(&pypi_metadata)
                }
                args::Destination::Changelog => metadata::extract_changelog_url(&pypi_metadata),
                _ => unreachable!(),
            }
        }
    }
}
