---
domain: confused
verb: decompose
constraint-type: unknown
problem: "{problem statement}"
scale: "{boulder|pebble}"
---

# Troubleshoot to Frame-Problem Handoff

## Thinking Trail

- **Considered**: {diagnostic approaches tried during troubleshooting}
- **Rejected**: {root causes eliminated}
- **Surprised by**: {problem is deeper than a simple degradation}
- **Models used**: Search-first diagnostics, systematic elimination
- **Constraints discovered**: {problem exceeds troubleshooting scope — needs re-framing}

## Decisions

1. **Troubleshoot outcome**: Stabilized but root cause requires broader change
2. **Domain shift**: Complicated → Confused (re-classify for broader approach)
3. **Rationale**: {fix applied but underlying issue needs architectural/process change}

## Actions Taken

- Symptom addressed: {what was stabilized}
- Root cause identified: {deeper issue requiring re-framing}
- Learnings saved: troubleshoot learnings database updated

## Output

Stabilized: {what was fixed}
Root cause: {deeper issue identified}
Re-frame needed: {why broader approach required}

## Domain Transition

**From**: Complicated (troubleshoot) → **To**: Confused (frame-problem)
Constraint shift: governing → unknown. Troubleshooting stabilized the symptom but revealed the root cause is a different class of problem. Need fresh classification.

## For /frame-problem

- **Context**: Troubleshooting stabilized {symptom} but root cause is {deeper issue}
- **What's fixed**: {immediate stabilization — do not re-troubleshoot}
- **What remains**: {root cause requiring different approach}
- **New signals**: {what troubleshooting revealed about the true problem}
- **Do NOT**: Re-diagnose the symptom — it's stabilized. Classify the root cause.

## Accumulated Context

Token guidance: target 300 tokens inline. For depth, use **references** — point to saved thinking files rather than embedding full content.
Soft cap: 600 tokens inline per handoff. If you need more, move detail to a knowledge file and reference it.
Accumulated cap: 800 tokens across a chain — compress to 200 at cap (keep: decisions, constraints, rejected paths). References do NOT count toward the cap.
