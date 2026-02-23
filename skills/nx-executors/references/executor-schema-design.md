# Executor Schema Design

Use this reference when designing `schema.json` for custom Nx executors.

## Baseline Schema Example

```json
{
  "$schema": "http://json-schema.org/schema",
  "type": "object",
  "properties": {
    "outputPath": {
      "type": "string",
      "description": "Where generated artifacts are written"
    },
    "watch": {
      "type": "boolean",
      "default": false,
      "description": "Enable incremental watch mode"
    }
  },
  "required": ["outputPath"]
}
```

## Validation Patterns

- Use `required` for mandatory operational parameters.
- Add defaults for safe optional flags.
- Prefer descriptive option text; it becomes CLI help content.

## Conditional/Dependent Example

```json
{
  "type": "object",
  "properties": {
    "mode": { "enum": ["dev", "prod"] },
    "minify": { "type": "boolean" }
  },
  "allOf": [
    {
      "if": { "properties": { "mode": { "const": "prod" } } },
      "then": { "required": ["minify"] }
    }
  ]
}
```
