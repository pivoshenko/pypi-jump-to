"""The module that contains core PyPI API functionality."""

import re

from typing import Any
from typing import Dict
from typing import List
from typing import Optional
from typing import Pattern

import requests

from returns.result import Failure
from returns.result import ResultE
from returns.result import Success


def get_package_info(package_name: str) -> ResultE[Dict[str, Any]]:
    """Get package info from the PyPI API."""

    pypi_url: str = "https://pypi.org/pypi/{package_name}/json"
    response: requests.Response = requests.get(pypi_url.format(package_name=package_name))

    try:
        response.raise_for_status()

    except requests.exceptions.HTTPError as exception:
        return Failure(exception)

    return Success(response.json())


def get_pypi_url(package_info: Dict[str, Any]) -> ResultE[str]:
    """Get the PyPI URL."""

    pypi_url: Optional[str] = package_info.get("info", {}).get("package_url", "").rstrip("/")

    return Success(pypi_url) if pypi_url else Failure(ValueError("PyPI URL not found."))


def get_homepage_url(package_info: Dict[str, Any]) -> ResultE[str]:
    """Get the hompage URL."""

    homepage_url: Optional[str] = package_info.get("info", {}).get("home_page", "").rstrip("/")

    return Success(homepage_url) if homepage_url else Failure(ValueError("Homepage URL not found."))


def get_git_url(package_info: Dict[str, Any]) -> ResultE[str]:
    """Get the GitHub/GitLab URL."""

    git_url_mather: Pattern[str] = re.compile(
        pattern=r"(https):\/\/(github.com|gitlab.com)\/[A-Za-z0-9_-]+\/[A-Za-z0-9_-]+",
    )

    project_urls: List[Optional[str]] = list(
        package_info.get("info", {}).get("project_urls", {}).values(),
    )

    applied_regex_urls: List[Optional[re.Match]] = [
        git_url_mather.match(url) for url in project_urls if url
    ]

    matched_urls: List[str] = [match.group() for match in applied_regex_urls if match]

    return (
        Success(matched_urls[0].rstrip("/"))
        if matched_urls
        else Failure(ValueError("Git URL not found."))
    )
