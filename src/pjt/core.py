"""The module that contains core PyPI API functionality."""

import re

from typing import Any
from typing import Dict
from typing import List
from typing import Pattern

import requests

from returns.result import Failure
from returns.result import ResultE
from returns.result import Success


_URL: str = "https://pypi.org/pypi/{package_name}/json"
_GIT_URL: Pattern[str] = re.compile(
    pattern=r"(https):\/\/(github.com|gitlab.com)\/[A-Za-z0-9_-]+\/[A-Za-z0-9_-]+",
)


def get_package_info(package_name: str) -> ResultE[Dict[str, Any]]:
    """Get package info from PyPI API."""

    response: requests.Response = requests.get(_URL.format(package_name=package_name))

    try:
        response.raise_for_status()

    except requests.exceptions.HTTPError as exception:
        return Failure(exception)

    return Success(response.json())


def get_pypi_url(package_info: Dict[str, Any]) -> ResultE[str]:
    """Get the PyPI URL."""

    pypi_url: str = package_info.get("info", {}).get("package_url", "")

    return Success(pypi_url) if pypi_url else Failure(ValueError("No PyPI URL found."))


def get_homepage_url(package_info: Dict[str, Any]) -> ResultE[str]:
    """Get the hompage URL."""

    homepage_url: str = package_info.get("info", {}).get("home_page", "")

    return Success(homepage_url) if homepage_url else Failure(ValueError("No Homepage URL found."))


def get_git_url(package_info: Dict[str, Any]) -> ResultE[str]:
    """Get the GitHub/GitLab URL."""

    project_urls: List[str] = list(package_info.get("info", {}).get("project_urls", {}).values())
