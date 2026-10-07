---
name: pick-model
description: Recommend the cheapest Claude model tier (Haiku, Sonnet, Opus) that still meets a task's quality bar, using a task-type matrix plus complexity escalators (scope, stakes, ambiguity, stakeholders). Use when asked "which model", "pick model", "model for", or before starting a costly task. Covers technical, business, creative work.
model: haiku
---

# Pick Model

Classify the task, apply the escalators, then recommend the cheapest Claude tier that meets the quality requirement.

## Mindset

- Right-size and do not default up. Keep to the cheapest tier that meets the quality bar. Escalating after a failed attempt costs one retry; an Opus habit costs every call.
- Judge reasoning demand, not volume. Pass a long document to a summariser at the same tier as a short one.
- Name tiers (fast, balanced, reasoning) before model names. Model names are retired and renamed; tier meaning lasts.
- Escalate with a stated reason. Always name the signal that moved the tier.

## When to Use

- Use it when a user asks which model fits a task, before the work starts.
- Use it when an agent must assign a model to a sub-task without a human in the loop.
- Use it when a pipeline or batch job overspends on a frontier model for simple work.
- Use it when the cost against capability trade-off must be explicit, for example in production routing.

## When Not to Use

- Do not use it when infrastructure or an explicit user instruction already fixes the model.
- Do not use it once the task is finished, because the choice is moot.
- Do not use it for cross-vendor comparisons or live pricing, which this skill does not hold.
- Do not use it when a fine-tuned or domain-specific model decides the outcome, not the tier.

## Procedure

1. Read the task from `$ARGUMENTS`. Ask for it when the argument is empty.
2. Match the task type to a base tier in the matrix below. Match what the task demands, not how long it is.
3. Run every escalator check below. Move up exactly one tier when any escalator fires, capped at Opus.
4. Apply the tie-breaks below to a borderline call.
5. Adjust for cost and latency only after the tier is settled.
6. Make the recommendation in the output template below. Always name the escalator or matrix row that decided it.

### Base tier by task type

| Tier | Typical work |
|---|---|
| 🟢 **Haiku** (fast) | Typo fixes, formatting, regex, extraction, templated fills, file conversion, factual lookup, summaries under 2K words |
| 🟡 **Sonnet** (balanced) | Single-file code and bug fixes, code review, tests, standard refactors, technical and business writing, research summaries (the default for skills and commands) |
| 🔴 **Opus** (reasoning) | Multi-file refactors, architecture, multi-system debugging, security audits, strategy, long-form reports, multi-framework analysis |

Read [the full matrix](references/decision-matrix.md) for technical, business, creative and command or agent tasks.

### Escalators

Move up one tier when the task shows any of these signals.

- Check **ambiguity**: requirements are underspecified or have several valid readings.
- Check **scope**: the change spans 3 or more files, systems or components.
- Check **stakes**: production, security, data loss or regulatory exposure.
- Check **novelty**: no established pattern exists.
- Check **stakeholders**: competing interests need balancing.
- Check **irreversibility**: strategic, long-term or organisational consequences.
- Check **sensitivity**: layoffs, executive messaging, crisis response.
- Check **synthesis**: the work crosses domains such as tech, business and legal.
- Check **cognition**: pattern detection across sessions, bias spotting, ethical reasoning or two or more analytical frameworks at once.

See [escalators](references/escalators.md) for edge cases and borderline readings.

### Tie-breaks

- Choose Sonnet over Haiku when the task needs any judgement.
- Choose Opus over Sonnet when trade-offs must be balanced or several valid approaches exist.
- Go one tier up when the output is customer-facing, production or irreversible.
- Start one tier lower when a retry is cheap, as in drafts, batch runs and exploration.

## Output Format

```text
[emoji] **[Model]**: [one-line reason naming the deciding signal]

Cost: [lowest|medium|highest] | Speed: [fastest|medium|slowest] | Confidence: [high|medium|low]

Tip: [only when a cheaper or stronger tier fits a stated condition]
```

```text
🔴 **Opus**: refactor touches 15 files and changes the auth architecture (Scope escalator)

Cost: highest | Speed: slowest | Confidence: high

Tip: Sonnet is enough if the scope shrinks to 2 files after exploration.
```

Set confidence to high when the task matches one tier clearly, medium when it sits between two tiers, and low when the task is too vague to classify. Ask one clarifying question instead of guessing when confidence is low.

## Anti-Patterns

- **NEVER default to the most powerful model for every task.** Oversized models inflate cost with no quality gain on simple work. Match the tier to the reasoning demand. **Why:** Haiku handles classification and routing at a fraction of the cost.
- **NEVER classify on the task label alone.** Check the escalators first. **Why:** one missed signal, such as a security-sensitive refactor, turns a Sonnet task into an Opus task.

  ```text
  # BAD: label only
  "refactor the auth module" -> Sonnet (looks like single-file coding)

  # GOOD: escalators checked
  "refactor the auth module": Scope (3+ files) + Stakes (credentials) -> Opus
  ```

- **NEVER hardcode model identifiers in agent workflows.** **Why:** providers retire models, and a pinned identifier fails on the day it is retired. Use tier aliases and resolve them at runtime.

  ```yaml
  # BAD: pinned identifiers that break on retirement
  routing:
    summariser: claude-3-haiku-20240307
    reviewer: claude-3-opus-20240229

  # GOOD: tiers resolved to current models at runtime
  routing:
    summariser: fast
    reviewer: reasoning
  ```

- **NEVER pick a model from benchmark leaderboards alone.** Test the shortlisted tier on a sample of the real workload. **Why:** benchmark tasks differ from your workload, so real performance depends on prompt structure, context length and domain.
- **NEVER choose Haiku for production-critical work to save money.** Apply the stakes escalator instead. **Why:** this is the false economy pitfall, because one outage or rework cycle costs far more than the saving.
- **NEVER treat speed as a tier.** **Why:** a fast model on a reasoning-heavy task returns wrong answers faster. Settle the tier first, then apply latency.
- **NEVER override an explicit user model choice.** Recommend only when no model is named. **Why:** the user often holds constraints, such as budget or latency, that the task text does not show.
- **NEVER quote prices or speed multipliers as fact.** **Why:** they change between releases. Describe cost and speed as relative ranks. Send readers to current vendor pricing for exact figures.

## Worked Examples

```text
Task: "summarise 5 meeting notes into bullets"
Matrix: Haiku. Escalators: none (under 2K words, no trade-offs).
Result: 🟢 Haiku, because it is plain text transformation.
```

```text
Task: "fix a typo in the production payment config"
Matrix: Haiku. Escalators: Stakes (production, payments) -> one tier up.
Result: 🟡 Sonnet. A typo alone is Haiku work; the stakes decide this one.
```

```text
Task: "plan market entry for a new region"
Matrix: Sonnet. Escalators: Irreversibility, Synthesis, Ambiguity -> one tier up, capped at Opus.
Result: 🔴 Opus. Three signals fired but the move is one tier, not three.
```

More cases are in [worked examples](references/examples.md).

## References

- [Decision matrix](references/decision-matrix.md): base tiers by technical, business, creative and command or agent task
- [Escalators](references/escalators.md): signal definitions, borderline readings and what does not count
- [Worked examples](references/examples.md): classified tasks by domain with the deciding signal
- [Reference](references/reference.md): model characteristics, file-type and domain matrices, cost and latency trade-offs, hybrid patterns and common mistakes
