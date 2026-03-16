# Define CI-Safe Mise Tasks to Replace npm Scripts

## Problem Description

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
