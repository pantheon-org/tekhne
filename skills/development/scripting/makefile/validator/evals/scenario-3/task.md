# Fix a Monorepo Root Makefile

## Problem/Feature Description

A platform team manages a monorepo with multiple sub-projects, each with its own Makefile. The root Makefile orchestrates builds across all sub-projects. Recently, developers running `make -j4` from the root reported corrupted build artifacts and cryptic error messages. The team lead suspects the root Makefile is not properly propagating build settings to sub-project builds. Additionally, one target that runs a multi-step shell script intermittently fails midway through but Make continues to the next step instead of aborting.

Review the root Makefile below, identify all issues contributing to these problems, produce a corrected version, and write a `fix-notes.md` explaining each change.

## Output Specification

Produce:
- A corrected `Makefile`
- A `fix-notes.md` documenting each issue and the fix applied

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/Makefile ===============
SERVICES = auth payment notification

.PHONY: all build test deploy clean

all: build test

build:
	@for svc in $(SERVICES); do \
		echo "Building $$svc"; \
		make -C services/$$svc build; \
	done

test:
	@for svc in $(SERVICES); do \
		echo "Testing $$svc"; \
		make -C services/$$svc test; \
	done

deploy: build test
	@for svc in $(SERVICES); do \
		make -C services/$$svc deploy; \
	done

setup-env:
	cp .env.template .env
	echo "DB_URL=postgres://localhost/myapp" >> .env
	echo "REDIS_URL=redis://localhost:6379" >> .env
	chmod 600 .env

clean:
	@for svc in $(SERVICES); do \
		make -C services/$$svc clean; \
	done
	rm -rf dist/
