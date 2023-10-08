"""Fixtures and configuration for the tests."""

from __future__ import annotations

import pytest

from cleo.commands.command import Command
from cleo.helpers import argument
from cleo.io.inputs.argument import Argument
from cleo.testers.application_tester import ApplicationTester

from pjt.cli.application import SingleCommandApplication


class DefaultCommand(Command):
    """Default command."""

    name: str = "default"
    arguments: list[Argument] = [argument(name="greeting")]  # noqa: RUF012

    def handle(self) -> int:
        """Execute the command."""

        self.info(f"Hello {self.argument('greeting')}! It's the default command!")

        return 0


class SideCommand(Command):
    """Side command."""

    name: str = "side"


@pytest.fixture()
def empty_app() -> SingleCommandApplication:
    """Get empty CLI application."""

    return SingleCommandApplication(name="pjt", version="v1")


@pytest.fixture()
def app() -> SingleCommandApplication:
    """Get CLI application."""

    application = SingleCommandApplication(name="pjt", version="v1")
    application.add(DefaultCommand(), default=True)
    application.add(SideCommand())

    return application


@pytest.fixture()
def tester(app: SingleCommandApplication) -> ApplicationTester:
    """Get a tester for the ``cleo`` applications."""

    app.catch_exceptions(False)

    return ApplicationTester(app)
