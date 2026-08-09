<h1 align="left">
  <img src="assets/logo.svg" alt="" height="40" align="left" style="vertical-align: middle; margin-right: 12px;">
  pypi-jump-to
</h1>

<p align="left">
  <a href="https://pypi.org/project/pypi-jump-to">
    <img alt="Python" src="https://img.shields.io/pypi/pyversions/pypi-jump-to?style=flat-square&logo=python&logoColor=white&color=4856CD&label=Python">
  </a>
  <a href="https://pypi.org/project/pypi-jump-to">
    <img alt="PyPI" src="https://img.shields.io/pypi/v/pypi-jump-to?style=flat-square&logo=pypi&logoColor=white&color=4856CD&label=PyPI">
  </a>
  <a href="https://github.com/pivoshenko/pypi-jump-to/actions/workflows/ci.yaml">
    <img alt="CI" src="https://img.shields.io/github/actions/workflow/status/pivoshenko/pypi-jump-to/ci.yaml?label=CI&style=flat-square&logo=githubactions&logoColor=white&color=0A6847">
  </a>
  <a href="https://docs.astral.sh/ruff">
    <img alt="Ruff" src="https://img.shields.io/badge/Style-ruff-black.svg?style=flat-square&logo=ruff&logoColor=white&color=D7FF64">
  </a>
  <a href="https://stand-with-ukraine.pp.ua">
    <img alt="StandWithUkraine" src="https://img.shields.io/badge/Support-Ukraine-FFC93C?style=flat-square&labelColor=07689F">
  </a>
</p>

## Overview

`pypi-jump-to (pjt)` - a quick navigation tool for the PyPI packages. Save five seconds thousands of times by jumping straight to the right URL:

```shell
pjt <package> [destination]
```

### Features

A single binary with no external dependencies, thanks to the pure Rust core, so it is fast and uses little memory. You get to the right page in seconds instead of building URLs or searching for them by hand.

### Available Destinations

- `h` → Homepage PyPI (default)
- `c` → Changelog
- `d` → Documentation
- `g` → Source code page (GitHub)
- `i` → Issues page (GitHub)
- `p` → Pull requests page (GitHub)
- `r` → Releases page (GitHub)
- `t` → Tags page (GitHub)
- `v` → Version history page (PyPI)

Omitting the destination takes you to the package page on PyPI as if you used `h`.

## Installation

Install `pypi-jump-to` with `uv`, `pipx`, or `pip`:

```shell
uv tool install pypi-jump-to

pipx install pypi-jump-to

pip install pypi-jump-to
```

## Usage

`pjt httpx` (no specified destination)

🐙 → https://pypi.org/project/httpx

`pjt fastapi d` (documentation)

🐙 → https://fastapi.tiangolo.com

`pjt pydantic r` (releases)

🐙 → https://github.com/samuelcolvin/pydantic/releases

## See Also

This project is inspired by the [`njt`](https://github.com/kachkaev/njt) tool for npm packages.
