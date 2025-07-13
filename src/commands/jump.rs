//! Module that contains jump command implementation for opening a target page in a browser.

use crate::handlers;
use open;

pub fn execute(cmd: &handlers::args::JumpCommand) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_url(&cmd.package_name, &cmd.destination)?;
    open::that(url).map_err(|error| format!("Failed to open browser: {error}"))?;
    Ok(())
}

fn build_url(
    package_name: &str,
    destination: &handlers::args::Destination,
) -> Result<String, Box<dyn std::error::Error>> {
    match destination {
        handlers::args::Destination::Homepage => {
            Ok(format!("https://pypi.org/project/{package_name}/"))
        }
        handlers::args::Destination::Versions => {
            Ok(handlers::metadata::build_pypi_versions_url(package_name))
        }
        _ => {
            let pypi_metadata =
                handlers::metadata::fetch_pypi_metadata(package_name).map_err(|error| {
                    format!("Failed to fetch metadata for '{package_name}': {error}")
                })?;

            match destination {
                handlers::args::Destination::Github => {
                    handlers::metadata::extract_github_url(&pypi_metadata)
                }
                handlers::args::Destination::Issues => {
                    handlers::metadata::extract_github_path_url(&pypi_metadata, "issues")
                }
                handlers::args::Destination::PullRequests => {
                    handlers::metadata::extract_github_path_url(&pypi_metadata, "pulls")
                }
                handlers::args::Destination::Releases => {
                    handlers::metadata::extract_github_path_url(&pypi_metadata, "releases")
                }
                handlers::args::Destination::Tags => {
                    handlers::metadata::extract_github_path_url(&pypi_metadata, "tags")
                }
                handlers::args::Destination::Documentation => {
                    handlers::metadata::extract_documentation_url(&pypi_metadata)
                }
                handlers::args::Destination::Changelog => {
                    handlers::metadata::extract_changelog_url(&pypi_metadata)
                }
                _ => unreachable!(),
            }
        }
    }
}
