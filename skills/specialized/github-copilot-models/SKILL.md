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

## Mindset

The model registry OpenCode ships with is a cache, not a source of truth — it is a config file bundled at OpenCode's own release time, so it is always at least as stale as the gap since that release. The GitHub Copilot API is the only place that reflects your actual, current, per-subscription access: a model can be added, removed, or moved from `preview` to `enabled` (or the reverse) without OpenCode's registry ever being updated. Every question about "what models can I use" or "why did this model stop working" should be answered by querying the live API first, then treating anything OpenCode's own listing says as a claim to verify, not a fact to trust.

This distinction matters most under two conditions the registry can never reflect: subscription-tier gating (your org's plan determines which models are `enabled` for you specifically, not globally) and policy-state churn (a model silently moving to `disabled` or `preview` is the single most common cause of "it worked yesterday" incidents).

## When to Use

Use this skill when you need to:

- **Validate model availability** before committing to a specific model in scripts or config
- **Compare context limits** when working with large codebases (need 100K+ tokens)
- **Check for new models** that may have been added since OpenCode's registry was last updated
- **Troubleshoot model selection** when a model isn't responding or seems unavailable
- **Filter by capability** (e.g., vision support for screenshot analysis)

## When NOT to Use

- The user already has a confirmed, currently-working model ID and only wants help with unrelated OpenCode configuration — don't route a simple config edit through a model query
- Questions about a different provider's model catalog (raw Anthropic API, OpenAI API directly, Azure OpenAI) — this skill is specifically for the GitHub Copilot proxy's `/models` endpoint and its `policy`/`capabilities` shape, which don't apply elsewhere

## Usage

### Quick Query

Use the provided script to fetch your available models, run relative to this skill's own directory:

```bash
# Plain listing
scripts/fetch-models.sh

# With JSON output for parsing
scripts/fetch-models.sh --json

# Filter by category
scripts/fetch-models.sh --category powerful

# Show only picker-enabled models
scripts/fetch-models.sh --picker-only
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

See [`references/manual-queries-and-workflows.md`](references/manual-queries-and-workflows.md) for the raw `curl`/`jq` equivalent of the script, plus worked example workflows (large-context filtering, vision-model discovery, per-vendor comparison).

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

## Decision Framework

### Model Selection Criteria

When choosing a model after querying:

1. **Context window**: Match to your typical task size (code review vs full codebase analysis)
2. **Policy state**: Only `"enabled"` models are usable; ignore `"disabled"` or restricted models
3. **Preview status**: Preview models may have limited availability based on subscription
4. **Capabilities**: Check for required features (vision, function calling, streaming)
5. **Category tags**: Use `powerful` for complex tasks, `versatile` for general use, `fast` for quick iterations

### Setting Defaults vs Per-Command

**Per-command** (recommended for testing):
- Use when trying new models
- Allows fallback if model fails
- Keeps project config stable

**Project default** (use after validation):
- Set in `.opencode/opencode.json` only after confirming model stability
- Avoids repeating `--model` flag on every command
- Risk: breaks workflow if model becomes unavailable

## Anti-Patterns

### NEVER hardcode model IDs without verifying availability first

**WHY:** GitHub Copilot model availability varies by subscription tier and region; a hardcoded ID that worked for one teammate can fail silently for another, or fail for everyone after a policy change.

❌ BAD:
```json
{ "defaultModel": "gpt-5.2-codex" }
```
set without ever checking whether the account actually has access.

✅ GOOD:
```bash
scripts/fetch-models.sh --json | jq -r '.data[].id'   # confirm it's in the list first
```

**Consequence:** The failure mode is silent until runtime — the config parses fine, and the error only surfaces as an opaque "model unavailable" when a job actually tries to use it.

### NEVER rely on OpenCode's model registry alone

**WHY:** OpenCode's registry is a snapshot baked in at release time; it does not track live per-account policy changes.

❌ BAD:
```bash
opencode models   # trusts a potentially stale bundled list
```

✅ GOOD:
```bash
scripts/fetch-models.sh --json   # queries the live GitHub Copilot API
```

**Consequence:** A model shown as available in `opencode models` can already be `disabled` on the account, and a genuinely new model can be invisible to OpenCode entirely.

### NEVER ignore `policy.state` when selecting a model

**WHY:** models with `"disabled"` or `"preview"` state can appear in the listing yet still reject requests at runtime.

❌ BAD:
```bash
jq '.data[0].id'   # takes the first model with no state check
```

✅ GOOD:
```bash
jq '.data[] | select(.policy.state == "enabled")'
```

**Consequence:** Scripts and configs built on an unfiltered list intermittently fail whenever the selected model happens to be `preview`- or `disabled`-gated for that account.

### NEVER assume context window size from a model's name

**WHY:** model names don't reliably encode context limits — a "3.5" in a name is a version number, not a token count, and vendors change limits between releases without renaming the model.

❌ BAD: guessing "this sounds like a big model" from the name alone.

✅ GOOD:
```bash
jq '.data[] | {id, context: .capabilities.limits.max_context_window_tokens}'
```

**Consequence:** A pipeline sized for the wrong context window either truncates input silently or rejects requests that should have fit.

### NEVER treat a `preview` model as production-safe

**WHY:** preview access is provisional and can be restricted or withdrawn without the standard deprecation notice a stable model would get, and `preview` models must always be cross-checked against `policy.state` before being wired into anything unattended.

❌ BAD: setting a `preview`-tagged model as a CI pipeline's `defaultModel`.

✅ GOOD: use `preview` models for manual, interactive testing only, and require `policy.state == "enabled"` (not `"preview"`) before a model is eligible for unattended use.

**Consequence:** A CI pipeline built on a preview model can start failing overnight with no corresponding code change, because the provider — not the user — changed the model's availability.

## Troubleshooting

Common issues (root causes and detailed diagnosis in [`references/troubleshooting.md`](references/troubleshooting.md)):

- **"Provider not found"** — target the correct provider explicitly
- **401 / "Invalid token"** — re-authenticate
- **Models not showing up that you expect** — OpenCode's registry may be stale; query the API directly
- **Model unavailable at runtime** — check `policy.state` and the `preview` flag

## Eval Scenarios

- [Scenario 1: Model audit script for large-codebase tasks](evals/scenario-1/task.md)
- [Scenario 2: Model recommendation report](evals/scenario-2/task.md)
- [Scenario 3: Setting a project default model safely](evals/scenario-3/task.md)
- [Scenario 4: Screenshot analysis pipeline — find vision models](evals/scenario-4/task.md)
- [Scenario 5: Incident — default model stopped responding](evals/scenario-5/task.md)

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Raw `curl`/`jq` API query and worked example filters | [Manual Queries and Workflows](references/manual-queries-and-workflows.md) | The packaged script isn't available, or you need a custom `jq` filter beyond its flags |
| Root-cause diagnosis for auth, staleness, and runtime-unavailable errors | [Troubleshooting](references/troubleshooting.md) | A query fails, a model that should be listed isn't, or a previously-working model stops responding |
| Official model listing endpoint | [GitHub Copilot Models API](https://api.githubcopilot.com/models) | Confirming the live response shape or endpoint behaviour directly |
| Provider auth setup and `auth.json` format | [OpenCode Authentication Docs](https://opencode.ai/docs/providers) | Setting up or debugging GitHub Copilot authentication in OpenCode |
| Tier-based model availability | [GitHub Copilot Subscription Plans](https://docs.github.com/en/copilot/about-github-copilot/subscription-plans-for-github-copilot) | Explaining why a model is `preview`-gated or unavailable on a given subscription |
