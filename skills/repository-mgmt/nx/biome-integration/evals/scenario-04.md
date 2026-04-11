# Scenario 04: Add Scoped Migration Override to biome.json for a Legacy Directory

## User Prompt

You are migrating a large Nx monorepo to Biome. The `apps/legacy-portal` directory contains a lot of TypeScript files that use non-null assertions heavily. Rather than globally disabling the `noNonNullAssertion` rule, you need to add a scoped, time-boxed override for just that directory.

Produce an updated `biome.json` that adds this scoped override.

## Requirements

- Keep the existing root-level formatter and linter configuration intact (formatter enabled, 2-space spaces, linter enabled, recommended rules).
- Add an `overrides` array with one entry that:
  - Matches files under `apps/legacy-portal/**` only.
  - Sets `linter.rules.style.noNonNullAssertion` to `"off"` within that scope.
- Do NOT disable any rules at the root level.

## Starting biome.json

```json
{
  "$schema": "https://biomejs.dev/schemas/1.9.4/schema.json",
  "files": {
    "ignoreUnknown": false
  },
  "formatter": {
    "enabled": true,
    "indentStyle": "space",
    "indentWidth": 2
  },
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": true
    }
  }
}
```

## Output

Produce the updated file `biome.json`.

## Expected Behavior

1. Retain root-level `formatter` settings (`enabled`, `indentStyle`, `indentWidth`) unchanged
2. Retain root-level `linter.enabled` and `linter.rules.recommended` unchanged
3. Add an `overrides` array to the file
4. The override entry's `includes` (or `include`) field targets `apps/legacy-portal/**`
5. The override entry sets `linter.rules.style.noNonNullAssertion` to `"off"` — and only within the override, not at root level

## Success Criteria

- **Root formatter config unchanged**: Root-level formatter settings (`enabled`, `indentStyle`, `indentWidth`) remain exactly as in the starting file.
- **Root linter config unchanged**: Root-level `linter.enabled` and `linter.rules.recommended` remain as in the starting file. No rules are disabled at root level.
- **overrides array present**: The output `biome.json` contains an `overrides` array with at least one entry.
- **Override includes correct path pattern**: The override entry's `includes` field targets `apps/legacy-portal` (e.g. `"apps/legacy-portal/**"`).
- **Override disables noNonNullAssertion only within the scoped override**: The override entry sets `linter.rules.style.noNonNullAssertion` to `"off"`. This rule is NOT set at the root `linter.rules` level.

## Failure Conditions

- Root formatter settings (`enabled`, `indentStyle`, or `indentWidth`) are changed from the starting values
- Root `linter.enabled` or `linter.rules.recommended` is changed, or any rule is disabled at root level
- The `overrides` array is absent from the output
- No override entry targets `apps/legacy-portal`
- `noNonNullAssertion` is disabled globally at root level instead of only inside the scoped override
