# Remediation Plan: moscow-prioritization

---
plan_date: "2026-03-02"
skill_name: "moscow-prioritization"
source_audit: ".context/audits/moscow-prioritization/2026-03-02/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 100/120 (83%) | 108/120 (90%) |
| **Grade** | B | A |
| **Priority** | Medium | |
| **Effort** | S | |

**Focus Areas**:
- D7: Pattern Recognition (6/10)
- D8: Practical Usability (11/15)

**Verdict**: Targeted improvements needed to reach grade A (+8 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Weak pattern recognition | D7 (6/10) | Medium | Skill trigger rate is low |
| Limited practical examples | D8 (11/15) | High | Agents struggle to apply skill |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Practical Usability - Priority: High

**Target**: Increase D8 from 11/15 to 13/15 (+2 points)

#### Step 1.1: Add Code Examples

Add executable code blocks with language tags.

**File**: `skills/moscow-prioritization/SKILL.md`

**Action**: Include bash/typescript examples with clear syntax highlighting.

---

## Verification Commands

```bash
# Re-run evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh moscow-prioritization --json --store

# Check target score achieved
sh skills/skill-quality-auditor/scripts/evaluate.sh moscow-prioritization --json | jq '.total >= 108'
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 108/120 |
| Grade | >= A |
| D8: Practical Usability | >= 13/15 |

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
git checkout HEAD~1 -- skills/moscow-prioritization/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
