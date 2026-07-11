# Scenario 04: Discover Conventions from AGENTS.md

## Setup

The project's `AGENTS.md` contains:

```
## Code conventions
- Use named exports only (no default exports)
- No console.log in production code
- JSDoc on all public functions
```

No tooling is configured for these conventions.

## User Prompt

"Can you set up tooling to enforce our conventions from AGENTS.md?"

## Expected Behavior

1. Read `AGENTS.md` (or the user's stated conventions source).
2. Extract each actionable convention:
   - Named exports only → `import/no-default-export`
   - No console.log → `no-console`
   - JSDoc on public functions → `jsdoc/require-jsdoc`
3. Generate or update ESLint config with these rules.
4. Explain each rule and its purpose.

## Success Criteria

- All 3 conventions are mapped to rules.
- The rules are appropriate for the conventions.
- The config is generated/updated.

## Failure Conditions

- Some conventions are missed.
- Incorrect rules are used (e.g., `no-console` for named exports).
- The agent skips reading AGENTS.md and asks the user to repeat conventions.
