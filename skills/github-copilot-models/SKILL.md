---
name: github-copilot-models
description: |-
  Query and display available GitHub Copilot AI models with their capabilities, context limits, and features. Use when: "what models are available", "show copilot models", "list github models", "check model capabilities", "switch models".

  Examples:
  - user: "What models can I use with GitHub Copilot?" → fetch and display available models
  - user: "Show me models with vision support" → filter models by capability
  - user: "Which model has the largest context window?" → compare model specifications
  - user: "List all GPT-5 models" → filter by model family
---

# GitHub Copilot Models Query

Query available GitHub Copilot AI models directly from the API to see what models you actually have access to (not just
what OpenCode knows about).

## Usage

### Quick Query

Use the provided script to fetch your available models:

```bash
# From project root
.opencode/skills/github-copilot-models/scripts/fetch-models.sh

# With JSON output for parsing
.opencode/skills/github-copilot-models/scripts/fetch-models.sh --json

# Filter by category
.opencode/skills/github-copilot-models/scripts/fetch-models.sh --category powerful

# Show only picker-enabled models
.opencode/skills/github-copilot-models/scripts/fetch-models.sh --picker-only
```

### Script Options

| `--json`           | Raw JSON output           | `fetch-models.sh --json`               |
| `--picker-only`    | Only show featured models | `fetch-models.sh --picker-only`        |
| `--category <cat>` | Filter by category        | `fetch-models.sh --category versatile` |
| `--family <name>`  | Filter by model family    | `fetch-models.sh --family claude`      |
| `--vendor <name>`  | Filter by vendor          | `fetch-models.sh --vendor Anthropic`   |
| `--vision`         | Only models with vision   | `fetch-models.sh --vision`             |
| `--help`           | Show all options          | `fetch-models.sh --help`               |

### Manual API Query

```bash
# Get auth token from OpenCode config
AUTH_TOKEN=$(jq -r '.["github-copilot"].access' ~/.local/share/opencode/auth.json)

# Query GitHub Copilot API
curl -s -H "Authorization: Bearer $AUTH_TOKEN" \
  "https://api.githubcopilot.com/models" | jq .
```

## Authentication

The script automatically reads your OpenCode authentication from:

```
~/.local/share/opencode/auth.json
```

If authentication fails:

```bash
# Re-authenticate with GitHub Copilot
opencode auth add github-copilot

# Verify authentication
opencode auth list
```

## Switching Models

After finding your desired model, specify it per-command or set a default:

```bash
# Per-command (recommended for testing)
opencode run --model gpt-5.2-codex "Refactor this code"

# Set project default in .opencode/opencode.json
{
  "defaultModel": "gpt-5.2-codex"
}
```

**Validation:** Test the model is active and responding correctly:

```bash
opencode run "Echo back: model working" && echo "✓ Model active"
```

## Example Workflows

### Find Best Model for Large Codebase Analysis

```bash
# Find models with 200K+ context
./scripts/fetch-models.sh --json | jq '.data[] | select(.capabilities.limits.max_context_window_tokens > 200000) | {id, context: .capabilities.limits.max_context_window_tokens}'
```

### Find Models with Vision for Screenshot Analysis

```bash
# Show vision-capable models
./scripts/fetch-models.sh --vision
```

### Compare All Claude Models

```bash
# Filter by vendor
./scripts/fetch-models.sh --vendor Anthropic
```

### Get Model IDs for Scripting

```bash
# Extract just the IDs
./scripts/fetch-models.sh --json | jq -r '.data[].id'
```

## Troubleshooting

Common issues:

- **"Provider not found"** — Run `opencode models github-copilot` to target the correct provider.
- **401 / "Invalid token"** — Re-authenticate with `opencode auth add github-copilot`.
- **Models not showing in OpenCode** — OpenCode's registry may be stale; use the script to query the API directly.
- **Model unavailable at runtime** — Check `policy.state` (`"enabled"` required) and the `preview` flag in the API response; preview models may have restricted availability depending on your subscription.
