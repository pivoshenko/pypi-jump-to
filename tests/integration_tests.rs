//! Integration tests for jump command URL building.

use pypi_jump_to::handlers::metadata::{
    PypiResponse, build_pypi_versions_url, extract_changelog_url, extract_documentation_url,
    extract_github_path_url, extract_github_url,
};
use std::collections::HashMap;

fn create_mock_pypi_response() -> PypiResponse {
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

    PypiResponse {
        info: pypi_jump_to::handlers::metadata::PypiInfo {
            project_urls: Some(project_urls),
            home_page: None,
        },
    }
}

#[cfg(test)]
mod url_building_tests {
    use super::*;
    use pypi_jump_to::handlers::metadata::{
        build_pypi_versions_url, extract_changelog_url, extract_documentation_url,
        extract_github_path_url, extract_github_url,
    };

    #[test]
    fn test_build_homepage_url() {
        let package_name = "requests";
        let expected_url = "https://pypi.org/project/requests/";
        assert_eq!(
            format!("https://pypi.org/project/{}/", package_name),
            expected_url
        );
    }

    #[test]
    fn test_build_versions_url() {
        let package_name = "requests";
        let url = build_pypi_versions_url(package_name);
        assert_eq!(url, "https://pypi.org/project/requests/#history");
    }

    #[test]
    fn test_build_github_url() {
        let metadata = create_mock_pypi_response();
        let result = extract_github_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/psf/requests");
    }

    #[test]
    fn test_build_issues_url() {
        let metadata = create_mock_pypi_response();
        let result = extract_github_path_url(&metadata, "issues");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/psf/requests/issues");
    }

    #[test]
    fn test_build_pull_requests_url() {
        let metadata = create_mock_pypi_response();
        let result = extract_github_path_url(&metadata, "pulls");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/psf/requests/pulls");
    }

    #[test]
    fn test_build_releases_url() {
        let metadata = create_mock_pypi_response();
        let result = extract_github_path_url(&metadata, "releases");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/psf/requests/releases");
    }

    #[test]
    fn test_build_tags_url() {
        let metadata = create_mock_pypi_response();
        let result = extract_github_path_url(&metadata, "tags");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/psf/requests/tags");
    }

    #[test]
    fn test_build_documentation_url() {
        let metadata = create_mock_pypi_response();
        let result = extract_documentation_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://requests.readthedocs.io");
    }

    #[test]
    fn test_build_changelog_url() {
        let metadata = create_mock_pypi_response();
        let result = extract_changelog_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://github.com/psf/requests/blob/main/HISTORY.md"
        );
    }
}

#[cfg(test)]
mod destination_mapping_tests {
    use super::*;

    #[test]
    fn test_destination_url_mapping() {
        let package_name = "requests";
        let metadata = create_mock_pypi_response();

        // Test direct URLs (no PyPI metadata needed)
        let homepage_url = format!("https://pypi.org/project/{}/", package_name);
        assert_eq!(homepage_url, "https://pypi.org/project/requests/");

        let versions_url = build_pypi_versions_url(package_name);
        assert_eq!(versions_url, "https://pypi.org/project/requests/#history");

        // Test URLs requiring PyPI metadata
        let github_url = extract_github_url(&metadata).unwrap();
        assert_eq!(github_url, "https://github.com/psf/requests");

        let issues_url = extract_github_path_url(&metadata, "issues").unwrap();
        assert_eq!(issues_url, "https://github.com/psf/requests/issues");

        let pulls_url = extract_github_path_url(&metadata, "pulls").unwrap();
        assert_eq!(pulls_url, "https://github.com/psf/requests/pulls");

        let releases_url = extract_github_path_url(&metadata, "releases").unwrap();
        assert_eq!(releases_url, "https://github.com/psf/requests/releases");

        let tags_url = extract_github_path_url(&metadata, "tags").unwrap();
        assert_eq!(tags_url, "https://github.com/psf/requests/tags");

        let docs_url = extract_documentation_url(&metadata).unwrap();
        assert_eq!(docs_url, "https://requests.readthedocs.io");

        let changelog_url = extract_changelog_url(&metadata).unwrap();
        assert_eq!(
            changelog_url,
            "https://github.com/psf/requests/blob/main/HISTORY.md"
        );
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;
    use pypi_jump_to::handlers::metadata::{PypiResponse, extract_github_url};

    #[test]
    fn test_package_with_no_github_repository() {
        let mut project_urls = HashMap::new();
        project_urls.insert(
            "Documentation".to_string(),
            "https://docs.example.com".to_string(),
        );

        let metadata = PypiResponse {
            info: pypi_jump_to::handlers::metadata::PypiInfo {
                project_urls: Some(project_urls),
                home_page: None,
            },
        };

        let result = extract_github_url(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No GitHub repository found"
        );
    }

    #[test]
    fn test_package_with_empty_project_urls() {
        let metadata = PypiResponse {
            info: pypi_jump_to::handlers::metadata::PypiInfo {
                project_urls: Some(HashMap::new()),
                home_page: None,
            },
        };

        let result = extract_github_url(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No GitHub repository found"
        );
    }

    #[test]
    fn test_package_with_null_project_urls() {
        let metadata = PypiResponse {
            info: pypi_jump_to::handlers::metadata::PypiInfo {
                project_urls: None,
                home_page: None,
            },
        };

        let result = extract_github_url(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No GitHub repository found"
        );
    }

    #[test]
    fn test_package_with_multiple_non_github_repositories() {
        let mut project_urls = HashMap::new();
        project_urls.insert(
            "Repository".to_string(),
            "https://gitlab.com/user/repo".to_string(),
        );
        project_urls.insert(
            "Source Code".to_string(),
            "https://bitbucket.org/user/repo".to_string(),
        );

        let metadata = PypiResponse {
            info: pypi_jump_to::handlers::metadata::PypiInfo {
                project_urls: Some(project_urls),
                home_page: None,
            },
        };

        let result = extract_github_url(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No GitHub repository found"
        );
    }
}
