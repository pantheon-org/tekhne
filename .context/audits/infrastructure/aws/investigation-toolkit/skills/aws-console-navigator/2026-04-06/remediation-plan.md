# Remediation Plan: infrastructure/aws/investigation-toolkit/skills/aws-console-navigator

---
plan_date: "2026-04-06"
skill_name: "infrastructure/aws/investigation-toolkit/skills/aws-console-navigator"
source_audit: ".context/audits/infrastructure/aws/investigation-toolkit/skills/aws-console-navigator/2026-04-06/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 122/140 (87%) | 126/140 (90%) |
| **Grade** | B+ | A |
| **Priority** | Medium | |
| **Effort** | S | |

**Focus Areas**:
- D8: Practical Usability (8/15)

**Verdict**: Targeted improvements needed to reach grade A (+4 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Limited practical examples | D8 (8/15) | High | Agents struggle to apply skill |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Practical Usability - Priority: High

**Target**: Increase D8 from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add Code Examples

Add executable code blocks with language tags.

**File**: `skills/infrastructure/aws/investigation-toolkit/skills/aws-console-navigator/SKILL.md`

**Action**: Include bash/typescript examples with clear syntax highlighting.

---

## Verification Commands

```bash
# Re-run evaluation
./scripts/evaluate.sh infrastructure/aws/investigation-toolkit/skills/aws-console-navigator --json --store

# Check target score achieved
./scripts/evaluate.sh infrastructure/aws/investigation-toolkit/skills/aws-console-navigator --json | jq ".total >= 126"
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
git checkout HEAD~1 -- skills/infrastructure/aws/investigation-toolkit/skills/aws-console-navigator/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
