# Design a Multi-Command Project Management CLI

## Problem Description

A developer tools company is building `pkgctl`, a CLI for managing internal packages across a monorepo. The tool needs three top-level subcommands:

- `publish` — publish a package to the internal registry. Options: `--tag` (defaults to `latest`), `--dry-run` flag.
- `deprecate` — mark a package version as deprecated. Options: `--reason` (required text), `--version` specifying the version to deprecate.
- `info` — display metadata about a package. Options: `--format` accepting either `json` or `table` (defaults to `table`).

All three subcommands take a required positional argument: the package name.

The CLI will grow significantly over time, so the architect wants the code structured so that each subcommand lives in its own file and the main entry point stays minimal. Service logic stubs should be separate from command wiring.

Produce the complete file set for the CLI. Write all TypeScript files and list the expected directory structure in a `structure.txt` file.

## Output Specification

Create the following files:
- `structure.txt` — the directory/file layout you chose
- All TypeScript source files implementing the CLI
