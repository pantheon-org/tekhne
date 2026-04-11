# Scenario 01: Scaffold a Deployment CLI with Service Layer

## User Prompt

An infrastructure team is building a deployment CLI for their microservices platform. The tool needs two subcommands: `deploy` and `rollback`. Both will eventually call into complex deployment orchestration services, but for now the service functions can be stubs that log their inputs.

The `deploy` command should accept:
- A required positional argument: the name of the service to deploy
- `--env` option: target environment (defaults to `staging`)
- `--branch` option: git branch to deploy (defaults to `main`)
- `--dry-run` flag: simulate the deployment without making changes

The `rollback` command should accept:
- A required positional argument: the name of the service to roll back
- `--steps` option: number of versions to roll back (defaults to `1`)
- `--force` flag: skip confirmation checks

The team has strict TypeScript standards and wants the code structured so that it's easy to unit-test the deployment logic independently of Commander.js.

Produce the full TypeScript source in a single file `cli.ts`. Service functions should be defined in the same file as named functions or constants for simplicity.

## Output Specification

Create `cli.ts` with the complete implementation.

## Expected Behavior

1. Define a TypeScript interface (or type alias) for the deploy command's options (covering `env`, `branch`, `dryRun` or equivalent)
2. Define a TypeScript interface (or type alias) for the rollback command's options (covering `steps`, `force` or equivalent)
3. Pass the complete options object to the deploy service function — not individual properties
4. Pass the complete options object to the rollback service function — not individual properties
5. Accept the options parameter as a typed object in service function signatures
6. Attach both subcommands via `.addCommand()`
7. Use kebab-case option flags (e.g., `--dry-run` not `--dryRun`)
8. Call `await program.parseAsync(process.argv)` at the entry point

## Success Criteria

- **TypeScript interface for deploy options**: A TypeScript interface (or type alias) is defined for the deploy command's options (covering `env`, `branch`, `dryRun` or equivalent)
- **TypeScript interface for rollback options**: A TypeScript interface (or type alias) is defined for the rollback command's options (covering `steps`, `force` or equivalent)
- **Complete options object passed to deploy service**: The deploy action handler passes the entire options object to its service function, NOT individual properties
- **Complete options object passed to rollback service**: The rollback action handler passes the entire options object to its service function, NOT individual properties
- **Service function signature accepts typed object**: The deploy or rollback service function signature accepts the options as a typed parameter (e.g., `options: DeployOptions`), not as individual string/boolean parameters
- **addCommand() used**: The main program uses `.addCommand()` to attach the deploy and rollback subcommands
- **kebab-case option flags**: Multi-word options use kebab-case in their flag definition (e.g., `--dry-run` not `--dryRun`)
- **parseAsync with await**: The entry point calls `await program.parseAsync(process.argv)`

## Failure Conditions

- Agent omits TypeScript interfaces for deploy or rollback options
- Agent destructures the options object and passes individual properties to service functions
- Agent defines service functions accepting individual parameters instead of a typed options object
- Agent uses `program.command('deploy')` inline instead of `addCommand()`
- Agent uses camelCase flags in option definitions (e.g., `--dryRun` instead of `--dry-run`)
- Agent uses `program.parse()` instead of `await program.parseAsync()`
