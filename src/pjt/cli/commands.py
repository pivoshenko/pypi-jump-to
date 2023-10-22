"""Module that contains CLI commands."""

from __future__ import annotations

from cleo.commands.command import Command
from cleo.helpers import argument
from cleo.io.inputs.argument import Argument
from returns.pipeline import flow
from returns.pipeline import is_successful
from returns.pointfree import bind
from returns.result import Success

from pjt.core import browser
from pjt.core import entities
from pjt.core import pypi


def get_description() -> str:
    """Get command description."""

    header = "🐙 ✨ 🐙 ✨ 🐙"
    tool_name = "<fg=cyan;options=bold>pypi-jump-to</>"
    description = "a quick navigation tool for the PyPI packages"

    return f"{header}\n  {tool_name} - {description}"


def get_destinations_description(destinations: entities.Destination) -> str:
    """Get a description of the destinations for the CLI."""

    header: str = "Available destinations"
    separator: str = "----------------------"

    footer_style: str = "<fg=dark_gray>\n{0}</>"
    footer: str = "Omitting the destination or entering an non-existing one takes you to the PyPI."

    row_style: str = "<fg=green>{0}</> → {1}"
    rows: list[str] = [
        row_style.format(destination.value.alias, destination.value.description)
        for destination in destinations  # type: ignore[attr-defined]
    ]

    return "\n".join((header, separator, *rows, footer_style.format(footer)))


class DefaultCommand(Command):
    """Default command."""

    name: str = "pjt"
    description: str = get_description()

    arguments: list[Argument] = [  # noqa: RUF012
        argument(
            "package",
            description="Package name",
        ),
        argument(
            "destination",
            optional=True,
            default="p",
            description=get_destinations_description(
                entities.Destination,  # type: ignore[arg-type]
            ),
        ),
    ]

    def handle(self) -> int:
        """Execute the command."""

        package: Success[str] = Success(self.argument("package"))
        destination = entities.Destination.get_by_alias(
            self.argument("destination"),
        )

        url_getter = destination.bind(pypi.get_url_getter)  # type: ignore[var-annotated, arg-type]
        container = flow(  # type: ignore[arg-type]
            package,
            bind(pypi.get_package_info),
            bind(url_getter),  # type: ignore[arg-type]
            bind(browser.open_url),
        )

        if is_successful(container):
            return 0

        raise container.failure()  # noqa: RSE102
