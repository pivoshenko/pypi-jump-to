"""pjt (pypi-jump-to) - a quick navigation tool for the PyPI packages."""

from __future__ import annotations

from pjt import __title__
from pjt import __version__
from pjt import cli


application = cli.application.SingleCommandApplication(name=__title__, version=__version__)
application.add(command=cli.commands.DefaultCommand(), default=True)


def run() -> int:  # pragma: no cover
    """Run the application."""

    return application.run()


if __name__ == "__main__":
    run()
