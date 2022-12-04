//! PyPI metadata parsing functionality.

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct PypiResponse {
    pub info: PypiInfo,
}

#[derive(Deserialize)]
pub struct PypiInfo {
    pub project_urls: Option<HashMap<String, String>>,
    pub home_page: Option<String>,
}

pub fn fetch_pypi_metadata(package: &str) -> Result<PypiResponse, Box<dyn std::error::Error>> {
    let pypi_url = format!("https://pypi.org/pypi/{}/json", package);

    let response = ureq::get(&pypi_url).call();

    match response {
        Ok(mut resp) => {
            let pypi_data: PypiResponse = resp.body_mut().read_json()?;
            Ok(pypi_data)
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("404") {
                Err(format!(
                    "Package not found on PyPI",
                )
                .into())
            } else if error_msg.contains("http status:") {
                Err(format!(
                    "PyPI API error: {}. Unable to fetch package information",
                    error_msg
                )
                .into())
            } else {
                Err(format!("Failed to connect to PyPI: {}", e).into())
            }
        }
    }
}

pub fn extract_github_url(metadata: &PypiResponse) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(ref project_urls) = metadata.info.project_urls {
        if let Some(source_url) = project_urls.get("Source") {
            return Ok(source_url.clone());
        }

        for key in ["Repository", "Source Code"] {
            if let Some(url) = project_urls.get(key) {
                if url.contains("github.com") {
                    return Ok(url.clone());
                }
            }
        }
    }

    Err("No GitHub repository found".into())
}

pub fn extract_documentation_url(
    metadata: &PypiResponse,
) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(ref project_urls) = metadata.info.project_urls {
        // Check for common documentation keys
        for key in ["Documentation", "Docs", "Document"] {
            if let Some(url) = project_urls.get(key) {
                return Ok(url.clone());
            }
        }
    }

    Err("No documentation URL found".into())
}

pub fn extract_changelog_url(
    metadata: &PypiResponse,
) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(ref project_urls) = metadata.info.project_urls {
        // Check for common changelog keys
        for key in [
            "Changelog",
            "Change Log",
            "Changes",
            "History",
            "Release Notes",
        ] {
            if let Some(url) = project_urls.get(key) {
                return Ok(url.clone());
            }
        }
    }

    Err("No changelog URL found".into())
}

pub fn extract_github_path_url(
    metadata: &PypiResponse,
    path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let github_url = extract_github_url(metadata)?;

    let sanitized_github_url = github_url.trim_end_matches('/').trim_end_matches(".git");

    Ok(format!("{}/{}", sanitized_github_url, path))
}

pub fn build_pypi_versions_url(package: &str) -> String {
    format!("https://pypi.org/project/{}/#history", package)
}
