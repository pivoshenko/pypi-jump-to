"""Module that contains core PyPI functionality."""

from __future__ import annotations

import re

from re import Pattern
from typing import Any

import requests

from returns.result import Failure
from returns.result import ResultE
from returns.result import Success


def get_package_info(package_name: str) -> ResultE[dict[str, Any]]:
    """Get package info from the PyPI API."""

    pypi_url: str = "https://pypi.org/pypi/{package_name}/json"
    response: requests.Response = requests.get(
        pypi_url.format(package_name=package_name),
        timeout=15,
    )

    try:
        response.raise_for_status()

    except requests.exceptions.HTTPError as exception:
        return Failure(exception)

    return Success(response.json())


def get_pypi_url(package_info: dict[str, Any]) -> ResultE[str]:
    """Get the PyPI URL."""

    pypi_url: str | None = package_info.get("info", {}).get("package_url", "").rstrip("/")

    return Success(pypi_url) if pypi_url else Failure(ValueError("PyPI URL not found."))


def get_homepage_url(package_info: dict[str, Any]) -> ResultE[str]:
    """Get the hompage URL."""

    homepage_url: str | None = package_info.get("info", {}).get("home_page", "").rstrip("/")

    return Success(homepage_url) if homepage_url else Failure(ValueError("Homepage URL not found."))


def get_repository_url(package_info: dict[str, Any]) -> ResultE[str]:
    """Get the GitHub/GitLab repository URL."""

    repository_url: Pattern[str] = re.compile(
        pattern=r"(https):\/\/(github.com|gitlab.com)\/[A-Za-z0-9_-]+\/[A-Za-z0-9_-]+",
    )

    project_urls: dict[str, str] = package_info.get("info", {}).get("project_urls", {})
    project_urls_values: list[str] = list(project_urls.values()) if project_urls else []

    applied_regex_urls: list[re.Match | None] = [
        repository_url.match(url) for url in project_urls_values if url
    ]

    matched_urls: list[str] = [match.group() for match in applied_regex_urls if match]

    return (
        Success(matched_urls[0].rstrip("/"))
        if matched_urls
        else Failure(ValueError("Repository URL not found."))
    )
