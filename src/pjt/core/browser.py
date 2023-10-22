"""Module that contains browser functionality."""

from __future__ import annotations

import webbrowser

from returns.result import Failure
from returns.result import ResultE
from returns.result import Success


def open_url(url: str) -> ResultE[bool]:
    """Open the URL in the default browser."""

    try:
        response: bool = webbrowser.open(url, new=1)

    except (webbrowser.Error, Exception) as exception:
        return Failure(webbrowser.Error(exception))

    return Success(response)
