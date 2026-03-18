# Remediation Plan: agentic-harness/agents-md

---
plan_date: "2026-03-18"
skill_name: "agentic-harness/agents-md"
source_audit: ".context/audits/agentic-harness/agents-md/2026-03-18/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 121/140 (86%) | 126/140 (90%) |
| **Grade** | B+ | A |
| **Priority** | Medium | |
| **Effort** | S | |

**Focus Areas**:
- D1: Knowledge Delta (15/20)
- D4: Specification Compliance (11/15)
- D8: Practical Usability (11/15)

**Verdict**: Targeted improvements needed to reach grade A (+5 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Low knowledge delta signals | D1 (15/20) | High | Skill may duplicate basic docs |
| Weak description field | D4 (11/15) | Medium | Skill discovery suffers |
| Limited practical examples | D8 (11/15) | High | Agents struggle to apply skill |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Practical Usability - Priority: High

**Target**: Increase D8 from 11/15 to 13/15 (+2 points)

#### Step 1.1: Add Code Examples

Add executable code blocks with language tags.

**File**: `skills/agentic-harness/agents-md/SKILL.md`

**Action**: Include bash/typescript examples with clear syntax highlighting.

---

## Verification Commands

```bash
# Re-run evaluation
./scripts/evaluate.sh agentic-harness/agents-md --json --store

# Check target score achieved
./scripts/evaluate.sh agentic-harness/agents-md --json | jq ".total >= 126"
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 126/140 |
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
git checkout HEAD~1 -- skills/agentic-harness/agents-md/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
