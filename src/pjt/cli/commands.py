"""Module that contains CLI commands."""

from cleo.commands.command import Command
from cleo.helpers import argument
from cleo.io.inputs.argument import Argument

from pjt import core


def get_destinations_description(destinations: core.entities.Destination) -> str:
    """Get a description of the destinations for the CLI."""

    header: str = "Available destinations"
    separator: str = "----------------------"

    footer_style: str = "<fg=dark_gray>{0!s}</>"
    footer: str = "Omitting the destination or entering an non-existing one takes you to the PyPI."

    row_style: str = "<fg=green>{0!s}</> → {1!s}"
    rows: list[str] = [
        row_style.format(destination.value.alias, destination.value.description)
        for destination in destinations  # type: ignore[attr-defined]
    ]

    return "\n".join((header, separator, *rows, footer_style.format(footer)))


class DefaultCommand(Command):
    """Default command."""

    name: str = "pjt"
    description: str = "🐙 pjt (pip-jump-to) - a quick navigation tool for the PyPI packages."
    arguments: list[Argument] = [
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
