//! Tests for PyPI metadata parsing functionality.

use pypi_jump_to::handlers::metadata::{
    PypiInfo, PypiResponse, build_pypi_versions_url, extract_changelog_url,
    extract_documentation_url, extract_github_path_url, extract_github_url,
};
use std::collections::HashMap;

fn create_test_metadata(project_urls: HashMap<String, String>) -> PypiResponse {
    PypiResponse {
        info: PypiInfo {
            project_urls: Some(project_urls),
            home_page: None,
        },
    }
}

fn create_empty_metadata() -> PypiResponse {
    PypiResponse {
        info: PypiInfo {
            project_urls: None,
            home_page: None,
        },
    }
}

#[cfg(test)]
mod github_url_extraction_tests {
    use super::*;

    #[test]
    fn test_extract_github_url_with_source_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "Source".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_github_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/user/repo");
    }

    #[test]
    fn test_extract_github_url_with_repository_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "Repository".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_github_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/user/repo");
    }

    #[test]
    fn test_extract_github_url_with_source_code_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "Source Code".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_github_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/user/repo");
    }

    #[test]
    fn test_extract_github_url_prioritizes_source_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "Source".to_string(),
            "https://github.com/user/primary-repo".to_string(),
        );
        urls.insert(
            "Repository".to_string(),
            "https://github.com/user/secondary-repo".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_github_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/user/primary-repo");
    }

    #[test]
    fn test_extract_github_url_filters_non_github_repositories() {
        let mut urls = HashMap::new();
        urls.insert(
            "Repository".to_string(),
            "https://gitlab.com/user/repo".to_string(),
        );
        urls.insert(
            "Source Code".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_github_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/user/repo");
    }

    #[test]
    fn test_extract_github_url_no_github_found() {
        let mut urls = HashMap::new();
        urls.insert(
            "Repository".to_string(),
            "https://gitlab.com/user/repo".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_github_url(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No GitHub repository found"
        );
    }

    #[test]
    fn test_extract_github_url_no_project_urls() {
        let metadata = create_empty_metadata();

        let result = extract_github_url(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No GitHub repository found"
        );
    }
}

#[cfg(test)]
mod documentation_url_extraction_tests {
    use super::*;

    #[test]
    fn test_extract_documentation_url_with_documentation_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "Documentation".to_string(),
            "https://example.readthedocs.io".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_documentation_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://example.readthedocs.io");
    }

    #[test]
    fn test_extract_documentation_url_with_docs_key() {
        let mut urls = HashMap::new();
        urls.insert("Docs".to_string(), "https://docs.example.com".to_string());
        let metadata = create_test_metadata(urls);

        let result = extract_documentation_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://docs.example.com");
    }

    #[test]
    fn test_extract_documentation_url_with_document_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "Document".to_string(),
            "https://document.example.com".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_documentation_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://document.example.com");
    }

    #[test]
    fn test_extract_documentation_url_not_found() {
        let mut urls = HashMap::new();
        urls.insert(
            "Source".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_documentation_url(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No documentation URL found"
        );
    }

    #[test]
    fn test_extract_documentation_url_no_project_urls() {
        let metadata = create_empty_metadata();

        let result = extract_documentation_url(&metadata);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No documentation URL found"
        );
    }
}

#[cfg(test)]
mod changelog_url_extraction_tests {
    use super::*;

    #[test]
    fn test_extract_changelog_url_with_changelog_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "Changelog".to_string(),
            "https://github.com/user/repo/blob/main/CHANGELOG.md".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_changelog_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://github.com/user/repo/blob/main/CHANGELOG.md"
        );
    }

    #[test]
    fn test_extract_changelog_url_with_change_log_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "Change Log".to_string(),
            "https://changelog.example.com".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_changelog_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://changelog.example.com");
    }

    #[test]
    fn test_extract_changelog_url_with_changes_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "Changes".to_string(),
            "https://changes.example.com".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_changelog_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://changes.example.com");
    }

    #[test]
    fn test_extract_changelog_url_with_history_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "History".to_string(),
            "https://history.example.com".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_changelog_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://history.example.com");
    }

    #[test]
    fn test_extract_changelog_url_with_release_notes_key() {
        let mut urls = HashMap::new();
        urls.insert(
            "Release Notes".to_string(),
            "https://releases.example.com".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_changelog_url(&metadata);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://releases.example.com");
    }

    #[test]
    fn test_extract_changelog_url_not_found() {
        let mut urls = HashMap::new();
        urls.insert(
            "Source".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_changelog_url(&metadata);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No changelog URL found");
    }
}

#[cfg(test)]
mod github_path_url_tests {
    use super::*;

    #[test]
    fn test_extract_github_path_url_basic() {
        let mut urls = HashMap::new();
        urls.insert(
            "Source".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_github_path_url(&metadata, "issues");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/user/repo/issues");
    }

    #[test]
    fn test_extract_github_path_url_with_trailing_slash() {
        let mut urls = HashMap::new();
        urls.insert(
            "Source".to_string(),
            "https://github.com/user/repo/".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_github_path_url(&metadata, "pulls");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/user/repo/pulls");
    }

    #[test]
    fn test_extract_github_path_url_with_git_suffix() {
        let mut urls = HashMap::new();
        urls.insert(
            "Source".to_string(),
            "https://github.com/user/repo.git".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_github_path_url(&metadata, "releases");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/user/repo/releases");
    }

    #[test]
    fn test_extract_github_path_url_with_both_slash_and_git() {
        let mut urls = HashMap::new();
        urls.insert(
            "Source".to_string(),
            "https://github.com/user/repo/.git".to_string(),
        );
        let metadata = create_test_metadata(urls);

        let result = extract_github_path_url(&metadata, "tags");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://github.com/user/repo/tags");
    }

    #[test]
    fn test_extract_github_path_url_no_github_found() {
        let metadata = create_empty_metadata();

        let result = extract_github_path_url(&metadata, "issues");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No GitHub repository found"
        );
    }
}

#[cfg(test)]
mod pypi_versions_url_tests {
    use super::*;

    #[test]
    fn test_build_pypi_versions_url() {
        let url = build_pypi_versions_url("requests");
        assert_eq!(url, "https://pypi.org/project/requests/#history");
    }

    #[test]
    fn test_build_pypi_versions_url_with_special_characters() {
        let url = build_pypi_versions_url("my-package_name");
        assert_eq!(url, "https://pypi.org/project/my-package_name/#history");
    }

    #[test]
    fn test_build_pypi_versions_url_empty_string() {
        let url = build_pypi_versions_url("");
        assert_eq!(url, "https://pypi.org/project//#history");
    }
}
