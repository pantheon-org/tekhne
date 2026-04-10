# Scenario 00: Audit and Fix a Project Makefile

## User Prompt

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

## Expected Behavior

1. Add a `.PHONY` declaration listing the non-file targets (`all`, `build`, `test`, `clean`, `lint`, `install`)
2. Explain in `audit-notes.md` that without `.PHONY`, a file named `clean` (or similar) causes Make to silently skip the target
3. Correct recipe lines to use hard tab characters instead of spaces
4. Explain in `audit-notes.md` that spaces instead of tabs cause `missing separator` errors in GNU Make
5. Address or note the unquoted `$(VENV)` variable in the `rm -rf` command (or quote it in the corrected Makefile)
6. Ensure `.PHONY` includes at least `clean` and `test`
7. Reference using `make -n` or a validation tool to detect the issues

## Success Criteria

- **.PHONY declared**: The corrected Makefile includes a `.PHONY` declaration listing the non-file targets
- **.PHONY issue explained**: `audit-notes.md` explains that without `.PHONY`, a file named `clean` would cause Make to silently skip the target
- **Tab indentation in recipes**: The corrected Makefile uses hard tab characters at the start of recipe lines
- **Tab issue explained**: `audit-notes.md` explains that spaces instead of tabs cause `missing separator` errors
- **rm unquoted variable noted**: `audit-notes.md` identifies or the corrected Makefile quotes the `$(VENV)` variable in `rm -rf`
- **All non-file targets covered**: The `.PHONY` declaration includes at least `clean` and `test`
- **Validation approach mentioned**: `audit-notes.md` or the agent output references using `make -n` or a validation tool

## Failure Conditions

- Agent does not add `.PHONY` to the corrected Makefile
- Agent does not explain the `.PHONY` issue in `audit-notes.md`
- Agent produces a corrected Makefile with spaces instead of tabs in recipe lines
- Agent does not explain the tab indentation issue in `audit-notes.md`
- Agent does not mention or fix the unquoted `$(VENV)` variable
- Agent omits `clean` and `test` from the `.PHONY` declaration
