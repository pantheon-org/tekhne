# Task: Add scoped migration override to biome.json for a legacy directory

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
