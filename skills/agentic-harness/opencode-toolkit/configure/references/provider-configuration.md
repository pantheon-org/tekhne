# Provider Configuration

Use this reference for provider setup patterns and model routing.

## Provider Setup Principles

- Keep provider entries explicit and minimal.
- Use environment-backed secrets (`baseEnv`) instead of inline keys.
- Prefer deterministic model identifiers.

## Example Pattern

```json
{
  "providers": [
    {
      "name": "openai",
      "baseEnv": "OPENAI_API_KEY"
    },
    {
      "name": "anthropic",
      "baseEnv": "ANTHROPIC_API_KEY"
    }
  ],
  "model": "openai/gpt-4.1"
}
```

## Notes

- If provider order affects resolution, document ordering rationale.
- Keep fallback models explicit for reliability in degraded provider scenarios.
