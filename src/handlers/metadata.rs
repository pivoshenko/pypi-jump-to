//! Module that contains PyPI metadata parsing functionality.

use serde;
use std::collections::HashMap;

const PYPI_API_BASE: &str = "https://pypi.org/pypi";
const GITHUB_DOMAIN: &str = "github.com";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(serde::Deserialize, Debug)]
pub struct PypiResponse {
    pub info: PypiInfo,
}

#[derive(serde::Deserialize, Debug)]
pub struct PypiInfo {
    pub project_urls: Option<HashMap<String, String>>,
    pub home_page: Option<String>,
}

fn extract_url_by_keys(metadata: &PypiResponse, keys: &[&str], error_msg: &str) -> Result<String> {
    if let Some(ref project_urls) = metadata.info.project_urls {
        for key in keys {
            if let Some(url) = project_urls.get(*key) {
                return Ok(url.clone());
            }
        }
    }
    Err(error_msg.into())
}

pub fn fetch_pypi_metadata(package_name: &str) -> Result<PypiResponse> {
    let pypi_url = format!("{}/{}/json", PYPI_API_BASE, package_name);

    let response = ureq::get(&pypi_url).call();

    match response {
        Ok(mut resp) => {
            let pypi_data: PypiResponse = resp.body_mut().read_json()?;
            Ok(pypi_data)
        }
        Err(error) => {
            let error_msg = error.to_string();
            if error_msg.contains("404") {
                Err(format!("Package '{}' not found on PyPI", package_name).into())
            } else if error_msg.contains("http status:") {
                Err(format!(
                    "PyPI API error for package '{}': {}. Unable to fetch package information",
                    package_name, error_msg
                )
                .into())
            } else {
                Err(format!(
                    "Failed to connect to PyPI for package '{}': {}",
                    package_name, error
                )
                .into())
            }
        }
    }
}

pub fn extract_github_url(metadata: &PypiResponse) -> Result<String> {
    if let Some(ref project_urls) = metadata.info.project_urls {
        if let Some(source_url) = project_urls.get("Source") {
            return Ok(source_url.clone());
        }

        for key in ["Repository", "Source Code"] {
            if let Some(url) = project_urls.get(key) {
                if url.contains(GITHUB_DOMAIN) {
                    return Ok(url.clone());
                }
            }
        }
    }

    Err("No GitHub repository found".into())
}

pub fn extract_documentation_url(metadata: &PypiResponse) -> Result<String> {
    extract_url_by_keys(
        metadata,
        &["Documentation", "Docs", "Document"],
        "No documentation URL found",
    )
}

pub fn extract_changelog_url(metadata: &PypiResponse) -> Result<String> {
    extract_url_by_keys(
        metadata,
        &[
            "Changelog",
            "Change Log",
            "Changes",
            "History",
            "Release Notes",
        ],
        "No changelog URL found",
    )
}

pub fn extract_github_path_url(metadata: &PypiResponse, path: &str) -> Result<String> {
    let github_url = extract_github_url(metadata)?;
    let sanitized_github_url = github_url.trim_end_matches(".git").trim_end_matches('/');

    Ok(format!("{}/{}", sanitized_github_url, path))
}

pub fn build_pypi_versions_url(package_name: &str) -> String {
    format!("https://pypi.org/project/{}/#history", package_name)
}
