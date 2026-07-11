# Scenario 03: Map TypeScript Strictness to tsconfig.json

## User Prompt

"Our standard says: enable strict null checks and no implicit any. Can you update the TypeScript config?"

## Setup

The project has a `tsconfig.json` already with `"strict": false` but no specific strictness options.

## Expected Behavior

1. Recognize that `strict: true` is the idiomatic way to enable both `strictNullChecks` and `noImplicitAny` (and other strict checks).
2. Recommend enabling `strict: true` rather than setting individual flags, unless a specific flag needs to be disabled.
3. If the project needs to opt-out of specific strict checks, set `strict: true` and disable the exception explicitly.
4. Update the `tsconfig.json`.
5. Explain the change.

## Success Criteria

- `strict: true` is set (or both individual flags are set with justification).
- The change is explained.
- Existing non-conflicting settings are preserved.

## Failure Conditions

- Only one flag is enabled when both were requested.
- `strict: false` remains alongside individual strict flags (which is ineffective).
- No explanation is given for the change.
