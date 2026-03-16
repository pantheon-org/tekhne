# Set Up Automated Makefile Validation in a CI Pipeline

## Problem/Feature Description

A backend team wants to add automated Makefile validation to their GitHub Actions CI pipeline to catch common Makefile issues before they reach production. They've had several incidents where developers committed Makefiles with spaces-instead-of-tabs, missing .PHONY declarations, and once a hardcoded database password that had to be rotated. The team wants the CI check to run on any PR that modifies a Makefile and to fail the PR if there are errors.

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
