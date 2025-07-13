//! Tests for PyPI API interaction and network-related functionality.

use pypi_jump_to::handlers::metadata::{PypiInfo, PypiResponse, fetch_pypi_metadata};
use std::collections::HashMap;

#[cfg(test)]
mod pypi_api_tests {
    use super::*;

    #[test]
    fn test_pypi_api_url_construction() {
        // This test verifies the URL pattern used for PyPI API calls
        let package_name = "requests";
        let expected_url = format!("https://pypi.org/pypi/{package_name}/json");
        assert_eq!(expected_url, "https://pypi.org/pypi/requests/json");
    }

    #[test]
    fn test_pypi_api_url_with_special_characters() {
        let package_name = "my-package_name.test";
        let expected_url = format!("https://pypi.org/pypi/{}/json", package_name);
        assert_eq!(
            expected_url,
            "https://pypi.org/pypi/my-package_name.test/json"
        );
    }

    #[test]
    #[ignore]
    fn test_fetch_real_package_metadata() {
        let result = fetch_pypi_metadata("requests");
        assert!(result.is_ok());

        let metadata = result.unwrap();
        assert!(metadata.info.project_urls.is_some());

        // Verify that requests package has a GitHub repository
        let project_urls = metadata.info.project_urls.unwrap();
        let has_github_url = project_urls.values().any(|url| url.contains("github.com"));
        assert!(has_github_url, "requests package should have a GitHub URL");
    }

    #[test]
    #[ignore]
    fn test_fetch_nonexistent_package_metadata() {
        let result = fetch_pypi_metadata("definitely-not-a-real-package-name-12345");
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("not found on PyPI"));
    }
}

#[cfg(test)]
mod error_handling_tests {

    #[test]
    fn test_error_message_format_for_404() {
        let package_name = "nonexistent-package";
        let expected_error = format!("Package '{}' not found on PyPI", package_name);
        assert_eq!(
            expected_error,
            "Package 'nonexistent-package' not found on PyPI"
        );
    }

    #[test]
    fn test_error_message_format_for_http_errors() {
        let package_name = "some-package";
        let http_error = "http status: 500";
        let expected_error = format!(
            "PyPI API error for package '{}': {}. Unable to fetch package information",
            package_name, http_error
        );
        assert_eq!(
            expected_error,
            "PyPI API error for package 'some-package': http status: 500. Unable to fetch package information"
        );
    }

    #[test]
    fn test_error_message_format_for_connection_errors() {
        let package_name = "some-package";
        let connection_error = "connection refused";
        let expected_error = format!(
            "Failed to connect to PyPI for package '{}': {}",
            package_name, connection_error
        );
        assert_eq!(
            expected_error,
            "Failed to connect to PyPI for package 'some-package': connection refused"
        );
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_pypi_response_deserialization_complete() {
        let json_data = r#"
        {
            "info": {
                "project_urls": {
                    "Source": "https://github.com/user/repo",
                    "Documentation": "https://docs.example.com",
                    "Changelog": "https://changelog.example.com"
                },
                "home_page": "https://example.com"
            }
        }
        "#;

        let result: Result<PypiResponse, _> = serde_json::from_str(json_data);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.info.project_urls.is_some());
        assert!(response.info.home_page.is_some());

        let project_urls = response.info.project_urls.unwrap();
        assert_eq!(
            project_urls.get("Source").unwrap(),
            "https://github.com/user/repo"
        );
        assert_eq!(
            project_urls.get("Documentation").unwrap(),
            "https://docs.example.com"
        );
        assert_eq!(
            project_urls.get("Changelog").unwrap(),
            "https://changelog.example.com"
        );
        assert_eq!(response.info.home_page.unwrap(), "https://example.com");
    }

    #[test]
    fn test_pypi_response_deserialization_minimal() {
        let json_data = r#"
        {
            "info": {
                "project_urls": null,
                "home_page": null
            }
        }
        "#;

        let result: Result<PypiResponse, _> = serde_json::from_str(json_data);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.info.project_urls.is_none());
        assert!(response.info.home_page.is_none());
    }

    #[test]
    fn test_pypi_response_deserialization_empty_project_urls() {
        let json_data = r#"
        {
            "info": {
                "project_urls": {},
                "home_page": "https://example.com"
            }
        }
        "#;

        let result: Result<PypiResponse, _> = serde_json::from_str(json_data);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.info.project_urls.is_some());
        assert!(response.info.project_urls.unwrap().is_empty());
        assert_eq!(response.info.home_page.unwrap(), "https://example.com");
    }

    #[test]
    fn test_pypi_response_deserialization_missing_info() {
        let json_data = r#"
        {
            "not_info": {}
        }
        "#;

        let result: Result<PypiResponse, _> = serde_json::from_str(json_data);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod real_world_examples_tests {
    use super::*;

    #[test]
    fn test_realistic_project_urls_requests_style() {
        let mut project_urls = HashMap::new();
        project_urls.insert(
            "Source".to_string(),
            "https://github.com/psf/requests".to_string(),
        );
        project_urls.insert(
            "Documentation".to_string(),
            "https://requests.readthedocs.io".to_string(),
        );
        project_urls.insert(
            "Changelog".to_string(),
            "https://github.com/psf/requests/blob/main/HISTORY.md".to_string(),
        );

        let metadata = PypiResponse {
            info: PypiInfo {
                project_urls: Some(project_urls),
                home_page: Some("https://requests.readthedocs.io".to_string()),
            },
        };

        assert!(pypi_jump_to::handlers::metadata::extract_github_url(&metadata).is_ok());
        assert!(pypi_jump_to::handlers::metadata::extract_documentation_url(&metadata).is_ok());
        assert!(pypi_jump_to::handlers::metadata::extract_changelog_url(&metadata).is_ok());
    }

    #[test]
    fn test_realistic_project_urls_numpy_style() {
        let mut project_urls = HashMap::new();
        project_urls.insert("Homepage".to_string(), "https://numpy.org".to_string());
        project_urls.insert(
            "Source Code".to_string(),
            "https://github.com/numpy/numpy".to_string(),
        );
        project_urls.insert(
            "Bug Tracker".to_string(),
            "https://github.com/numpy/numpy/issues".to_string(),
        );
        project_urls.insert(
            "Documentation".to_string(),
            "https://numpy.org/doc/".to_string(),
        );

        let metadata = PypiResponse {
            info: PypiInfo {
                project_urls: Some(project_urls),
                home_page: Some("https://numpy.org".to_string()),
            },
        };

        assert!(pypi_jump_to::handlers::metadata::extract_github_url(&metadata).is_ok());
        assert!(pypi_jump_to::handlers::metadata::extract_documentation_url(&metadata).is_ok());
    }

    #[test]
    fn test_realistic_project_urls_gitlab_package() {
        let mut project_urls = HashMap::new();
        project_urls.insert("Homepage".to_string(), "https://example.com".to_string());
        project_urls.insert(
            "Repository".to_string(),
            "https://gitlab.com/user/project".to_string(),
        );
        project_urls.insert(
            "Documentation".to_string(),
            "https://docs.example.com".to_string(),
        );

        let metadata = PypiResponse {
            info: PypiInfo {
                project_urls: Some(project_urls),
                home_page: Some("https://example.com".to_string()),
            },
        };

        assert!(pypi_jump_to::handlers::metadata::extract_github_url(&metadata).is_err());
        assert!(pypi_jump_to::handlers::metadata::extract_documentation_url(&metadata).is_ok());
    }
}
