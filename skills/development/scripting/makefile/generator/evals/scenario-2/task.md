# Task: Generate a Go + Docker Makefile

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
