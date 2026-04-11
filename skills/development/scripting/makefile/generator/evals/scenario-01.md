# Scenario 01: Generate a C Project Makefile

## User Prompt

You are building a C application called `httpd` for a project with the following layout:

```
httpd/
├── src/
│   ├── main.c
│   ├── server.c
│   └── router.c
├── include/
│   └── httpd.h
└── tests/
    └── test_router.c
```

Requirements:
- Compiler: GCC, user-overridable
- Compile flags: `-Wall -Wextra -O2`, user-overridable
- Output binary: `build/httpd`
- Dependencies: auto-detected via `-MMD -MP` flags
- Standard targets: `all`, `install`, `clean`, `test`, `help`
- `install` copies the binary to `$(PREFIX)/bin` (PREFIX defaults to `/usr/local`)
- `help` prints available targets using `##` comment extraction
- Install prefix must be user-overridable with `?=`
- Use the Modern GNU Make header (not POSIX-compatible header)
- Use order-only prerequisites for the build directory

Produce a single `Makefile` that satisfies all the requirements above. Include a validation checklist at the end as a comment block showing which items were verified.

## Expected Behavior

1. Begin with the Modern GNU Make header: `SHELL := bash`, `.ONESHELL:`, `.SHELLFLAGS := -eu -o pipefail -c`, `.DELETE_ON_ERROR:`, `.SUFFIXES:`, and both `MAKEFLAGS +=` lines
2. Declare all non-file targets (`all`, `install`, `clean`, `test`, `help`) in a single `.PHONY` line
3. Use `?=` for user-overridable variables (`CC`, `CFLAGS`, `PREFIX`, `DESTDIR`); use `:=` for project-internal variables (`SOURCES`, `OBJECTS`)
4. Include `-MMD -MP` in the compile rule and include generated `.d` files with `-include $(OBJECTS:.o=.d)` or equivalent
5. Use `| $(BUILDDIR)` order-only prerequisite in the object file pattern rule instead of inline `mkdir -p`
6. Use the `install` command (not `cp`) in the install target, create the target directory with `install -d`, apply correct mode (`-m 755`), and respect `DESTDIR`
7. Place `##` comments above each user-facing target; have the `help` target extract and print them using `grep` or `awk`
8. Use hard tab characters (not spaces) for all recipe line indentation
9. Include a comment block at the end listing which validation checklist items were verified

## Success Criteria

- **Modern header present**: Makefile begins with `SHELL := bash`, `.ONESHELL:`, `.SHELLFLAGS := -eu -o pipefail -c`, `.DELETE_ON_ERROR:`, `.SUFFIXES:`, and both `MAKEFLAGS +=` lines
- **.PHONY declaration**: All non-file targets are declared in a single `.PHONY` line
- **User-overridable variables use ?=**: `CC`, `CFLAGS`, `PREFIX`, and `DESTDIR` use `?=`; project-internal variables use `:=`
- **Automatic dependency tracking**: Compile rule includes `-MMD -MP` flags and the Makefile includes generated `.d` files
- **Order-only prerequisite for build directory**: Object file pattern rule uses `| $(BUILDDIR)` order-only prerequisite
- **install target correctness**: `install` target uses the `install` command (not `cp`), creates the target directory with `install -d`, applies `-m 755`, and respects `DESTDIR`
- **help target with ## comments**: Each user-facing target has a `##` comment; the `help` target extracts and prints them
- **Tab indentation in recipes**: All recipe lines are indented with tab characters, not spaces
- **Validation checklist comment**: The Makefile includes a comment block at the end listing which validation checklist items were verified

## Failure Conditions

- Agent omits any line from the Modern GNU Make header
- Agent omits `.PHONY` or leaves non-file targets undeclared
- Agent uses `=` instead of `?=` for user-overridable variables
- Agent omits `-MMD -MP` or the `-include` of generated `.d` files
- Agent uses inline `mkdir -p` instead of an order-only prerequisite
- Agent uses `cp` instead of the `install` command in the install target
- Agent does not extract `##` comments in the `help` target
- Agent uses spaces instead of tabs for recipe indentation
