# Remediation Plan: project-mgmt/implementation-planner

---
plan_date: "2026-03-11"
skill_name: "project-mgmt/implementation-planner"
source_audit: ".context/audits/project-mgmt/implementation-planner/2026-03-11/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 119/140 (85%) | 126/140 (90%) |
| **Grade** | B+ | A |
| **Priority** | Medium | |
| **Effort** | S | |

**Focus Areas**:
- D1: Knowledge Delta (14/20)
- D5: Progressive Disclosure (10/15)

**Verdict**: Targeted improvements needed to reach grade A (+7 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Low knowledge delta signals | D1 (14/20) | High | Skill may duplicate basic docs |
| Poor progressive disclosure | D5 (10/15) | High | Skill is too long or lacks refs |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Progressive Disclosure - Priority: High

**Target**: Increase D5 from 10/15 to 13/15 (+3 points)

#### Step 1.1: Create Reference Files

Move detailed content to `references/` directory.

**Action**: Extract deep-dive content into separate files, keep SKILL.md as navigation hub.

---

## Verification Commands

```bash
# Re-run evaluation
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh project-mgmt/implementation-planner --json --store

# Check target score achieved
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh project-mgmt/implementation-planner --json | jq ".total >= 126"
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
git checkout HEAD~1 -- skills/project-mgmt/implementation-planner/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
