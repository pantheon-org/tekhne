# Remediation Plan: documentation/conventional-commits

---
plan_date: "2026-04-08"
skill_name: "documentation/conventional-commits"
source_audit: ".context/audits/documentation/conventional-commits/2026-04-08/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 122/140 (87%) | 126/140 (90%) |
| **Grade** | B+ | A |
| **Priority** | Medium | |
| **Effort** | S | |

**Focus Areas**:
- D5: Progressive Disclosure (11/15)

**Verdict**: Targeted improvements needed to reach grade A (+4 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Poor progressive disclosure | D5 (11/15) | High | Skill is too long or lacks refs |

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
./scripts/evaluate.sh documentation/conventional-commits --json --store

# Check target score achieved
./scripts/evaluate.sh documentation/conventional-commits --json | jq ".total >= 126"
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 126/140 |
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
git checkout HEAD~1 -- skills/documentation/conventional-commits/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
