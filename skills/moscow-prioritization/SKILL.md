---
name: moscow-prioritization
description: Prioritize product requirements with the MoSCoW framework in a deterministic way. Use when teams need to define MVP scope, sequence releases, resolve stakeholder conflicts, prevent scope creep, or rebalance backlog under time, budget, or staffing constraints. Keywords: moscow, must should could wont, requirement prioritization, backlog, mvp, release planning, scope control, stakeholder alignment.
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
---

# MoSCoW Prioritization

## When to Use

Use this skill when you need to rank requirements for a release.

## When Not to Use

Do not use this skill for task-level estimation, bug triage, or sprint capacity planning by itself.

## Principles

1. `Must` means release failure if missing.
2. `Should` means high value but deferrable.
3. `Could` means optional enhancement.
4. `Won't` means explicitly out of scope for this release.

## Deterministic Workflow

1. Collect candidate requirements in one list.
2. Confirm release constraints (date, budget, team).
3. Categorize each item with the decision tree in `references/categorization-decision-tree.md`.
4. Challenge every `Must` using failure-focused questions.
5. Rebalance if `Must` work exceeds 60% of effort.
6. Publish final table with owners and review date.

## Quick Commands

### Create a workshop draft

```bash
cp skills/moscow-prioritization/references/facilitator-workshop-template.md .context/moscow-workshop-draft.md
```

Expected result: reusable working document for prioritization.

### Create a decision log

```bash
cat > .context/moscow-decisions-$(date +%Y-%m-%d).md <<'MD'
# MoSCoW Decision Log

| Requirement | Category | Rationale | Owner |
| --- | --- | --- | --- |
MD
```

Expected result: traceable decisions with ownership.

### Validate skill quality after edits

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh moscow-prioritization --json
```

Expected result: updated score and grade for this skill.

### Lint this skill documentation

```bash
bunx markdownlint-cli2 "skills/moscow-prioritization/**/*.md"
```

Expected result: no markdownlint errors.

## Anti-Patterns

### NEVER label stakeholder preference as `Must`

**WHY:** Preference is not the same as release-critical need.

**BAD:** "Executive asked for dark mode, so it is Must."
**GOOD:** "Dark mode is Should unless release fails without it."

**Consequence:** `Must` list inflates and blocks true essentials.

### NEVER skip explicit `Won't` items

**WHY:** Missing exclusions invite silent scope creep.

**BAD:** Keep backlog open-ended with no rejected items.
**GOOD:** Record rejected items with revisit date.

**Consequence:** Team repeatedly reopens settled decisions.

### NEVER finalize priorities without constraint checks

**WHY:** Categories are invalid without timeline and capacity context.

**BAD:** Categorize before confirming release limits.
**GOOD:** Validate date, staffing, and budget first.

**Consequence:** Plans collapse during execution.

### NEVER keep categories without rationale

**WHY:** Unjustified categories cannot be defended or audited.

**BAD:** Spreadsheet with only item and category.
**GOOD:** Include business outcome and risk rationale.

**Consequence:** Priorities are re-litigated in every review.

## References

- `references/categorization-decision-tree.md`
- `references/facilitator-workshop-template.md`
- `references/effort-balancing-and-tradeoffs.md`
