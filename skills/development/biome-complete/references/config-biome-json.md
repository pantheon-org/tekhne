# Biome Config (`biome.json`)

Use this reference when creating or tightening `biome.json`.

## Baseline Config

```json
{
  "$schema": "https://biomejs.dev/schemas/1.9.4/schema.json",
  "vcs": {
    "enabled": true,
    "clientKind": "git",
    "useIgnoreFile": true
  },
  "files": {
    "ignoreUnknown": true
  },
  "formatter": {
    "enabled": true,
    "lineWidth": 100,
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

## Verify

```bash
bunx @biomejs/biome check .
```

## Notes

- Keep `useIgnoreFile` enabled to honor `.gitignore`.
- Tune `lineWidth` to repository standards before broad formatting.
