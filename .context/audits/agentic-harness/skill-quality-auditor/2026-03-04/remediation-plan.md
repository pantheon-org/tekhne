# Remediation Plan: agentic-harness/skill-quality-auditor

---
plan_date: "2026-03-04"
skill_name: "agentic-harness/skill-quality-auditor"
source_audit: ".context/audits/agentic-harness/skill-quality-auditor/2026-03-04/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 121/140 (86%) | 126/140 (90%) |
| **Grade** | B+ | A |
| **Priority** | Medium | |
| **Effort** | S | |

**Focus Areas**:
- D4: Specification Compliance (10/15)
- D5: Progressive Disclosure (11/15)
- D7: Pattern Recognition (6/10)

**Verdict**: Targeted improvements needed to reach grade A (+5 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Weak description field | D4 (10/15) | Medium | Skill discovery suffers |
| Poor progressive disclosure | D5 (11/15) | High | Skill is too long or lacks refs |
| Weak pattern recognition | D7 (6/10) | Medium | Skill trigger rate is low |

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
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh agentic-harness/skill-quality-auditor --json --store

# Check target score achieved
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh agentic-harness/skill-quality-auditor --json | jq ".total >= 126"
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
git checkout HEAD~1 -- skills/agentic-harness/skill-quality-auditor/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
