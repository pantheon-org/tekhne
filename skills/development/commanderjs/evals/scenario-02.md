# Scenario 02: Design a Multi-Command Project Management CLI

## User Prompt

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

## Expected Behavior

1. Define each subcommand (`publish`, `deprecate`, `info`) in its own dedicated file (e.g., `commands/publish.ts`, `commands/deprecate.ts`, `commands/info.ts`)
2. Create a `commands/index.ts` barrel file that re-exports all three command instances
3. Export each command as a named `Command` instance from its own file (not inline in the main program)
4. Attach subcommands via `.addCommand()` in the main entry point
5. Define TypeScript interfaces or type aliases for at least two of the three subcommand options
6. Constrain the `info` command's `--format` option to `json` and `table` using `.choices()` or `new Option(...).choices([...])`
7. Declare the `deprecate` command's `--reason` option with `.requiredOption()` rather than `.option()` with manual validation
8. Use kebab-case for all multi-word option flags
9. Call `await program.parseAsync(process.argv)` at the entry point
10. Load the program version from `package.json` rather than a hardcoded string literal

## Success Criteria

- **Separate file per command**: Each of the three subcommands is defined in its own dedicated file
- **Barrel export in commands/index.ts**: A `commands/index.ts` barrel file exists that re-exports all three command instances
- **Commands exported as Command instances**: Each command file exports its `Command` instance as a named export
- **addCommand() in main program**: The main entry point uses `.addCommand()` to attach each subcommand
- **TypeScript interfaces defined**: At least two of the three subcommands have a dedicated TypeScript interface or type for their options
- **choices() for format option**: The `info` command's `--format` option uses `.choices()` or `new Option(...).choices([...])` to constrain to `json` and `table`
- **requiredOption for reason**: The `deprecate` command's `--reason` option is declared with `.requiredOption()`
- **kebab-case flags**: All multi-word option flags use kebab-case
- **parseAsync with await**: The main entry point calls `await program.parseAsync(process.argv)`
- **Version from package.json**: The program version is loaded from `package.json` rather than a hardcoded string literal

## Failure Conditions

- Agent defines all subcommands inline in the main program instead of separate files
- Agent omits the `commands/index.ts` barrel file
- Agent defines command instances anonymously instead of as named exports
- Agent uses `program.command()` inline instead of `.addCommand()`
- Agent defines TypeScript interfaces for fewer than two subcommands
- Agent uses `.option('--format <format>')` without `.choices()` for the `info` command
- Agent uses `.option('--reason <reason>')` instead of `.requiredOption()` for the `deprecate` command
- Agent hardcodes the program version as a string literal
