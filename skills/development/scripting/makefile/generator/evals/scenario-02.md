# Scenario 02: Generate a Go + Docker Makefile

## User Prompt

You are maintaining a Go microservice called `payments-svc`. The project uses Go modules and needs:

- Binary output: `build/payments-svc`
- Version embedding at build time via `-ldflags "-X main.Version=$(VERSION)"` where `VERSION` defaults to `0.1.0`
- Git commit hash embedded similarly: `-X main.Commit=$(shell git rev-parse --short HEAD)`
- Docker targets: `docker-build`, `docker-push`, `docker-run`
- Docker image name: `$(REGISTRY)/payments-svc:$(VERSION)` where `REGISTRY ?= ghcr.io/myorg`
- `docker-push` depends on `docker-build` and must have error handling for both the versioned tag and the `:latest` tag
- `docker-push` must not be parallelizable with `docker-build`
- No credentials or tokens in the Makefile
- Standard targets: `all`, `build`, `test`, `clean`, `help`
- Modern GNU Make header required

Produce a `Makefile` for this project.

## Expected Behavior

1. Begin with the Modern GNU Make header: `SHELL := bash`, `.ONESHELL:`, `.SHELLFLAGS := -eu -o pipefail -c`, `.DELETE_ON_ERROR:`, `.SUFFIXES:`, and both `MAKEFLAGS +=` lines
2. Pass `-ldflags` with `-X` flags embedding both `VERSION` and the git commit hash using `$(shell git rev-parse --short HEAD)` in the build recipe
3. Declare `REGISTRY` with `?=` so callers can override it; declare `VERSION` with `?=` or make it overridable
4. List `docker-build` as a prerequisite of `docker-push` so build completes before push
5. Use `|| { echo ...; exit 1; }` or equivalent error handling for both the versioned and `:latest` tag push operations
6. Use `.NOTPARALLEL` or dependency ordering to prevent parallel execution of `docker-build` and `docker-push`
7. Include no tokens, passwords, or secret values in the Makefile; delegate authentication to environment variables or external tooling
8. Declare all non-file targets including `docker-build`, `docker-push`, `docker-run`, `all`, `build`, `test`, `clean`, and `help` in `.PHONY`

## Success Criteria

- **Modern header present**: Makefile begins with the full modern header
- **Version and commit ldflags**: The build recipe passes `-ldflags` with `-X` flags embedding both `VERSION` and the git commit hash
- **REGISTRY variable uses ?=**: `REGISTRY` is declared with `?=`; `VERSION` also uses `?=` or is overridable
- **docker-push depends on docker-build**: `docker-push` lists `docker-build` as a prerequisite
- **docker-push error handling**: `docker-push` recipe uses `|| { echo ...; exit 1; }` or equivalent for both push operations
- **Parallel safety for Docker targets**: `.NOTPARALLEL` is declared or dependency ordering prevents parallel `docker-build` and `docker-push`
- **No hardcoded credentials**: Makefile contains no tokens, passwords, or secret values
- **.PHONY completeness**: All non-file targets including Docker targets are listed in `.PHONY`

## Failure Conditions

- Agent omits any line from the Modern GNU Make header
- Agent omits `-ldflags` version embedding or the git commit hash from the build recipe
- Agent hardcodes `REGISTRY` or `VERSION` with `=` instead of `?=`
- Agent does not list `docker-build` as a prerequisite of `docker-push`
- Agent does not add error handling to the `docker-push` recipe
- Agent hardcodes credentials or tokens in the Makefile
- Agent omits Docker targets from `.PHONY`
