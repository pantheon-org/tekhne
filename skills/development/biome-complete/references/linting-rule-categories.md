# Linting Rule Categories

Use this reference when enabling or tuning Biome linter rules.

## Workflow

1. Start with `recommended: true`.
2. Promote high-value diagnostics to `error`.
3. Downgrade noisy rules only with justification.
4. Re-run `check` and confirm signal quality.

## Example Rule Overrides

```json
{
  "linter": {
    "rules": {
      "recommended": true,
      "suspicious": {
        "noExplicitAny": "error"
      },
      "style": {
        "useImportType": "warn"
      }
    }
  }
}
```

## Verify

```bash
bunx @biomejs/biome check .
```

## Anti-pattern

- Do not disable full rule groups to silence one warning.
