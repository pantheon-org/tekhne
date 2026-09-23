# Manual API Queries and Example Workflows

Load this reference when the packaged `scripts/fetch-models.sh` script isn't available in the current environment, or when composing a custom `jq` filter beyond what the script's flags expose.

## Manual API Query (no script)

```bash
# Get auth token from OpenCode config
AUTH_TOKEN=$(jq -r '.["github-copilot"].access' ~/.local/share/opencode/auth.json)

# Query GitHub Copilot API directly
curl -s -H "Authorization: Bearer $AUTH_TOKEN" \
  "https://api.githubcopilot.com/models" | jq .
```

This is exactly what `scripts/fetch-models.sh` does internally — reach for the script first; use this only when the script isn't present or you need a one-off `jq` expression the script's flags don't cover.

## Example Workflows

### Find best model for large-codebase analysis

```bash
# Find models with 200K+ context
scripts/fetch-models.sh --json | jq '.data[] | select(.capabilities.limits.max_context_window_tokens > 200000) | {id, context: .capabilities.limits.max_context_window_tokens}'
```

### Find models with vision for screenshot analysis

```bash
scripts/fetch-models.sh --vision
```

### Compare all models from one vendor

```bash
scripts/fetch-models.sh --vendor Anthropic
```

### Get model IDs for scripting

```bash
scripts/fetch-models.sh --json | jq -r '.data[].id'
```

### Build an allow-list of production-safe models

Combine the `policy.state` and context-window checks into one filter, since neither alone is sufficient for a pipeline that must not break overnight:

```bash
scripts/fetch-models.sh --json | jq '
  .data[]
  | select(.policy.state == "enabled")
  | select(.capabilities.limits.max_context_window_tokens >= 100000)
  | .id
'
```
