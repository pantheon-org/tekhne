# Remediation Plan: software-engineering/probe

---
plan_date: "2026-04-11"
skill_name: "software-engineering/probe"
source_audit: ".context/audits/software-engineering/probe/2026-04-11/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 115/140 (82%) | 126/140 (90%) |
| **Grade** | B | A |
| **Priority** | Medium | |
| **Effort** | M | |

**Focus Areas**:
- D5: Progressive Disclosure (11/15)
- D9: Eval Validation (7/20)

**Verdict**: Targeted improvements needed to reach grade A (+11 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Poor progressive disclosure | D5 (11/15) | High | Skill is too long or lacks refs |
| Missing or incomplete evals | D9 (7/20) | High | Skill not validated at runtime |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Progressive Disclosure - Priority: High

**Target**: Increase D5 from 11/15 to 13/15 (+2 points)

#### Step 1.1: Create Reference Files

Move detailed content to `references/` directory.

**Action**: Extract deep-dive content into separate files, keep SKILL.md as navigation hub.


### Phase 2: Eval Validation - Priority: High

**Target**: Increase D9 from 7/20 to 17/20 (+10 points)

#### Step 2.1: Create Eval Scenarios

Use the `creating-eval-scenarios` skill to generate evaluation scenarios.

**Action**: Run the eval scenario creation workflow to produce:
- `evals/instructions.json` - Extract all instructions from SKILL.md
- `evals/summary.json` - Coverage statistics (target >= 80%)
- `evals/scenario-N/` - 5 scenarios with task.md, criteria.json, capability.txt

#### Step 2.2: Run Evals

```bash
tessl eval run <tile-path>
tessl eval view-status <status_id> --json
```

#### Step 2.3: Validate Coverage

Verify `summary.json` shows `coverage_percentage >= 80` and all criteria.json files sum to 100.

---

## Verification Commands

```bash
# Re-run evaluation
./scripts/evaluate.sh software-engineering/probe --json --store

# Check target score achieved
./scripts/evaluate.sh software-engineering/probe --json | jq ".total >= 126"
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 126/140 |
| Grade | >= A |
| D5: Progressive Disclosure | >= 13/15 |
| D9: Eval Validation | >= 17/20 |

---

## Effort Estimate

| Phase | Effort | Time |
|-------|--------|------|
| Total | M | 2-4 hours |

---

## Dependencies

- None (standalone skill)

---

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/software-engineering/probe/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
