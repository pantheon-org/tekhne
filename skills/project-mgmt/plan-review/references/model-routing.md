# Model Routing for Plan Review

Achieving 3 different models for the 3 reviewers depends on a saved config. This
reference documents the config and the model pairings.

## Saved config: .claude/plan-review.json

Save the reviewer models once so the skill stops asking. The skill uses the first
valid file in this order:

1. `.claude/plan-review.json` in the project root
2. `~/.claude/plan-review.json` (user-wide default)

```json
{
  "technicalStrategic": "opus",
  "risk": "sonnet"
}
```

Values are the Agent tool's `model` values: `opus`, `sonnet`, `haiku`, `fable`. The enum
in [the schema](../assets/schemas/plan-review-config.schema.json) follows those values, so
update it when they change; if the Agent call rejects a configured model, report it and ask
as in step 5. `technicalStrategic` applies to the Technical and Strategic reviewers and `risk` to
the Risk reviewer. Pick two different models for different blind spots. Validate
with [scripts/validate-plan-review-config.sh](../scripts/validate-plan-review-config.sh);
the starting point is [assets/templates/plan-review.json](../assets/templates/plan-review.json).
In the tables below, Opus, Sonnet and Haiku mean the `opus`, `sonnet` and `haiku` values.

If `technicalStrategic` and `risk` name the same model, use it and note in the final report
that model diversity is limited. If the file is invalid (the validator exits non-zero), say
what it printed and ask as usual, without using any later file. After the user chooses
models, offer to save the choice to `.claude/plan-review.json` so the question is not asked
again.

Known limit: the shape gives Technical and Strategic one model. Three distinct models would
need a new shape.

## Recommended model pairings

| Reviewer | subagent_type | Recommended model | Rationale |
|----------|--------------|-------------------|-----------|
| Technical | `general` | Claude Sonnet 5 or Opus 4.8 | Strong reasoning for implementation feasibility |
| Strategic | `general` | Claude Sonnet 5 or Opus 4.8 | Both Technical and Strategic need reasoning depth — use the same strong model |
| Risk | `explore` | Claude Haiku 4.5 or Sonnet 5 | A different model brings different blind spots; a cheaper model is fine for adversarial review |

## When only 2 distinct models are configured

The Technical and Strategic reviewers share the `general` subagent type and Risk uses
`explore` (in Claude Code, `general-purpose` and `Explore`). With 2 distinct models, use:

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

## Pricing and pairings

Present the table via the question box (structure defined in
[assets/templates/model-selection.yaml](../assets/templates/model-selection.yaml),
validated by [assets/schemas/model-selection.schema.json](../assets/schemas/model-selection.schema.json)).
Assign one model to **Technical + Strategic** (`general` subagent type) and a
**different model** to **Risk** (`explore` subagent type) — different models
mean different blind spots.

### Claude Code

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

### Default recommendation ("not sure" / "surprise me")

- **Claude Code:** Claude Sonnet 5 (Technical/Strategic) + Claude Haiku 4.5 (Risk) -- ~$0.10/run
