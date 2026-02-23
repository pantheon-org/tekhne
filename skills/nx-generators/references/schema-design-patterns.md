# Schema Design Patterns

Use this reference to design generator schemas that fail early and guide users clearly.

## Baseline Schema Pattern

```json
{
  "cli": "nx",
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "description": "Artifact name"
    },
    "directory": {
      "type": "string",
      "description": "Target directory"
    }
  },
  "required": ["name"]
}
```

## Conditional Schema Example

```json
{
  "type": "object",
  "properties": {
    "publishable": { "type": "boolean" },
    "importPath": { "type": "string" }
  },
  "allOf": [
    {
      "if": { "properties": { "publishable": { "const": true } } },
      "then": { "required": ["importPath"] }
    }
  ]
}
```

## Design Guidance

- Use `required` for mandatory values.
- Add clear `description` fields for CLI help readability.
- Provide sane defaults for optional flags.
- Avoid ambiguous option names across generators.
