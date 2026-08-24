# Model Routing for Plan Review

Achieving 3 different models for the 3 reviewers depends on your opencode
configuration. This reference documents the available configuration options.

## opencode.json subAgents config

OpenCode supports routing different `subagent_type` values to different models
via `subAgents` in `opencode.json`:

```jsonc
{
  "subAgents": {
    "general": {
      "model": "claude-sonnet-5",
      "systemPrompt": "You are a thorough, detail-oriented reviewer."
    },
    "explore": {
      "model": "deepseek-v4-flash",
      "systemPrompt": "You are a skeptical, adversarial reviewer focused on finding flaws."
    }
  }
}
```

## Recommended model pairings

| Reviewer | subagent_type | Recommended model | Rationale |
|----------|--------------|-------------------|-----------|
| Technical | `general` | Claude Sonnet 5 or GPT 5.1 Codex Mini | Strong reasoning for implementation feasibility |
| Strategic | `general` | Claude Sonnet 5 or GPT 5.1 Codex Mini | Both Technical and Strategic need reasoning depth — use the same strong model |
| Risk | `explore` | DeepSeek V4 Flash or GPT 5 Nano | A different model brings different blind spots; cheaper models are fine for adversarial review |

## When only 2 subagent types are available

OpenCode provides `general` and `explore` subagent types (plus `atlassian`
which is manual-only). If you can only configure 2 distinct models, use:

- **Strongest model** → `general` (covers both Technical and Strategic — run them
  as separate `general` calls on the same model; the different prompts still
  produce divergent analysis)
- **Second model** → `explore` (Risk reviewer gets a fresh perspective)

## When all subagents share one model

The reviewer prompts and question sets still provide useful diversity:

- Technical asks about feasibility, gaps, consistency
- Strategic asks about alignment, scope, priority
- Risk asks about blind spots, failure modes, edge cases

Even on the same model, these produce meaningfully different output. The
consolidation catches contradictions between them.

## Pricing and pairing by environment

Present the relevant table via the `question` tool (structure defined in
[assets/templates/model-selection.yaml](../assets/templates/model-selection.yaml),
validated by [assets/schemas/model-selection.schema.json](../assets/schemas/model-selection.schema.json)).
Assign one model to **Technical + Strategic** (`general` subagent type) and a
**different model** to **Risk** (`explore` subagent type) — different models
mean different blind spots.

### OpenCode Zen (pay-as-you-go)

| Tier | Pair | Tech/Strategic | Risk | Cost/run | When |
|------|------|---------------|------|----------|------|
| **Best value** (Recommended) | DS Flash + GPT 5 Nano | DeepSeek V4 Flash ($0.14/M in) | GPT 5 Nano ($0.05/M in) | ~$0.03 | Default for most reviews |
| **Balanced** | DS Flash + Claude Haiku | DeepSeek V4 Flash ($0.14/M in) | Claude Haiku 4.5 ($1.00/M in) | ~$0.08 | Want an Anthropic perspective on risk |
| **Max depth** | DS Pro + GPT 5 Nano | DeepSeek V4 Pro ($1.74/M in) | GPT 5 Nano ($0.05/M in) | ~$0.12 | Critical, high-stakes plan review |
| **Budget** | GPT 5 Nano (all) | GPT 5 Nano ($0.05/M in) | GPT 5 Nano ($0.05/M in) | ~$0.01 | Draft plan, quick sanity check |

### OpenCode Go (flat-rate subscription)

All models cost the same (no per-token charge). Prioritise diversity:

```jsonc
{
  "subAgents": {
    "general": { "model": "deepseek-v4-flash" },   // Technical/Strategic
    "explore": { "model": "minimax-m3" }            // Risk -- different family
  }
}
```

| Pair | Tech/Strategic | Risk | Why |
|------|---------------|------|-----|
| **DS Flash + MiniMax** (Recommended) | DeepSeek V4 Flash (1M ctx) | MiniMax M3 (512K ctx) | Different model families, good diversity |
| **DS Flash + GLM-5.2** | DeepSeek V4 Flash (1M ctx) | GLM-5.2 (1M ctx) | Risk gets same giant context for long plans |
| **DS Flash + Kimi Code** | DeepSeek V4 Flash (1M ctx) | Kimi K2.7 Code (262K ctx) | Code-oriented risk lens |

### Native Anthropic CLI harness

All three models are available directly:

| Model | Input/MTok | Output/MTok | Context | Reasoning | Best for |
|-------|-----------|------------|---------|-----------|----------|
| **Claude Haiku 4.5** | $1.00 | $5.00 | 200K | Adequate | Risk reviewer -- cheap, fast, still catches blind spots |
| **Claude Sonnet 5** | $3.00 | $15.00 | 1M | Strong | Technical/Strategic -- sweet spot for depth |
| **Claude Opus 4.8** | $5.00 | $25.00 | 1M | Maximum | Both reviewers -- only if plan is mission-critical |

| Pair | Tech/Strategic | Risk | Cost/run | When |
|------|---------------|------|----------|------|
| **Sonnet + Haiku** (Recommended) | Claude Sonnet 5 | Claude Haiku 4.5 | ~$0.10 | Best capability/cost balance |
| **Sonnet + Sonnet** | Claude Sonnet 5 | Claude Sonnet 5 | ~$0.15 | Same model everywhere (less diverse) |
| **Opus + Haiku** | Claude Opus 4.8 | Claude Haiku 4.5 | ~$0.18 | Max depth on technical analysis, cheap risk |
| **Opus + Sonnet** | Claude Opus 4.8 | Claude Sonnet 5 | ~$0.25 | Both reviewers get strong reasoning |

### Bring Your Own Key (BYOK)

Ask the user which models their provider supports. If unsure, recommend:

- Technical/Strategic: the strongest reasoning model available
- Risk: a different model (ideally from a different provider family)

### Default recommendation ("not sure" / "surprise me")

- **OpenCode Zen:** DeepSeek V4 Flash (Technical/Strategic) + GPT 5 Nano (Risk) -- ~$0.03/run
- **OpenCode Go:** DeepSeek V4 Flash (Technical/Strategic) + MiniMax M3 (Risk) -- flat-rate
- **Native Anthropic CLI harness:** Claude Sonnet 5 (Technical/Strategic) + Claude Haiku 4.5 (Risk) -- ~$0.10/run
- **BYOK:** Strongest model (Technical/Strategic) + cheapest different model (Risk)
