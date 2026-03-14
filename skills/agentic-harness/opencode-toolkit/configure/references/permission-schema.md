# Permission Schema Patterns

Use this reference for safe, maintainable OpenCode permission configuration.

## Permission Strategy

- Start from restrictive defaults.
- Allow only required commands and file scopes.
- Separate local development convenience from CI/production safeguards.

## Example Pattern

```json
{
  "permission": {
    "edit": "ask",
    "webfetch": "allow",
    "bash": {
      "npm *": "allow",
      "git *": "allow",
      "rm *": "ask",
      "*": "ask"
    }
  }
}
```

## Common Patterns

- Use command-pattern granularity for `bash` rules.
- Keep file access scoped to repository paths when configurable.
- Re-test after each permission block change.
