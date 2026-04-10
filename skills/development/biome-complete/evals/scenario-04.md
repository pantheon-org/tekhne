# Scenario 04: Troubleshoot biome check Failures After ESLint Migration

## User Prompt

A team just completed an ESLint-to-Biome migration. They removed ESLint, added Biome, and their `biome.json` enables all recommended rules. Now `bunx @biomejs/biome check .` reports 47 errors across the codebase and CI is blocked.

Their current `biome.json`:

```json
{
  "$schema": "https://biomejs.dev/schemas/1.9.0/schema.json",
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": true
    }
  },
  "formatter": {
    "enabled": true,
    "indentStyle": "space",
    "indentWidth": 2
  }
}
```

They need a strategy to identify which rule groups are causing failures so they can address them one at a time rather than disabling everything or suppressing all 47 errors at once.

Produce a `TRIAGE.md` file that:
1. Describes the isolation strategy (which biome.json changes to make first)
2. Shows an example `biome.json` with specific rule groups temporarily disabled to narrow down the source
3. Explains the re-enable-one-at-a-time approach
4. Notes what to do once the offending group is found

## Expected Behavior

1. Describe temporarily disabling specific rule groups (not all linting) to identify which group causes failures
2. Provide a `biome.json` example with specific rule groups (e.g., `suspicious`, `correctness`, `style`) set to `off` or `warn` — not just `recommended: false`
3. Explain the process of re-enabling one rule group at a time and running `biome check` after each change
4. Describe the outcome action once the offending group is found (fix violations, add targeted suppressions, or configure specific rules)
5. Avoid recommending blanket suppression or disabling all linting as the solution

## Success Criteria

- **Isolation strategy described**: `TRIAGE.md` explains the approach of temporarily disabling rule groups to identify which group causes failures
- **Example biome.json shows group-level disabling**: `TRIAGE.md` includes a `biome.json` example that sets specific rule groups (e.g., `suspicious`, `correctness`, `style`) to `off` or `warn` — not just `recommended: false`
- **Re-enable one at a time approach explained**: `TRIAGE.md` describes re-enabling one rule group at a time and running `biome check` after each change to isolate the failing group
- **Outcome action described**: `TRIAGE.md` explains what to do once the offending group is found (fix violations, add targeted suppressions, or configure specific rules within that group)
- **No blanket suppression recommended**: `TRIAGE.md` does not suggest disabling all linting or using blanket suppressions as the solution

## Failure Conditions

- Agent recommends disabling all linting (`"linter": { "enabled": false }`) as the solution
- Agent produces a `biome.json` example using only `recommended: false` rather than group-level disabling
- Agent omits the re-enable-one-at-a-time approach
- Agent does not describe what to do after identifying the offending rule group
- Agent recommends blanket `biome-ignore` suppressions across all 47 errors
