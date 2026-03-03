# Remediation Plan: logql-generator

---
plan_date: "2026-03-02"
skill_name: "logql-generator"
source_audit: ".context/audits/logql-generator/2026-03-02/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 94/120 (78%) | 102/120 (85%) |
| **Grade** | C+ | B+ |
| **Priority** | High | |
| **Effort** | S | |

**Focus Areas**:
- D3: Anti-Pattern Quality (8/15)
- D5: Progressive Disclosure (10/15)
- D6: Freedom Calibration (10/15)

**Verdict**: Targeted improvements needed to reach grade B+ (+8 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Insufficient anti-patterns | D3 (8/15) | High | Agents repeat common mistakes |
| Poor progressive disclosure | D5 (10/15) | High | Skill is too long or lacks refs |
| Imbalanced constraint language | D6 (10/15) | Medium | Over/under-prescriptive |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Anti-Pattern Quality - Priority: High

**Target**: Increase D3 from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add NEVER/ALWAYS Constraints

Add explicit anti-pattern warnings to prevent common mistakes.

**File**: `skills/logql-generator/SKILL.md`

**Action**: Add section with BAD vs GOOD examples.


### Phase 2: Progressive Disclosure - Priority: High

**Target**: Increase D5 from 10/15 to 13/15 (+3 points)

#### Step 2.1: Create Reference Files

Move detailed content to `references/` directory.

**Action**: Extract deep-dive content into separate files, keep SKILL.md as navigation hub.

---

## Verification Commands

```bash
# Re-run evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh logql-generator --json --store

# Check target score achieved
sh skills/skill-quality-auditor/scripts/evaluate.sh logql-generator --json | jq '.total >= 102'
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 102/120 |
| Grade | >= B+ |
| D3: Anti-Pattern Quality | >= 13/15 |
| D5: Progressive Disclosure | >= 13/15 |

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
git checkout HEAD~1 -- skills/logql-generator/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
