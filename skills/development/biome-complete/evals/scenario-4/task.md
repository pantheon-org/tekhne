# Troubleshoot biome check Failures After ESLint Migration

## Problem Description

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
