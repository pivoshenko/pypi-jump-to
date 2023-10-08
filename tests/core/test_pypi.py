"""Tests for the module ``src/pjt/core/pypi.py``."""
from __future__ import annotations

from typing import Any

import pytest

from returns.pipeline import is_successful
from returns.result import ResultE

from pjt.core import pypi


@pytest.mark.parametrize(
    argnames=("package_name", "expected"),
    argvalues=[
        ("pip", True),
        ("fake_pip", False),
    ],
)
def test_get_package_info(package_name: str, expected: bool) -> None:
    """Test for the ``get_package_info`` function."""

    package_info: ResultE[dict[str, Any]] = pypi.get_package_info(package_name=package_name)

    assert is_successful(package_info) == expected


@pytest.mark.parametrize(
    argnames=("package_info", "expected", "expected_url"),
    argvalues=[
        (
            {
                "info": {
                    "home_page": "https://pandas.pydata.org",
                    "package_url": "https://pypi.org/project/pandas/",
                    "project_url": "https://pypi.org/project/pandas/",
                    "project_urls": {
                        "Bug Tracker": "https://github.com/pandas-dev/pandas/issues",
                        "Documentation": "https://pandas.pydata.org/pandas-docs/stable",
                        "Homepage": "https://pandas.pydata.org",
                        "Source Code": "https://github.com/pandas-dev/pandas",
                    },
                },
            },
            True,
            "https://pypi.org/project/pandas",
        ),
        (
            {},
            False,
            "",
        ),
    ],
)
def test_get_pypi_url(package_info: dict[str, Any], expected: bool, expected_url: str) -> None:
    """Test for the ``get_pypi_url`` function."""

    pypi_url: ResultE[str] = pypi.get_pypi_url(package_info=package_info)

    if expected:
        assert pypi_url.unwrap() == expected_url

    assert is_successful(pypi_url) == expected


@pytest.mark.parametrize(
    argnames=("package_info", "expected", "expected_url"),
    argvalues=[
        (
            {
                "info": {
                    "home_page": "https://pandas.pydata.org",
                    "package_url": "https://pypi.org/project/pandas/",
                    "project_url": "https://pypi.org/project/pandas/",
                    "project_urls": {
                        "Bug Tracker": "https://github.com/pandas-dev/pandas/issues",
                        "Documentation": "https://pandas.pydata.org/pandas-docs/stable",
                        "Homepage": "https://pandas.pydata.org",
                        "Source Code": "https://github.com/pandas-dev/pandas",
                    },
                },
            },
            True,
            "https://pandas.pydata.org",
        ),
        ({}, False, ""),
    ],
)
def test_get_homepage_url(package_info: dict[str, Any], expected: bool, expected_url: str) -> None:
    """Test for the ``get_homepage_url`` function."""

    homepage_url: ResultE[str] = pypi.get_homepage_url(package_info=package_info)

    if expected:
        assert homepage_url.unwrap() == expected_url

    assert is_successful(homepage_url) == expected


@pytest.mark.parametrize(
    argnames=("package_info", "expected", "expected_url"),
    argvalues=[
        (
            {
                "info": {
                    "home_page": "https://pandas.pydata.org",
                    "package_url": "https://pypi.org/project/pandas/",
                    "project_url": "https://pypi.org/project/pandas/",
                    "project_urls": {
                        "Bug Tracker": "https://github.com/pandas-dev/pandas/issues",
                        "Documentation": "https://pandas.pydata.org/pandas-docs/stable",
                        "Homepage": "https://pandas.pydata.org",
                        "Source Code": "https://github.com/pandas-dev/pandas",
                    },
                },
            },
            True,
            "https://github.com/pandas-dev/pandas",
        ),
        ({}, False, ""),
    ],
)
def test_get_repository_url(
    package_info: dict[str, Any],
    expected: bool,
    expected_url: str,
) -> None:
    """Test for the ``get_repository_url`` function."""

    git_url: ResultE[str] = pypi.get_repository_url(package_info=package_info)

    if expected:
        assert git_url.unwrap() == expected_url

    assert is_successful(git_url) == expected
