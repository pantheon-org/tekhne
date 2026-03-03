# Remediation Plan: github-copilot-models

---
plan_date: "2026-03-02"
skill_name: "github-copilot-models"
source_audit: ".context/audits/github-copilot-models/2026-03-02/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 87/120 (72%) | 102/120 (85%) |
| **Grade** | C | B+ |
| **Priority** | High | |
| **Effort** | M | |

**Focus Areas**:
- D1: Knowledge Delta (15/20)
- D2: Mindset + Procedures (8/15)
- D3: Anti-Pattern Quality (8/15)
- D4: Specification Compliance (11/15)
- D7: Pattern Recognition (6/10)

**Verdict**: Targeted improvements needed to reach grade B+ (+15 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Low knowledge delta signals | D1 (15/20) | High | Skill may duplicate basic docs |
| Missing mindset/procedures | D2 (8/15) | High | Agents lack decision frameworks |
| Insufficient anti-patterns | D3 (8/15) | High | Agents repeat common mistakes |
| Weak description field | D4 (11/15) | Medium | Skill discovery suffers |
| Weak pattern recognition | D7 (6/10) | Medium | Skill trigger rate is low |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Anti-Pattern Quality - Priority: High

**Target**: Increase D3 from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add NEVER/ALWAYS Constraints

Add explicit anti-pattern warnings to prevent common mistakes.

**File**: `skills/github-copilot-models/SKILL.md`

**Action**: Add section with BAD vs GOOD examples.

---

## Verification Commands

```bash
# Re-run evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh github-copilot-models --json --store

# Check target score achieved
sh skills/skill-quality-auditor/scripts/evaluate.sh github-copilot-models --json | jq '.total >= 102'
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 102/120 |
| Grade | >= B+ |
| D3: Anti-Pattern Quality | >= 13/15 |

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
git checkout HEAD~1 -- skills/github-copilot-models/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
