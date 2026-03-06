# Phase 2: Conventional Commits — Assess and Resolve

**Duration:** 30-45 minutes
**Purpose:** Determine whether conventional-commits needs hub conversion or is already complete

## Background

The audit flagged this skill at 636 lines. Current SKILL.md is 109 lines — a significant reduction has
already occurred. It has a tile.json but no references/ directory. The question: is it done, or does
it need hub conversion?

## Decision Criteria

Run an audit to determine the current score:

```bash
bun cli/index.ts audit skill documentation/conventional-commits
```

**Branch A — Score ≥ 96/120 (B grade):**
The skill is complete. No hub conversion needed. Close out this phase with a note.

**Branch B — Score < 96/120:**
The skill likely needs either content enrichment or hub conversion. Inspect the audit's dimensional
scores to determine which.

## Tasks (Branch A — Score ≥ 96)

- [ ] Record the score in this file
- [ ] Mark phase complete — no further action

## Tasks (Branch B — Score < 96)

### 1. Read the Audit Dimensional Breakdown

Identify which dimensions score lowest (likely D2 descriptions, D5 anti-patterns, or D6 scope).

### 2. Determine Remediation

**If the issue is content depth/richness** (not size):
- Add concrete examples, edge cases, or tooling guidance to the existing SKILL.md
- Do NOT create unnecessary references/ for a 109-line skill

**If the skill needs hub conversion** (score penalized for size or scope):
- Create `references/` with focused sub-topics (e.g., tooling-setup.md, changelog-automation.md)
- Rewrite SKILL.md as a Navigation Hub per the same pattern as Phase 1

**Suggested split if conversion needed:**

| Topic | Target reference file |
|-------|-----------------------|
| Commit tooling: commitlint, husky, semantic-release | `references/tooling-setup.md` |
| CHANGELOG and release automation workflows | `references/changelog-automation.md` |
| CI enforcement and GitHub Actions patterns | `references/ci-enforcement.md` |

### 3. Validate

```bash
bun cli/index.ts audit skill documentation/conventional-commits
```

**Success Criteria:**
- [ ] Score documented
- [ ] Remediation applied (or explicitly deferred with justification)
- [ ] Post-remediation score ≥ 96/120 if work was done

## Outputs

- Audit score (pre and post if remediation applied)
- Decision record: hub conversion yes/no with rationale

## Next Phase

[Phase 3: Reference File Audit](../phase-3-reference-audit/)
