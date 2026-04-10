# Scenario 04: Define CI-Safe Mise Tasks to Replace npm Scripts

## User Prompt

A team wants to move their build automation from npm scripts to Mise tasks so they can use the same commands locally and in CI without depending on `npm run` as an intermediary.

Their current `package.json` scripts:

```json
{
  "scripts": {
    "build": "tsc -p tsconfig.json && cp -r src/assets dist/assets",
    "test": "jest --runInBand --forceExit",
    "lint": "eslint src/ --ext .ts,.tsx --max-warnings 0",
    "clean": "rm -rf dist/ coverage/ .jest-cache/"
  }
}
```

The project uses Node.js 22.4.0 and TypeScript.

Produce a `mise.toml` that:
1. Pins Node.js to 22.4.0
2. Defines all four scripts as Mise tasks with identical behavior
3. Ensures tasks are self-contained (no reliance on implicit shell state)
4. Does not embed any secrets or environment-specific values

## Expected Behavior

1. Pin `node = "22.4.0"` in the `[tools]` section (exact version, not `latest` or a range)
2. Define all four tasks: `build`, `test`, `lint`, and `clean`
3. Match each task's run command to the original npm script (use `tsc`, `jest`, `eslint`, `rm -rf` — not wrapped in `npm run`)
4. Ensure no task relies on a prior `cd` command or unexported environment variable; commands work from the project root
5. Contain no API keys, tokens, or credentials in the `[env]` section

## Success Criteria

- **Node.js 22.4.0 pinned in [tools]**: `mise.toml` has `[tools]` section with `node = "22.4.0"` (exact version, not `latest` or range)
- **All 4 tasks defined**: `mise.toml` defines tasks for `build`, `test`, `lint`, and `clean` — all four scripts are present
- **Task commands match npm script behavior**: Each task run command matches the original npm script (`tsc`, `jest`, `eslint`, `rm -rf`) — not wrapped in `npm run`
- **Tasks are self-contained**: No task relies on a prior `cd` command or unexported environment variable; commands work from the project root
- **No secrets in [env]**: The `mise.toml` does not contain API keys, tokens, or credentials in the `[env]` section

## Failure Conditions

- Uses a floating version specifier or different version for Node.js
- Omits one or more of the four tasks (`build`, `test`, `lint`, `clean`)
- Wraps task commands in `npm run` instead of using the direct tool commands
- Task relies on `cd` or unexported environment state to work correctly
- Includes API keys, tokens, or credentials in the `[env]` section
