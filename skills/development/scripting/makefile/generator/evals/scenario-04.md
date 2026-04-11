# Scenario 04: Correct Anti-Patterns in an Existing Makefile

## User Prompt

The following Makefile has been submitted for review. It contains several violations of the makefile-generator skill's anti-patterns and best practices. Identify each issue, explain why it is a problem, and produce a corrected version of the Makefile.

```makefile
CC = gcc
CFLAGS = -Wall -O2
PREFIX = /usr/local
ENVIRONMENT = production

all: build

build:
    gcc -o app main.c server.c

deploy:
    cd dist
    npm publish

release:
    make -C packages/core all
    make -C packages/ui all

clean:
    rm -rf build/

test:
    ./run_tests.sh

install:
    cp app $(PREFIX)/bin/app
```

Your response must:
1. List each anti-pattern found (name it, reference the skill rule it violates)
2. Show the corrected Makefile as a complete file
3. Confirm which validation checklist items the corrected Makefile satisfies

## Expected Behavior

1. Identify that `clean`, `test`, `build`, `deploy`, `release`, `all`, and `install` are missing `.PHONY` and explain that a file named `clean` or `test` would silently block the target
2. Identify that `make -C packages/core all` uses bare `make` instead of `$(MAKE)` and explain that jobserver flags (`-j`, `-n`, `-k`) are not inherited
3. Identify that `cd dist` followed by `npm publish` on separate lines means the `cd` has no effect on the second line
4. Identify that `CC`, `CFLAGS`, and `PREFIX` use `=` instead of `?=` and that this prevents caller override
5. Identify that the `install` target uses `cp` instead of the `install` utility and does not create the destination directory
6. Produce a corrected Makefile declaring all non-file targets in `.PHONY`
7. Replace bare `make` with `$(MAKE)` for recursive calls
8. Chain `cd` and subsequent commands with `&&` or use `.ONESHELL:`
9. List which skill validation checklist items the corrected Makefile satisfies

## Success Criteria

- **Missing .PHONY identified**: Agent identifies missing `.PHONY` and explains that a file named `clean` or `test` would silently block the target
- **Bare make identified**: Agent identifies that `make -C packages/core all` uses bare `make` instead of `$(MAKE)` and explains the jobserver issue
- **Multi-line cd+command anti-pattern identified**: Agent identifies that `cd dist` + `npm publish` on separate lines means the `cd` has no effect on the second line
- **Hardcoded variable assignment identified**: Agent identifies that `CC`, `CFLAGS`, and `PREFIX` use `=` instead of `?=`
- **install uses cp instead of install command**: Agent identifies that the `install` target uses `cp` instead of the `install` utility
- **Corrected Makefile uses .PHONY**: The corrected Makefile declares all non-file targets in `.PHONY`
- **Corrected Makefile uses $(MAKE)**: The corrected Makefile replaces bare `make` with `$(MAKE)` for recursive calls
- **Corrected Makefile fixes cd+command**: The corrected `deploy` target chains `cd` and the following command with `&&` or uses `.ONESHELL:`
- **Validation checklist confirmation**: Agent lists which skill validation checklist items the corrected Makefile satisfies

## Failure Conditions

- Agent misses the missing `.PHONY` issue
- Agent misses the bare `make` anti-pattern
- Agent does not identify the multi-line `cd` + command problem
- Agent does not identify that `=` should be `?=` for user-overridable variables
- Agent does not identify the `cp` vs `install` utility issue
- Agent's corrected Makefile still uses bare `make` instead of `$(MAKE)`
- Agent's corrected Makefile still has `cd` on a separate line from `npm publish`
