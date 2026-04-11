# Scenario 04: Create AGENTS.md for pnpm Monorepo

## User Prompt

Your team has set up a new monorepo using pnpm workspaces for a React application with a shared component library. The previous developer started creating AGENTS.md but left incomplete and possibly incorrect commands.

The tech lead has asked you to complete the AGENTS.md documentation, ensuring all commands are correct and validated. This is critical because broken commands in documentation have caused deployment issues in the past.

## Output Specification

Create/finish `AGENTS.md` with validated commands. The documentation should include:

- How to install dependencies
- How to run the application in development
- How to build packages
- How to run tests
- How to lint

IMPORTANT: Verify each command works correctly before including it. The tech lead will test every command by copy-pasting.

## Expected Behavior

1. Inspect the monorepo config files (`pnpm-workspace.yaml`, `package.json`, workspace packages) before writing or completing any commands
2. Use `pnpm` commands throughout — never substitute `npm` or `yarn` for a pnpm workspace
3. Validate each command against the project structure before including it: check test runner, build system, and lint config
4. Produce a complete AGENTS.md covering install, dev server, build, test, and lint — with correct flags and paths
5. Ensure no command would fail when copy-pasted verbatim by the tech lead

## Success Criteria

- **Command validation performed**: Agent validated that commands work before including them (evidence of checking)
- **Correct package manager**: Uses correct package manager commands (e.g., pnpm for pnpm workspace, not npm)
- **Valid test command**: Test command matches project's test setup (e.g., correct test runner, proper flags)
- **Valid build command**: Build command is valid for the project's build system
- **Valid lint command**: Lint command uses configured linter with correct paths
- **No broken commands**: No commands that would fail when copy-pasted (no typos, correct flags)

## Failure Conditions

- Uses `npm install` or `yarn` in a pnpm workspace project
- Includes test commands referencing a test runner not configured in the project
- Provides build commands that don't match the workspace's build system
- Includes lint commands with incorrect paths or flags that would fail
- Copies commands from a template or assumes defaults without checking project config
- Delivers a AGENTS.md with any command that would produce an error when run verbatim
