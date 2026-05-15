# Remediation Plan: documentation/research/triage-paper

---
plan_date: "2026-04-08"
skill_name: "documentation/research/triage-paper"
source_audit: ".context/audits/documentation/research/triage-paper/2026-04-08/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 119/140 (85%) | 126/140 (90%) |
| **Grade** | B+ | A |
| **Priority** | Medium | |
| **Effort** | S | |

**Focus Areas**:
- D9: Eval Validation (8/20)

**Verdict**: Targeted improvements needed to reach grade A (+7 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Missing or incomplete evals | D9 (8/20) | High | Skill not validated at runtime |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Eval Validation - Priority: High

**Target**: Increase D9 from 8/20 to 17/20 (+9 points)

#### Step 1.1: Create Eval Scenarios

Use the `creating-eval-scenarios` skill to generate evaluation scenarios.

**Action**: Run the eval scenario creation workflow to produce:
- `evals/instructions.json` - Extract all instructions from SKILL.md
- `evals/summary.json` - Coverage statistics (target >= 80%)
- `evals/scenario-N/` - 5 scenarios with task.md, criteria.json, capability.txt

#### Step 1.2: Run Evals

```bash
tessl eval run <tile-path>
tessl eval view-status <status_id> --json
```

#### Step 1.3: Validate Coverage

Verify `summary.json` shows `coverage_percentage >= 80` and all criteria.json files sum to 100.

---

## Verification Commands

```bash
# Re-run evaluation
./scripts/evaluate.sh documentation/research/triage-paper --json --store

# Check target score achieved
./scripts/evaluate.sh documentation/research/triage-paper --json | jq ".total >= 126"
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 126/140 |
| Grade | >= A |
| D9: Eval Validation | >= 17/20 |

---

## Effort Estimate

| Phase | Effort | Time |
|-------|--------|------|
| Total | S | 1-2 hours |

---

## Dependencies

- None (standalone skill)

---

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/documentation/research/triage-paper/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
