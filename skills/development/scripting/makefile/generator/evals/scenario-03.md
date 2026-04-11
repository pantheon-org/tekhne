# Scenario 03: Generate a Python Package Makefile

## User Prompt

You are creating a Makefile for a Python library called `dataparse`. The project uses `pyproject.toml` and `hatch` (or `python -m build`) for packaging. Requirements:

- `develop` target: installs the package in editable mode with dev extras (`pip install -e .[dev]`)
- `test` target: runs `pytest` with coverage
- `lint` target: runs `ruff check .` and `mypy src/`
- `build` target: runs `python -m build`
- `clean` target: removes `dist/`, `build/`, `*.egg-info/`, and `__pycache__` directories
- `help` target: auto-generated from `##` comments
- All targets are phony (none produce files directly)
- Python interpreter is user-overridable: `PYTHON ?= python3`
- Modern GNU Make header required

Produce a `Makefile` for this Python project. Each target recipe must handle multi-step commands correctly (do not rely on separate shell lines for `cd` + command).

## Expected Behavior

1. Begin with the Modern GNU Make header: `SHELL := bash`, `.ONESHELL:`, `.SHELLFLAGS := -eu -o pipefail -c`, `.DELETE_ON_ERROR:`, `.SUFFIXES:`, and both `MAKEFLAGS +=` lines
2. Declare `PYTHON` with `?= python3` so callers can substitute a different interpreter path
3. Declare `develop`, `test`, `lint`, `build`, `clean`, and `help` in `.PHONY`
4. Install in editable mode with dev extras using `pip install -e .[dev]` or equivalent; use the `$(PYTHON)` variable
5. Remove `dist/`, `build/`, `*.egg-info/`, and `__pycache__` directories in the `clean` recipe
6. Chain multi-step commands with `&&` or rely on `.ONESHELL:` from the header — do not use `cd` in one line and a command in the next separate line
7. Place `##` comments above each user-facing target; have the `help` target extract and print them using `grep` or `awk`

## Success Criteria

- **Modern header present**: Makefile begins with the full modern header
- **PYTHON variable uses ?=**: `PYTHON` is declared with `?= python3`
- **.PHONY covers all targets**: `develop`, `test`, `lint`, `build`, `clean`, and `help` are all listed in `.PHONY`
- **develop target correctness**: `develop` recipe installs in editable mode with dev extras using `$(PYTHON)` variable
- **clean target completeness**: `clean` recipe removes `dist/`, `build/`, `*.egg-info/`, and `__pycache__` directories
- **Multi-step recipe safety**: No recipe uses `cd` in one line and a command in the next separate line; commands are chained with `&&` or `.ONESHELL:` is active
- **help target with ## comment extraction**: Each user-facing target has a `##` comment; `help` target extracts and prints them

## Failure Conditions

- Agent omits any line from the Modern GNU Make header
- Agent uses `=` instead of `?=` for the `PYTHON` variable
- Agent omits any target from `.PHONY`
- Agent does not use the `$(PYTHON)` variable in the `develop` recipe
- Agent omits one or more directories from the `clean` recipe
- Agent uses `cd` in one recipe line followed by a command in the next line (multi-step shell bug)
- Agent does not use `##` comments or does not implement the `help` target
