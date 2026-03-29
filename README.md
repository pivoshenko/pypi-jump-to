<div align="center">
  <img alt="logo" src="https://github.com/pivoshenko/pypi-jump-to/blob/main/docs/assets/logo.svg?raw=True" height=250>
</div>

<p align="center">
  <a href="https://opensource.org/licenses/MIT">
    <img alt="License" src="https://img.shields.io/pypi/l/pypi-jump-to?style=flat-square&logo=opensourceinitiative&logoColor=white&color=0A6847&label=License">
  </a>
  <a href="https://pypi.org/project/pypi-jump-to">
    <img alt="Python" src="https://img.shields.io/pypi/pyversions/pypi-jump-to?style=flat-square&logo=python&logoColor=white&color=4856CD&label=Python">
  </a>
  <a href="https://pypi.org/project/pypi-jump-to">
    <img alt="PyPI" src="https://img.shields.io/pypi/v/pypi-jump-to?style=flat-square&logo=pypi&logoColor=white&color=4856CD&label=PyPI">
  </a>
  <a href="https://github.com/pivoshenko/pypi-jump-to/actions/workflows/ci.yaml">
    <img alt="CI" src="https://img.shields.io/github/actions/workflow/status/pivoshenko/pypi-jump-to/ci.yaml?label=CI&style=flat-square&logo=githubactions&logoColor=white&color=0A6847">
  </a>
  <a href="https://stand-with-ukraine.pp.ua">
    <img alt="StandWithUkraine" src="https://img.shields.io/badge/Support-Ukraine-FFC93C?style=flat-square&labelColor=07689F">
  </a>
</p>

## Overview

`pypi-jump-to (pjt)` - a quick navigation tool for the PyPI packages. Save five seconds thousands of times by quickly jumping to the right URL:

```shell
pjt <package> [destination]
```

### Features

- **Binary / Zero dependencies**. A single binary with no external dependencies due to the pure Rust core
- **Memory efficient**. Built with Rust for minimal resource usage
- **Lightning fast**. Navigate to any PyPI package destination in seconds
- **Developer productivity**. No more manual URL construction or searching

### Available destinations

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

To install `pypi-jump-to`, you can use `uv` or `pipx` (or `pip` if you prefer):

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

## See also

This project is inspired by the [`njt`](https://github.com/kachkaev/njt) tool for npm packages.
