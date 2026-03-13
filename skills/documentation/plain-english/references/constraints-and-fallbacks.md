# Constraints and Fallback Paths

## Hard Rules

- ALWAYS state the audience explicitly before writing.
- ALWAYS define every acronym on first use — scan the full document after drafting.
- ALWAYS lead with key message (problem + impact + action) for executive audiences.
- NEVER hide decisions after background context; recommendation goes in paragraph one.
- NEVER use passive voice for required actions when ownership is known — name the owner.
- NEVER leave a deadline undefined for a required action; undated recommendations are ignored.

## Fallback Paths

Apply when information is missing. **State the fallback rule explicitly at the top of your output before writing the document.**

| Situation | Rule | Required output prefix |
|-----------|------|------------------------|
| Unknown audience | Default to manager-level clarity | `Audience: unknown. Applying manager-level clarity (fallback).` |
| Unknown terminology | Add inline glossary | `Unknown terminology detected. Adding inline definitions on first use.` |
| Conflicting priorities | Lead with risk + deadline for each | `Multiple critical issues — leading with risk and deadlines.` |

### Unknown Audience

Write `Audience: unknown. Applying manager-level clarity (fallback).` at the top of your output, then:

- Avoid jargon and translate all metrics to business terms.
- Keep depth moderate — more than a one-liner, less than a technical spec.
- Include owner and deadline for every action item.

### Unknown Terminology

When a term's plain-language equivalent is unclear:

1. Write the original term.
2. Add a parenthetical definition immediately after: "...using TLS (Transport Layer Security — the encryption protocol that keeps data private in transit)..."
3. If many terms are unknown, append a glossary at the end of the document.

### Conflicting Priorities

When two or more critical issues compete, open with:

```
IMMEDIATE ACTION REQUIRED — [N] issues, all deadlines within [timeframe].

Issue 1 — [Plain-language label] (Deadline: [specific time])
[One sentence: what happened, who is affected, what decision is needed.]

Issue 2 — [Plain-language label] (Deadline: [specific time])
[One sentence: what happened, who is affected, what decision is needed.]

Actions needed:
- [Owner] must [action] by [deadline].
- [Owner] must [action] by [deadline].
```

Do not merge the issues. Each must be independently understandable.

## Action Item Format Rule

Every action item — in any document type — MUST follow this pattern:

```
[Team or person name] must [specific action] by [concrete date or deadline].
```

If the owner is unknown, write `OWNER TBD` — never leave an action ownerless or undated.

Examples:

| BAD | GOOD |
|-----|------|
| "The database should be optimized." | "Database team (Alex Chen) must optimize query performance by March 15." |
| "Monitoring needs to be improved." | "DevOps (Sam Park) must configure alerting thresholds before the Q2 release (May 1)." |
| "Legal review is recommended." | "Legal team (Maria Kovacs) must complete contract review by Friday 17:00." |

## Audience Depth Guide

| Audience | Jargon tolerance | Decision format | Metric translation |
|----------|-----------------|-----------------|-------------------|
| Executive (CEO/CFO/board) | Zero — all terms translated | Single sentence + approve/reject | Business impact only (cost, time, risk) |
| Manager/VP | Minimal — common business terms OK | Recommendation + options | Outcomes (e.g., "4× faster") |
| Compliance/legal | Domain terms OK; tech terms translated | Risk + regulatory impact + action | Deadlines and penalties |
| Cross-functional | Light jargon with parenthetical definitions | Problem + impact + owner | Mixed — follow audience mix |
