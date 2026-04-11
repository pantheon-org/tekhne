# Scenario 00: Build a File Conversion CLI Tool

## User Prompt

A data engineering team needs a small command-line tool to convert JSON files to CSV format. The tool will be used in shell scripts and automated pipelines, so correct exit codes are important. When the conversion succeeds the tool should exit with code 0; on any error it should exit with a non-zero code and print a useful message.

The tool should accept a required input file path and an optional output file path (defaulting to the input filename with a `.csv` extension). It should also support a `--verbose` flag for debug output during development.

The conversion logic itself can be a stub — just write the structure of the tool with correct argument handling, error management, and program entry point. The team will fill in the actual JSON-to-CSV logic later.

Write the complete TypeScript source for the tool in a file called `cli.ts`. The tool should be runnable with `bun cli.ts <input> [output]`.

## Output Specification

Create `cli.ts` with the full CLI implementation. The file should be executable as a bun script.

## Expected Behavior

1. Call `program.parseAsync()` (not `program.parse()`) at the entry point
2. Precede the `parseAsync` call with `await`
3. Declare the required input file argument using `.argument('<input>', ...)` — do not read from `process.argv` directly
4. Declare the optional output file argument using `.argument('[output]', ...)`
5. Wrap action handler logic in a `try/catch` block
6. Exit with a non-zero code (e.g., `process.exit(1)`) or call `program.error()` in the catch block
7. Declare the action handler as `async`

## Success Criteria

- **parseAsync used**: The entry point calls `program.parseAsync()` (not `program.parse()`)
- **await on parseAsync**: The call to `parseAsync` is preceded by `await`
- **No direct process.argv access**: The code does NOT directly read `process.argv[2]` or similar indices to extract argument values; all arguments go through Commander.js `.argument()` declarations
- **try/catch in action handler**: The async action handler wraps its logic in a `try/catch` block
- **Error handling with exit code**: The catch block exits with a non-zero code (e.g., `process.exit(1)`) or calls `program.error()`
- **Async action handler**: The action handler is declared as `async`
- **Required argument declared**: The input file is declared as a required argument using `.argument('<input>', ...)` with angle brackets
- **Optional argument declared**: The output file is declared as an optional argument using `.argument('[output]', ...)` with square brackets

## Failure Conditions

- Agent uses `program.parse()` instead of `program.parseAsync()`
- Agent calls `parseAsync` without `await`
- Agent reads `process.argv[2]` or similar indices directly instead of using `.argument()` declarations
- Agent omits `try/catch` in the action handler
- Agent does not call `process.exit(1)` or `program.error()` in the error path
- Agent declares the action handler as synchronous instead of `async`
- Agent uses `[input]` (square brackets) for the required argument instead of `<input>` (angle brackets)
- Agent uses `<output>` (angle brackets) for the optional argument instead of `[output]` (square brackets)
