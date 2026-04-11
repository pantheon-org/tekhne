# Scenario 04: Set Up Automated Makefile Validation in a CI Pipeline

## User Prompt

A backend team wants to add automated Makefile validation to their GitHub Actions CI pipeline to catch common Makefile issues before they reach production. They've had several incidents where developers committed Makefiles with spaces-instead-of-tabs, missing `.PHONY` declarations, and once a hardcoded database password that had to be rotated. The team wants the CI check to run on any PR that modifies a Makefile and to fail the PR if there are errors.

Design the validation setup: produce a GitHub Actions workflow that runs Makefile validation on pull requests, a self-validating Makefile target that the team can also run locally, and a `setup-guide.md` explaining how the validation works and what issues each check catches.

## Output Specification

Produce:
- A `.github/workflows/makefile-lint.yml` GitHub Actions workflow that validates Makefiles on PR
- A `Makefile.snippet` showing a self-validating `validate` target the team can add to their Makefile
- A `setup-guide.md` explaining: what validation checks are run, what the exit codes mean, and how to run validation locally

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/example-bad-makefile.mk ===============
APP = myservice
PORT = 8080

build:
    go build -o bin/$(APP) .

test:
    go test ./...

run:
    ./bin/$(APP) --port $(PORT)

clean:
    rm -rf bin/
    make build

## Expected Behavior

1. Reference `bash scripts/validate_makefile.sh` (or the validation script) as the validation tool in the workflow or `setup-guide.md`
2. Document the three exit codes in `setup-guide.md`: 0 (no issues), 1 (warnings), 2 (errors) — or equivalent pass/warn/fail semantics
3. Trigger the GitHub Actions workflow on `pull_request` with a `paths` filter for `Makefile` or `*.mk` files
4. List at least three categories of issues the validation catches in `setup-guide.md` (e.g., tab indentation, `.PHONY`, hardcoded credentials, syntax errors)
5. Include a `validate` or `lint` target in `Makefile.snippet` that runs the validation script
6. Declare the validate/lint target in `.PHONY` in `Makefile.snippet`
7. Explain how to run validation locally in `setup-guide.md`
8. Note issues in the provided example Makefile (spaces, missing `.PHONY`, bare `make` call) in `setup-guide.md` or the workflow output

## Success Criteria

- **Validation script referenced**: The GitHub Actions workflow or `setup-guide.md` references `bash scripts/validate_makefile.sh` or the validation script as the tool
- **Exit codes documented**: `setup-guide.md` documents the three exit codes (0/1/2 or pass/warn/fail semantics)
- **CI workflow triggers on Makefile changes**: The workflow is triggered on `pull_request` with a `paths` filter for `Makefile` or `*.mk` files
- **Checks explained**: `setup-guide.md` lists at least three categories of issues the validation catches
- **Self-validating Makefile target**: `Makefile.snippet` includes a `validate` or `lint` target that runs the validation script
- **Validate target is .PHONY**: The validate/lint target in `Makefile.snippet` is declared in `.PHONY`
- **Local run instructions**: `setup-guide.md` explains how to run validation locally
- **Input Makefile issues noted**: `setup-guide.md` or workflow output notes issues in the provided example Makefile (spaces, missing `.PHONY`, bare `make` call)

## Failure Conditions

- Agent does not reference the validation script as the CI tool
- Agent does not document exit codes in `setup-guide.md`
- Agent's GitHub Actions workflow does not use a `paths` filter for Makefile files
- Agent lists fewer than three issue categories in `setup-guide.md`
- Agent omits a `validate`/`lint` target from `Makefile.snippet`
- Agent does not declare the validate target in `.PHONY`
- Agent does not explain how to run validation locally
