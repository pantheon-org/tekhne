# Scenario 03: Fix a Monorepo Root Makefile

## User Prompt

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

## Expected Behavior

1. Replace bare `make -C` calls in the `build` target with `$(MAKE) -C`
2. Replace bare `make -C` calls in the `test` target with `$(MAKE) -C`
3. Replace bare `make -C` calls in the `deploy` target with `$(MAKE) -C`
4. Replace bare `make -C` calls in the `clean` target with `$(MAKE) -C`
5. Explain in `fix-notes.md` that bare `make` does not inherit jobserver flags and breaks parallel builds with `-j`
6. Add `.ONESHELL` OR explain in `fix-notes.md` why `.ONESHELL` would prevent the multi-step `setup-env` script from continuing past failures
7. Explain in `fix-notes.md` how `.ONESHELL` affects multi-step recipe error handling
8. Ensure all four targets (build, test, deploy, clean) that contained bare `make -C` calls are updated to `$(MAKE) -C`

## Success Criteria

- **$(MAKE) in build target**: The corrected `build` target uses `$(MAKE) -C` instead of bare `make -C`
- **$(MAKE) in test target**: The corrected `test` target uses `$(MAKE) -C` instead of bare `make`
- **$(MAKE) in deploy target**: The corrected `deploy` target uses `$(MAKE) -C` instead of bare `make`
- **$(MAKE) in clean target**: The corrected `clean` target uses `$(MAKE) -C` instead of bare `make`
- **Bare make risk explained**: `fix-notes.md` explains that bare `make` does not inherit jobserver flags and breaks parallel builds with `-j`
- **.ONESHELL or equivalent addressed**: The corrected Makefile adds `.ONESHELL` OR `fix-notes.md` explains that `.ONESHELL` would prevent the multi-step script from continuing past failures
- **.ONESHELL benefit explained**: `fix-notes.md` explains how `.ONESHELL` affects multi-step recipe error handling
- **All recursive calls fixed**: All four targets that contained bare `make -C` calls are updated to `$(MAKE) -C`

## Failure Conditions

- Agent leaves any `make -C` (bare `make`) call in the corrected Makefile
- Agent does not explain that bare `make` fails to inherit jobserver flags
- Agent does not address `.ONESHELL` in either the corrected Makefile or `fix-notes.md`
- Agent does not explain the `.ONESHELL` benefit for multi-step recipe reliability
