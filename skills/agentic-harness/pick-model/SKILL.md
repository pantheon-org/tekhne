---
name: pick-model
description: Recommend optimal Claude model (haiku/sonnet/opus) for a task. Use when user asks "which model", "pick model", "model for", or before starting costly/complex tasks. Covers tech and non-tech tasks.
model: haiku
context: main
---

# Pick Model

Classify user's task → recommend optimal model with reasoning.

## Instructions

1. Parse task description from `$ARGUMENTS`
2. Classify against decision matrix below
3. Output recommendation using format template

## Decision Matrix

### Technical Tasks

| Model | When to Use |
|---|---|
| 🟢 **Haiku** | Simple transforms, formatting, regex, typo fix, status query, template fill, data extraction, factual lookup (no reasoning), file conversion |
| 🟡 **Sonnet** | Single-file coding, bug fix, code review, moderate debugging, test writing, PR review, standard refactoring, technical docs, API integration (known patterns) |
| 🔴 **Opus** | Multi-file refactor (3+ files), architecture/design decisions, complex debugging (multi-system), framework migration, security audit, novel algorithm design, system design with trade-offs |

### Business & Strategy Tasks

| Model | When to Use |
|---|---|
| 🟢 **Haiku** | Summarization (<2K words), data extraction, status reports, simple translations, template filling, meeting notes formatting |
| 🟡 **Sonnet** | Content creation (blog, email, docs), research summaries, competitive analysis, standard business writing, persuasive proposals, marketing copy, customer communications |
| 🔴 **Opus** | Strategic planning, business model design, M&A analysis, organizational design, change management plans, competitive strategy, market entry decisions, crisis response, stakeholder management (competing interests), long-form reports (>2K words), executive presentations with nuance |

### Creative & Analysis Tasks

| Model | When to Use |
|---|---|
| 🟢 **Haiku** | Basic formatting, simple data viz suggestions, straightforward categorization |
| 🟡 **Sonnet** | Creative writing, brainstorming (single framework), persona development, user research synthesis, A/B test analysis, survey analysis |
| 🔴 **Opus** | Multi-framework brainstorming (SCAMPER + Starbursting + trade-off analysis), cross-session pattern detection, bias identification, retrospective analysis, ethical reasoning, strategic foresight, scenario planning |

### Commands, Skills, Agents

| Model | When to Use |
|---|---|
| 🟢 **Haiku** | Simple conversions (PDF, EPUB), format checks, simple utilities, minimal reasoning |
| 🟡 **Sonnet** | Standard workflows, context management, serialization, most skills/commands (DEFAULT) |
| 🔴 **Opus** | Strategic analysis (brainstorm, retrospectives), multi-framework reasoning, high-stakes decisions, pattern detection across sessions |

## Complexity Escalators

Upgrade one tier if task has ANY of these signals:

### Technical Escalators
- **Ambiguity**: Underspecified requirements, multiple valid interpretations → +1 tier
- **Scope**: Affects 3+ files/systems/components → +1 tier
- **Stakes**: Production system, security, data-loss risk, regulatory compliance → +1 tier
- **Novelty**: No established pattern, novel algorithm, cutting-edge tech → +1 tier

### Business Escalators
- **Multiple stakeholders**: Competing interests, need to balance trade-offs → +1 tier
- **Strategic impact**: Long-term consequences, irreversible decisions, organizational change → +1 tier
- **Political sensitivity**: Layoffs, restructuring, executive communications, crisis → +1 tier
- **Cross-functional**: Requires synthesis across domains (tech + business + legal) → +1 tier

### Cognitive Escalators
- **Pattern detection**: Requires analyzing trends across multiple data points/sessions → +1 tier
- **Bias identification**: Needs to spot blindspots, cognitive biases, assumptions → +1 tier
- **Ethical reasoning**: Moral ambiguity, fairness considerations, unintended consequences → +1 tier
- **Multi-framework**: Applying 2+ analytical frameworks simultaneously → +1 tier

**Cap at Opus.** If multiple escalators apply, still cap at Opus (don't "double upgrade").

## Decision Guidance

**When uncertain between two models:**
- **Haiku vs Sonnet**: Does it require *any* reasoning/judgment? → Sonnet
- **Sonnet vs Opus**: Are there trade-offs to balance or multiple valid approaches? → Opus
- **Default rule**: When in doubt, go one tier up (better quality > cost savings)

**Quality vs Cost trade-offs:**
- **Cost-sensitive**: Batch processing, exploratory work, drafts → prefer lower tier
- **Quality-critical**: Customer-facing, executive, production, irreversible → prefer higher tier
- **Iteration-friendly**: Can easily retry with higher tier if insufficient → start lower

**Speed considerations:**
- Haiku is ~3-5x faster than Sonnet, ~10x faster than Opus
- For latency-sensitive workflows (UI feedback, real-time), prefer Haiku/Sonnet
- For batch/async work, speed matters less than quality

## Output Format

```
[emoji] **[Model]** — [1-line reason]

💰 Cost: [lowest/medium/highest] | ⚡ Speed: [fastest/medium/slowest]

💡 [Optional: "Consider [other model] if [condition]"]
```

**Example output:**
```
🔴 **Opus** — Multi-stakeholder strategic decision with trade-offs

💰 Cost: highest | ⚡ Speed: slowest

💡 Consider Sonnet if this is exploratory (draft) rather than final recommendation
```

## Examples

### Technical Tasks

| Task | Recommendation | Rationale |
|---|---|---|
| "fix typo in README" | 🟢 **Haiku** | Trivial single edit, no reasoning |
| "convert PDF to markdown" | 🟢 **Haiku** | Simple conversion, no decisions |
| "debug flaky integration test" | 🟡 **Sonnet** | Single-system debugging, moderate reasoning |
| "refactor auth across 15 files" | 🔴 **Opus** | Multi-file (3+ escalator) + architectural decisions |
| "design database schema for e-commerce" | 🔴 **Opus** | Architectural decision with trade-offs, long-term impact |
| "plan microservices migration strategy" | 🔴 **Opus** | Complex architectural planning + strategic impact escalator |

### Business & Strategy

| Task | Recommendation | Rationale |
|---|---|---|
| "summarize this meeting transcript" | 🟢 **Haiku** | Simple text transformation, <2K words |
| "extract action items from notes" | 🟢 **Haiku** | Data extraction, no reasoning |
| "write blog post about AI trends" | 🟡 **Sonnet** | Creative writing, moderate reasoning |
| "draft sales proposal for enterprise client" | 🟡 **Sonnet** | Persuasive writing, moderate reasoning |
| "analyze competitor pricing strategy" | 🟡 **Sonnet** | Research/analysis, single framework |
| "plan market entry strategy for Europe" | 🔴 **Opus** | Strategic impact + cross-functional + ambiguity escalators |
| "design organizational restructuring plan" | 🔴 **Opus** | Political sensitivity + multiple stakeholders + strategic impact |
| "M&A due diligence analysis" | 🔴 **Opus** | Strategic stakes + cross-functional synthesis required |
| "crisis communication plan for data breach" | 🔴 **Opus** | Political sensitivity + stakes + multiple stakeholders |

### Creative & Analysis

| Task | Recommendation | Rationale |
|---|---|---|
| "translate paragraph to French" | 🟢 **Haiku** | Simple language transform, no reasoning |
| "brainstorm product names (single session)" | 🟡 **Sonnet** | Creative generation, moderate reasoning |
| "brainstorm with SCAMPER + trade-off analysis" | 🔴 **Opus** | Multi-framework escalator (SCAMPER + weighted scoring) |
| "retrospect: analyze collaboration patterns" | 🔴 **Opus** | Pattern detection + bias identification escalators |
| "identify blindspots in strategy" | 🔴 **Opus** | Bias identification + ethical reasoning escalators |
| "plan 3-day conference with speakers" | 🔴 **Opus** | Complex scheduling + multiple stakeholders + constraints |

### Commands, Skills, Agents

| Task | Recommendation | Rationale |
|---|---|---|
| "command: convert EPUB to markdown" | 🟢 **Haiku** | Simple workflow, minimal reasoning |
| "command: save session context" | 🟡 **Sonnet** | Context management, serialization logic |
| "command: brainstorm with research + SCAMPER" | 🔴 **Opus** | Multi-framework escalator + strategic analysis |
| "command: retrospect domain learnings" | 🔴 **Opus** | Pattern detection across sessions + bias identification |
| "skill: format code with prettier" | 🟢 **Haiku** | Simple deterministic task |
| "skill: standard workflow implementation" | 🟡 **Sonnet** | Standard workflow, moderate reasoning |
| "agent: explore codebase architecture" | 🔴 **Opus** | Complex exploration + architectural synthesis |

## Philosophy

- **Right-size, don't default up** — the cheapest model that meets quality requirements is the correct choice; upgrading is easy, right-sizing takes discipline.
- **Task type over task size** — model selection depends on reasoning complexity, not the volume of text or files involved.
- **Tier aliases over model names** — use fast/balanced/reasoning tiers; specific model names change; tier semantics persist.
- **Escalate explicitly** — if a lower tier fails, escalate with a documented reason rather than defaulting to the top tier always.

## When to Use

- When a user asks "which model should I use for this task?" before starting work
- When selecting between multiple LLM tiers for an automated agent workflow or pipeline
- When cost vs capability tradeoff needs to be explicit (e.g. batch jobs, production routing)
- When an agent must self-assign a model for a sub-task without human input
- When an existing workflow is over-spending on frontier models for simple tasks

## When Not to Use

- When the model is already fixed by infrastructure constraints (e.g. a provider only offers one model)
- When the task has already been completed and model selection is moot
- When the user is asking about non-Claude providers and needs a cross-vendor comparison tool
- When fine-tuned or domain-specific models are the deciding factor, not tier
- When the decision depends on real-time pricing data not available in this skill

## Anti-Patterns

- **NEVER default to the most powerful model for every task** — Oversized models inflate costs without quality gain on simple tasks. **Why:** A haiku/flash-class model handles classification and routing at 10x lower cost.
- **NEVER pick a model based on benchmark leaderboards alone** — Benchmark tasks often differ from production workloads. **Why:** Real task performance depends on prompt structure, context length, and domain specificity.
- **NEVER hardcode model names in agent workflows** — Providers rename and deprecate models frequently. **Why:** Hardcoded names break silently on deprecation; use model tier aliases (fast/balanced/reasoning).
- **NEVER skip escalator checks for ambiguous tasks** — Underestimating complexity leads to poor output requiring costly reruns. **Why:** A single missed escalator (e.g. multi-stakeholder, security risk) can push a task from Sonnet to Opus quality requirements.
- **NEVER conflate speed preference with model tier** — Choosing Haiku solely for latency on a reasoning-heavy task produces wrong answers. **Why:** Speed and capability are separate dimensions; use the decision matrix first, then consider latency constraints.

## Usage Examples

**Selecting a model for a code review task:**
```bash
# Task: review 200-line TypeScript file for bugs
# Tier: balanced (Sonnet-class) — reasoning needed but not frontier
# Escalators: none (single file, no production risk flagged)
# Output: model alias + rationale
# -> Recommendation: Sonnet — single-file code review, moderate reasoning required
```

**Routing a summarization request:**
```bash
# Task: summarize 5 meeting notes into bullet points
# Tier: fast (Haiku-class) — no complex reasoning needed
# Escalators: none (<2K words, no stakeholder trade-offs)
# -> Recommendation: Haiku — simple text transformation, no judgment required
```

**Classifying a strategic planning task:**
```bash
# Task: design a market entry strategy for a new region
# Tier: reasoning (Opus-class)
# Escalators: strategic impact + cross-functional synthesis + ambiguity
# -> Recommendation: Opus — multiple escalators detected (strategic impact, ambiguity, cross-functional)
```

## References

- [Reference](references/reference.md) — extended decision matrix by file type and domain, cost/latency tradeoffs, edge cases, hybrid task patterns, and common mistakes

</content>
</invoke>