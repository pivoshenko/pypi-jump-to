"""Module that contains entities."""

from __future__ import annotations

import enum
import dataclasses

from returns.result import Failure
from returns.result import ResultE
from returns.result import Success


@dataclasses.dataclass
class DestinationInfo:
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
        description="homepage (e.g., website, docs)",
    )
    repository: DestinationInfo = DestinationInfo(
        alias="r",
        description="repository (e.g., github)",
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

    @classmethod
    def get_by_alias(cls, alias: str) -> ResultE[Destination]:
        """Get a field based on the alias."""

        for destination in Destination:
            if destination.value.alias == alias:
                return Success(destination)

        return Failure(KeyError(alias))
