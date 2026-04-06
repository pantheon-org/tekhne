# Remediation Plan: project-mgmt/pr-decomposition

---
plan_date: "2026-04-03"
skill_name: "project-mgmt/pr-decomposition"
source_audit: ".context/audits/project-mgmt/pr-decomposition/2026-04-03/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 112/140 (80%) | 126/140 (90%) |
| **Grade** | B | A |
| **Priority** | Medium | |
| **Effort** | M | |

**Focus Areas**:
- D1: Knowledge Delta (15/20)
- D8: Practical Usability (8/15)
- D9: Eval Validation (11/20)

**Verdict**: Targeted improvements needed to reach grade A (+14 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Low knowledge delta signals | D1 (15/20) | High | Skill may duplicate basic docs |
| Limited practical examples | D8 (8/15) | High | Agents struggle to apply skill |
| Missing or incomplete evals | D9 (11/20) | High | Skill not validated at runtime |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Practical Usability - Priority: High

**Target**: Increase D8 from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add Code Examples

Add executable code blocks with language tags.

**File**: `skills/project-mgmt/pr-decomposition/SKILL.md`

**Action**: Include bash/typescript examples with clear syntax highlighting.


### Phase 2: Eval Validation - Priority: High

**Target**: Increase D9 from 11/20 to 17/20 (+6 points)

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
./scripts/evaluate.sh project-mgmt/pr-decomposition --json --store

# Check target score achieved
./scripts/evaluate.sh project-mgmt/pr-decomposition --json | jq ".total >= 126"
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 126/140 |
| Grade | >= A |
| D8: Practical Usability | >= 13/15 |
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
git checkout HEAD~1 -- skills/project-mgmt/pr-decomposition/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
