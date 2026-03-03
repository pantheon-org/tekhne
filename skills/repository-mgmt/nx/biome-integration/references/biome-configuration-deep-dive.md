# Biome Configuration Deep Dive for Nx

Use this guide when you need advanced configuration details beyond the SKILL hub.

## Root Configuration Baseline

Create and maintain one root `biome.json` as the default authority:

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

## Nx Target Defaults for Cache Stability

Ensure lint and format targets include config and dependency inputs:

```json
{
  "targetDefaults": {
    "biome-lint": {
      "cache": true,
      "inputs": [
        "default",
        "^default",
        "{workspaceRoot}/biome.json",
        { "externalDependencies": ["@biomejs/biome"] }
      ]
    },
    "biome-format": {
      "cache": true,
      "inputs": [
        "default",
        "^default",
        "{workspaceRoot}/biome.json",
        { "externalDependencies": ["@biomejs/biome"] }
      ]
    }
  }
}
```

## Advanced Rule Tuning Strategy

- Keep `recommended: true` as baseline.
- Override only rules that show recurring false positives for your repository.
- Prefer narrow path-scoped exceptions over global relaxations.
- Record reason + expiration for temporary suppressions.

Example scoped override (temporary migration guardrail):

```json
{
  "overrides": [
    {
      "includes": ["apps/legacy/**"],
      "linter": {
        "rules": {
          "style": {
            "noNonNullAssertion": "off"
          }
        }
      }
    }
  ]
}
```

## Performance Optimization Tips

- Run `nx run-many -t biome-lint` instead of ad hoc per-project loops.
- Use Nx affected workflows in CI to limit scope.
- Reset daemon cache (`nx reset`) after plugin or target definition changes.
- Keep plugin inference logic lightweight; avoid expensive filesystem traversal.

## Verification Checklist

- `biome.json` exists at workspace root.
- `biome-lint` and `biome-format` are cache-enabled.
- Cache inputs include root config and external Biome dependency.
- CI verification runs non-mutating checks.
