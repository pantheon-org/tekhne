# Formatter Options

Use this reference when normalizing project formatting with Biome.

## Common Formatter Settings

```json
{
  "formatter": {
    "enabled": true,
    "indentStyle": "space",
    "indentWidth": 2,
    "lineWidth": 100,
    "lineEnding": "lf"
  },
  "javascript": {
    "formatter": {
      "quoteStyle": "double",
      "trailingCommas": "all"
    }
  }
}
```

## Verify

```bash
bunx @biomejs/biome format . --write
```

```bash
bunx @biomejs/biome check .
```

## Migration Note

Run Biome formatter once after removing Prettier from the same files.
