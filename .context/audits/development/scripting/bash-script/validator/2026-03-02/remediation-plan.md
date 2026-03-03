# Remediation Plan: bash-script-validator

---
plan_date: "2026-03-02"
skill_name: "bash-script-validator"
source_audit: ".context/audits/bash-script-validator/2026-03-02/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 99/120 (82%) | 108/120 (90%) |
| **Grade** | B | A |
| **Priority** | Medium | |
| **Effort** | S | |

**Focus Areas**:
- D2: Mindset + Procedures (10/15)
- D5: Progressive Disclosure (11/15)
- D6: Freedom Calibration (10/15)

**Verdict**: Targeted improvements needed to reach grade A (+9 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Missing mindset/procedures | D2 (10/15) | High | Agents lack decision frameworks |
| Poor progressive disclosure | D5 (11/15) | High | Skill is too long or lacks refs |
| Imbalanced constraint language | D6 (10/15) | Medium | Over/under-prescriptive |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Progressive Disclosure - Priority: High

**Target**: Increase D5 from 11/15 to 13/15 (+2 points)

#### Step 1.1: Create Reference Files

Move detailed content to `references/` directory.

**Action**: Extract deep-dive content into separate files, keep SKILL.md as navigation hub.

---

## Verification Commands

```bash
# Re-run evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh bash-script-validator --json --store

# Check target score achieved
sh skills/skill-quality-auditor/scripts/evaluate.sh bash-script-validator --json | jq '.total >= 108'
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 108/120 |
| Grade | >= A |
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
git checkout HEAD~1 -- skills/bash-script-validator/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
