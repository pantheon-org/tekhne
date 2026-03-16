# Scaffold a Deployment CLI with Service Layer

## Problem Description

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
