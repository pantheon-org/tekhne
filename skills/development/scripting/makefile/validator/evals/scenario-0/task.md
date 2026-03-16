# Audit and Fix a Project Makefile

## Problem/Feature Description

A data science team has adopted a Makefile to manage their Python project's build, test, and cleanup tasks. A new team member tried to run `make clean` on a machine where someone accidentally created an empty file named `clean` in the project root, and the command was silently skipped. The team's lead suspects there are other lurking issues in the Makefile that could cause similarly confusing behavior. They want a thorough audit of the Makefile, with all issues fixed and a brief explanation of each one.

Review the Makefile below, identify all issues, produce a corrected version, and write a short `audit-notes.md` explaining what was wrong and why it matters.

## Output Specification

Produce:
- A corrected `Makefile` with all issues fixed
- An `audit-notes.md` explaining each issue found and the reason it is problematic

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/Makefile ===============
VENV = .venv
PYTHON = $(VENV)/bin/python

all:
    $(PYTHON) -m pytest tests/

build:
    $(PYTHON) setup.py build

test:
    $(PYTHON) -m pytest tests/ -v

clean:
    rm -rf $(VENV) build/ dist/ *.egg-info

lint:
    $(PYTHON) -m flake8 src/

install:
    pip install -e .

.DEFAULT_GOAL := all
