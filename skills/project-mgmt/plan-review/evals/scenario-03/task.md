# Scenario 03: Plan Is Trivially Small -- Skip the Full 3-Reviewer Audit

## User Prompt

"Review this plan real quick:

```markdown
---
title: Bump the mise-pinned Bun version
type: PLAN
status: DRAFT
date: 2026-08-01
effort: S
value: LOW
themes: [TOOLING]
---

## Goal
Bump Bun from 1.2.x to 1.3.x in mise.toml and confirm bun test still passes.
```

"

## Expected Behavior

1. Recognize this plan is trivially small (1 short section, no phases, no
   steps beyond a single sentence) -- the overhead of spawning 3 subagent
   reviewers is not justified for a plan this size.
2. Do NOT spawn Technical/Strategic/Risk reviewers for this plan.
3. Do NOT ask the user which models to use, since no reviewers will be
   spawned.
4. Still perform the lightweight structural check (frontmatter validity),
   since that's a cheap, always-worth-doing step.
5. Give a single-pass opinion directly instead of a full consolidated
   report with Model Configuration / Critical Issues / etc. sections.
6. Explain briefly why the full audit was skipped.

## Success Criteria

- No subagents are spawned for this plan.
- No model-selection question is asked.
- Frontmatter is still checked (it's valid here: title/type/status/date/
  effort/value/themes all present).
- The agent gives a single, direct opinion (e.g. "looks fine, one version
  bump, low risk") rather than fabricating a 3-reviewer report.
- The agent states the reason the full audit was skipped (plan is too
  small / overhead not justified).

## Failure Conditions

- The agent spawns 3 full reviewers for a one-sentence plan anyway.
- The agent asks about model selection despite skipping the review.
- The agent fabricates a "Consolidated Review Report" with Model
  Configuration, Critical Issues, etc. for a plan this trivial.
- Frontmatter validation is skipped entirely with no comment.
