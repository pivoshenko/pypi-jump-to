//! Jump command implementation for opening a target page in browser.

use crate::handlers::{args, metadata};
use std;

pub fn execute(cmd: &args::JumpCommand) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_url(&cmd.package_name, &cmd.destination)?;
    open_browser(&url)?;
    Ok(())
}

fn build_url(
    package_name: &str,
    destination: &args::Destination,
) -> Result<String, Box<dyn std::error::Error>> {
    match destination {
        args::Destination::Homepage => {
            metadata::fetch_pypi_metadata(package_name)?;
            Ok(format!("https://pypi.org/project/{}/", package_name))
        }
        args::Destination::Versions => {
            metadata::fetch_pypi_metadata(package_name)?;
            Ok(metadata::build_pypi_versions_url(package_name))
        }
        args::Destination::Github => {
            let pypi_metadata = metadata::fetch_pypi_metadata(package_name)?;
            metadata::extract_github_url(&pypi_metadata)
        }
        args::Destination::Issues => {
            let pypi_metadata = metadata::fetch_pypi_metadata(package_name)?;
            metadata::extract_github_path_url(&pypi_metadata, "issues")
        }
        args::Destination::PullRequests => {
            let pypi_metadata = metadata::fetch_pypi_metadata(package_name)?;
            metadata::extract_github_path_url(&pypi_metadata, "pulls")
        }
        args::Destination::Releases => {
            let pypi_metadata = metadata::fetch_pypi_metadata(package_name)?;
            metadata::extract_github_path_url(&pypi_metadata, "releases")
        }
        args::Destination::Tags => {
            let pypi_metadata = metadata::fetch_pypi_metadata(package_name)?;
            metadata::extract_github_path_url(&pypi_metadata, "tags")
        }
        args::Destination::Documentation => {
            let pypi_metadata = metadata::fetch_pypi_metadata(package_name)?;
            metadata::extract_documentation_url(&pypi_metadata)
        }
        args::Destination::Changelog => {
            let pypi_metadata = metadata::fetch_pypi_metadata(package_name)?;
            metadata::extract_changelog_url(&pypi_metadata)
        }
    }
}

fn open_browser(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(url).spawn()?;
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        return Err("Unsupported platform".into());
    }
    Ok(())
}
