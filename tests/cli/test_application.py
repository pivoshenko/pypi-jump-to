"""Tests for the module that contains CLI application."""

from __future__ import annotations

import pytest

from cleo.testers.application_tester import ApplicationTester

from pjt.cli.application import SingleCommandApplication


def test_display_name(empty_app: SingleCommandApplication) -> None:
    """Test ``display_name`` method."""

    assert empty_app.display_name == "pjt"


@pytest.mark.parametrize(
    argnames=("application", "expected_name"),
    argvalues=[
        (pytest.lazy_fixture("empty_app"), "list"),  # type: ignore[operator]
        (pytest.lazy_fixture("app"), "default"),  # type: ignore[operator]
    ],
)
def test_default_command_name(application: SingleCommandApplication, expected_name: str) -> None:
    """Test ``_default_command`` attribute."""

    assert application._default_command == expected_name  # noqa: SLF001


def test_default_command_execution(tester: ApplicationTester) -> None:
    """Test default command execution."""

    tester.execute("World", decorated=False)

    assert tester.io.fetch_output() == "Hello World! It's the default command!\n"


def test_help_command_execution(tester: ApplicationTester) -> None:
    """Test ``help`` command execution."""

    tester.execute("--help", decorated=False)

    assert tester.io.fetch_output().startswith("\nUsage:\n  default") is True
