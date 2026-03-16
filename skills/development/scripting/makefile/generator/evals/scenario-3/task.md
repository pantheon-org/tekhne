# Task: Generate a Python Package Makefile

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
