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

## Core Concepts

### Model Picker Status

Models returned by the API have a `model_picker_enabled` flag:

- **`true`**: Actively promoted by GitHub, ready for production use
- **`false`**: Available but not featured (legacy, specialized, or deprecated)

### Model Categories

GitHub organizes models into categories via `model_picker_category`:

- **`powerful`**: High-performance models for complex tasks (large context windows)
- **`versatile`**: Balanced models for general-purpose coding
- **`lightweight`**: Fast, efficient models for quick tasks

### Capabilities

Each model includes a `capabilities` object with:

- **Context limits**: `max_context_window_tokens`, `max_output_tokens`, `max_prompt_tokens`
- **Vision support**: Image formats, max images, max size
- **Tool support**: `parallel_tool_calls`, `tool_calls`
- **Structured outputs**: JSON schema enforcement (GPT models only)
- **Thinking budget**: Min/max tokens for reasoning (Claude, Gemini)
- **Streaming**: Real-time response streaming

## Usage

### Quick Query

Use the provided script to fetch your available models:

```bash
# From project root
.opencode/skills/github-copilot-models/fetch-models.sh

# With JSON output for parsing
.opencode/skills/github-copilot-models/fetch-models.sh --json

# Filter by category
.opencode/skills/github-copilot-models/fetch-models.sh --category powerful

# Show only picker-enabled models
.opencode/skills/github-copilot-models/fetch-models.sh --picker-only
```

### Script Options

| Option             | Description               | Example                                |
| ------------------ | ------------------------- | -------------------------------------- |
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

## Model Information

### Key Fields

```json
{
  "id": "claude-sonnet-4.5",
  "name": "Claude Sonnet 4.5",
  "vendor": "Anthropic",
  "model_picker_enabled": true,
  "model_picker_category": "versatile",
  "preview": false,
  "capabilities": {
    "family": "claude-sonnet-4.5",
    "limits": {
      "max_context_window_tokens": 144000,
      "max_output_tokens": 32000,
      "max_prompt_tokens": 128000
    },
    "supports": {
      "vision": true,
      "tool_calls": true,
      "streaming": true
    }
  }
}
```

### Understanding Limits

- **`max_context_window_tokens`**: Total tokens (prompt + output)
- **`max_prompt_tokens`**: Maximum input size
- **`max_output_tokens`**: Maximum response length
- **`max_non_streaming_output_tokens`**: Limit when not streaming (some models)

Example: Claude Sonnet 4.5

- Context: 144K total
- Prompt: Up to 128K
- Output: Up to 32K streaming, 16K non-streaming

## Comparing Models

### By Context Window

Largest context windows (as of 2026-02-04):

1. **GPT-5.1/5.2 Codex variants**: 400K tokens
2. **GPT-5.1/5.2, GPT-5-mini**: 264K tokens
3. **Claude Opus 4.5**: 160K tokens
4. **Claude Sonnet/Haiku 4.5**: 144K tokens
5. **GPT-4.1, GPT-4o, Gemini**: 128K tokens

### By Vision Capabilities

- **Most images**: Gemini (10 images)
- **Good for multi-image**: Claude (5 images)
- **Single image**: GPT models (1 image)
- **Formats**: Most support JPEG, PNG, WebP; Gemini adds HEIC/HEIF

### By Output Length

Longest outputs:

1. **GPT-5.1/5.2 Codex variants**: 128K tokens
2. **GPT-5.1/5.2, Gemini**: 64K tokens
3. **Claude models**: 32K tokens (streaming)

### By Feature

**Structured Outputs (JSON Schema):**

- ✅ GPT-5.x models
- ✅ GPT-4.1
- ❌ Claude models
- ❌ Gemini models

**Thinking Budget (Reasoning):**

- ✅ Claude (1K-32K tokens)
- ✅ Gemini (128-32K tokens)
- ❌ GPT models

**Parallel Tool Calls:**

- ✅ All featured models

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

After finding your desired model:

```bash
# Method 1: Interactive selector (if supported by OpenCode)
opencode --model

# Method 2: Specify in command
opencode run --model gpt-5.2-codex "Refactor this code"

# Method 3: Set default in project config
# Edit .opencode/opencode.json:
{
  "defaultModel": "gpt-5.2-codex"
}

# Method 4: Set default in global config
# Edit ~/.config/opencode/opencode.json
```

## Troubleshooting

### "Provider not found" Error

Make sure you're querying `github-copilot` specifically:

```bash
opencode models github-copilot
```

### "Invalid token" or 401 Error

Re-authenticate:

```bash
opencode auth add github-copilot
```

### Models Not Showing in OpenCode

OpenCode's model registry may be out of sync. Use this skill's script to query the API directly for the most up-to-date
list.

### Model Not Available When Trying to Use

Some models may appear in the API response but have usage restrictions:

- Check `policy.state` (should be `"enabled"`)
- Check `preview` flag (preview models may have limited availability)
- Verify your GitHub Copilot subscription level

## Example Workflows

### Find Best Model for Large Codebase Analysis

```bash
# Find models with 200K+ context
./fetch-models.sh --json | jq '.data[] | select(.capabilities.limits.max_context_window_tokens > 200000) | {id, context: .capabilities.limits.max_context_window_tokens}'
```

### Find Models with Vision for Screenshot Analysis

```bash
# Show vision-capable models
./fetch-models.sh --vision
```

### Compare All Claude Models

```bash
# Filter by vendor
./fetch-models.sh --vendor Anthropic
```

### Get Model IDs for Scripting

```bash
# Extract just the IDs
./fetch-models.sh --json | jq -r '.data[].id'
```

## Resources

- **GitHub Copilot Models Docs**:
  [https://docs.github.com/en/copilot/using-github-copilot/ai-models](https://docs.github.com/en/copilot/using-github-copilot/ai-models)
- **OpenCode Docs**: [https://opencode.ai/docs](https://opencode.ai/docs)
- **API Endpoint**: `https://api.githubcopilot.com/models`
