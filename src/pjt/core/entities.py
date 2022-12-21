"""Module that contains entities."""

import enum
import dataclasses


@dataclasses.dataclass
class DestinationInfo(object):
    """Dataclass that represents destination info."""

    alias: str
    description: str


class Destination(enum.Enum):
    """Available destinations."""

    pypi: DestinationInfo = DestinationInfo(
        alias="p",
        description="package info on https://pypi.org",
    )
    homepage: DestinationInfo = DestinationInfo(
        alias="h",
        description="homepage (e.g., docs)",
    )
    repository: DestinationInfo = DestinationInfo(
        alias="r",
        description="repository (e.g., github, gitlab)",
    )
    changelog: DestinationInfo = DestinationInfo(
        alias="c",
        description="changelog",
    )
    issues: DestinationInfo = DestinationInfo(
        alias="i",
        description="issues",
    )
    pr: DestinationInfo = DestinationInfo(
        alias="pr",
        description="pull requests",
    )
    releases: DestinationInfo = DestinationInfo(
        alias="rl",
        description="list of github releases",
    )


@dataclasses.dataclass
class URL(object):
    """Dataclass that represents an URL."""

    destination: str
    value: str


@dataclasses.dataclass
class Package(object):
    """Dataclass that represents a Python package."""

    name: str
    version: str
    urls: list[URL]
    releases: list[tuple[str, str]]
