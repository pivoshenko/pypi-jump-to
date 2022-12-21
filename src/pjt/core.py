"""The module that contains core functionality."""

import webbrowser  # pragma: no cover

from returns.result import Failure  # pragma: no cover
from returns.result import ResultE  # pragma: no cover
from returns.result import Success  # pragma: no cover


def open_url(url: str) -> ResultE[bool]:  # pragma: no cover
    """Open the URL in a browser."""

    try:
        response: bool = webbrowser.open(url, new=1)

    except (webbrowser.Error, Exception) as exception:
        return Failure(webbrowser.Error(exception))

    return Success(response)
