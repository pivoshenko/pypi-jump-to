"""Module that contains CLI commands."""

from __future__ import annotations

from cleo.commands.command import Command
from cleo.helpers import argument
from cleo.io.inputs.argument import Argument

from pjt import core


def get_description() -> str:
    """Get command description."""

    header = "🐙 ✨ 🐙 ✨ 🐙"
    tool_name = "<fg=cyan;options=bold>pypi-jump-to</>"
    description = "a quick navigation tool for the PyPI packages"

    return f"{header}\n  {tool_name} - {description}"


def get_destinations_description(destinations: core.entities.Destination) -> str:
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
                core.entities.Destination,  # type: ignore[arg-type]
            ),
        ),
    ]

    def handle(self) -> int:
        """Execute the command."""

        return 0
