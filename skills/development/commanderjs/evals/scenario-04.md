# Scenario 04: Write Tests for a Report Generation CLI

## User Prompt

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

## Expected Behavior

1. Call `program.exitOverride()` in test setup (or a test program factory) so Commander throws instead of calling `process.exit` during tests
2. Use `.choices()` or `new Option(...).choices([...])` to enforce the `pdf`/`csv`/`html` constraint on `--format`
3. Write at least one test that verifies an invalid `--format` value causes an error (thrown exception or rejected promise)
4. Await all calls to `parseAsync` in `cli.test.ts` — no fire-and-forget
5. Define a TypeScript interface or type for the `generate` command's options in `cli.ts`
6. Use kebab-case in option flag definitions in `cli.ts` (e.g., `--output-dir` not `--outputDir`)
7. Write at least one test that invokes the command without optional arguments and asserts that defaults are applied
8. Wrap action handler logic in a `try/catch` block in `cli.ts`
9. Pass the complete options object to the service function, not individual properties

## Success Criteria

- **exitOverride() in test setup**: The test setup (or test program factory) calls `program.exitOverride()` so Commander throws instead of calling `process.exit` during tests
- **choices() for format option**: The `--format` option uses `.choices()` or `new Option(...).choices([...])` to enforce the `pdf`/`csv`/`html` constraint
- **Invalid format test**: At least one test verifies that an invalid `--format` value causes an error
- **parseAsync with await in tests**: In `cli.test.ts`, all calls to `parseAsync` are awaited
- **TypeScript interface for options**: A TypeScript interface or type is defined for the `generate` command's options in `cli.ts`
- **kebab-case flags**: Multi-word options use kebab-case in their flag definition in `cli.ts`
- **Defaults test**: At least one test invokes the command without optional arguments and asserts that defaults are applied
- **try/catch in action handler**: The action handler in `cli.ts` wraps its logic in a `try/catch` block
- **Typed options to service**: The action handler passes the complete options object to the service function, not individual properties

## Failure Conditions

- Agent omits `exitOverride()` in test setup, causing `process.exit` to terminate the test runner
- Agent uses `.option('--format <format>')` without `.choices()` for enum validation
- Agent does not write a test for the invalid `--format` case
- Agent fires `parseAsync` without `await` in tests
- Agent omits a TypeScript interface for the generate command's options
- Agent uses camelCase flags (e.g., `--outputDir`) instead of kebab-case in `cli.ts`
- Agent does not write a defaults test
- Agent omits `try/catch` from the action handler
- Agent passes individual option properties to the service instead of the whole options object
