# Write Tests for a Report Generation CLI

## Problem Description

A team has built a report generation CLI (`report-cli`) and now needs unit tests to verify that the CLI's argument parsing and option handling work correctly before wiring it up to the actual report generation service.

The CLI has a single `generate` command with the following signature:
- Required positional argument: `<report-type>` — the kind of report to generate
- `--output-dir` option: directory to write the report to (defaults to `./reports`)
- `--format` option: output format, restricted to `pdf`, `csv`, or `html` (defaults to `html`)
- `--from` option: start date for the report period
- `--to` option: end date for the report period
- `--verbose` flag: enable debug output

The tests should verify:
1. That valid arguments parse correctly and the service receives them
2. That an invalid `--format` value causes an error
3. That the command can be invoked without optional arguments and defaults apply

Write both the CLI implementation (`cli.ts`) and the test file (`cli.test.ts`). Use bun:test for the test runner. Mock or stub the report generation service — no actual file I/O or service calls are needed.

## Output Specification

Create `cli.ts` and `cli.test.ts`.
