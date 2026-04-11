# Scenario 05: Generate a Multi-Directory C++ Project Makefile

## User Prompt

You are building a top-level Makefile for a C++ project with three sub-components:

```
engine/
  src/   (produces libengine.a)
renderer/
  src/   (produces librenderer.a, depends on libengine.a)
app/
  src/   (produces bin/game, links libengine.a and librenderer.a)
```

Top-level Makefile requirements:
- Use `$(MAKE)` for all recursive calls into sub-directories, never bare `make`
- Build order: engine first, then renderer, then app (explicit dependency ordering)
- `all` builds everything in correct order
- `clean` calls `$(MAKE) clean` in each sub-directory
- `test` calls `$(MAKE) test` in each sub-directory
- `help` target present with `##` comments

For the `app/` component Makefile (produce this as `app/Makefile`):
- Compiler: `CXX ?= g++`
- Source glob: `$(wildcard src/*.cpp)`
- Object files in `build/` directory using order-only prerequisite `| $(BUILDDIR)`
- Dependency tracking: `-MMD -MP` and `-include` of generated `.d` files
- Modern GNU Make header

Produce both the top-level `Makefile` and `app/Makefile`.

## Expected Behavior

1. Use `$(MAKE) -C <dir>` for all recursive sub-directory calls in the top-level Makefile — never bare `make`
2. Build `engine` before `renderer` before `app` in the top-level `all` target, either via explicit prerequisites or sequential `$(MAKE)` calls
3. Include the full modern header in `app/Makefile` (`SHELL := bash`, `.ONESHELL:`, `.SHELLFLAGS := -eu -o pipefail -c`, `.DELETE_ON_ERROR:`, `.SUFFIXES:`, both `MAKEFLAGS +=` lines)
4. Use `| $(BUILDDIR)` as an order-only prerequisite in the `app/Makefile` object pattern rule; the `BUILDDIR` target creates the directory
5. Include `-MMD -MP` in the compile rule and `-include` the generated `.d` files in `app/Makefile`
6. Declare all non-file targets in `.PHONY` in both the top-level Makefile and `app/Makefile`

## Success Criteria

- **Top-level uses $(MAKE) exclusively**: All recursive sub-directory calls in the top-level Makefile use `$(MAKE) -C <dir>`, never bare `make`
- **Build order enforced**: The top-level `all` target builds `engine` before `renderer` before `app`
- **app/Makefile modern header**: `app/Makefile` includes the full modern header
- **Order-only prerequisite for build directory**: `app/Makefile` object pattern rule uses `| $(BUILDDIR)` as an order-only prerequisite
- **Dependency tracking in app/Makefile**: `app/Makefile` compile rule includes `-MMD -MP` and the Makefile includes `.d` files with `-include`
- **.PHONY in both files**: Both top-level Makefile and `app/Makefile` declare all non-file targets in `.PHONY`

## Failure Conditions

- Agent uses bare `make -C <dir>` instead of `$(MAKE) -C <dir>` anywhere in the top-level Makefile
- Agent does not enforce the engine → renderer → app build order
- Agent omits any line from the Modern GNU Make header in `app/Makefile`
- Agent uses inline `mkdir -p` instead of an order-only prerequisite in `app/Makefile`
- Agent omits `-MMD -MP` or the `-include` of generated `.d` files from `app/Makefile`
- Agent omits `.PHONY` from either file
