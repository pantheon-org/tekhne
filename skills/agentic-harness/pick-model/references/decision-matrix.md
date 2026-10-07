# Decision Matrix

Base tier by task type, before escalators. Match what the task demands, not how long it is. Apply the escalators in [escalators](escalators.md) afterwards.

## Technical tasks

| Tier | Task types |
|---|---|
| 🟢 **Haiku** | Simple transforms, formatting, regex, typo fixes, status queries, template fills, data extraction, factual lookup with no reasoning, file conversion |
| 🟡 **Sonnet** | Single-file coding, bug fixes, code review, moderate debugging, test writing, PR review, standard refactoring, technical docs, API integration with known patterns |
| 🔴 **Opus** | Multi-file refactors (3 or more files), architecture and design decisions, multi-system debugging, framework migration, security audit, novel algorithm design, system design with trade-offs |

## Business and strategy tasks

| Tier | Task types |
|---|---|
| 🟢 **Haiku** | Summaries under 2K words, data extraction, status reports, simple translation, template filling, meeting-note formatting |
| 🟡 **Sonnet** | Content creation (blog, email, docs), research summaries, competitive analysis, standard business writing, persuasive proposals, marketing copy, customer communications |
| 🔴 **Opus** | Strategic planning, business model design, M&A analysis, organisational design, change management, market entry, crisis response, stakeholder management with competing interests, long-form reports over 2K words, executive presentations with nuance |

## Creative and analysis tasks

| Tier | Task types |
|---|---|
| 🟢 **Haiku** | Basic formatting, simple data visualisation suggestions, straightforward categorisation |
| 🟡 **Sonnet** | Creative writing, single-framework brainstorming, persona development, user research synthesis, A/B test analysis, survey analysis |
| 🔴 **Opus** | Multi-framework brainstorming (for example SCAMPER plus Starbursting plus trade-off analysis), cross-session pattern detection, bias identification, retrospective analysis, ethical reasoning, strategic foresight, scenario planning |

## Commands, skills and agents

| Tier | Task types |
|---|---|
| 🟢 **Haiku** | Simple conversions (PDF, EPUB), format checks, simple utilities, minimal reasoning |
| 🟡 **Sonnet** | Standard workflows, context management, serialisation, most skills and commands (the default) |
| 🔴 **Opus** | Strategic analysis (brainstorms, retrospectives), multi-framework reasoning, high-stakes decisions, pattern detection across sessions |

## Choosing between tiers

- **Haiku or Sonnet**: does the task need any reasoning or judgement? If yes, choose Sonnet.
- **Sonnet or Opus**: are there trade-offs to balance or several valid approaches? If yes, choose Opus.
- **Quality first**: customer-facing, executive, production and irreversible work goes one tier up.
- **Cost first**: batch processing, exploration and drafts start one tier lower, because a retry is cheap.
- **Latency**: use the fast and balanced tiers for interactive feedback. Batch and asynchronous work can trade speed for quality.

Rank cost and speed relatively (lowest to highest, fastest to slowest). Take exact prices and multipliers from current vendor pricing, because they change between releases.
