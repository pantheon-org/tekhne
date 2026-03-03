# Remediation Plan: cfn-template-compare

---
plan_date: "2026-03-02"
skill_name: "cfn-template-compare"
source_audit: ".context/audits/cfn-template-compare/2026-03-02/audit.json"
---

## Executive Summary

| Metric | Current | Target |
|--------|---------|--------|
| **Score** | 88/120 (73%) | 102/120 (85%) |
| **Grade** | C | B+ |
| **Priority** | High | |
| **Effort** | M | |

**Focus Areas**:
- D1: Knowledge Delta (15/20)
- D2: Mindset + Procedures (8/15)
- D3: Anti-Pattern Quality (8/15)
- D5: Progressive Disclosure (11/15)
- D6: Freedom Calibration (10/15)
- D8: Practical Usability (11/15)

**Verdict**: Targeted improvements needed to reach grade B+ (+14 points).

---

## Critical Issues

| Issue | Dimension | Severity | Impact |
|-------|-----------|----------|--------|
| Low knowledge delta signals | D1 (15/20) | High | Skill may duplicate basic docs |
| Missing mindset/procedures | D2 (8/15) | High | Agents lack decision frameworks |
| Insufficient anti-patterns | D3 (8/15) | High | Agents repeat common mistakes |
| Poor progressive disclosure | D5 (11/15) | High | Skill is too long or lacks refs |
| Imbalanced constraint language | D6 (10/15) | Medium | Over/under-prescriptive |
| Limited practical examples | D8 (11/15) | High | Agents struggle to apply skill |

---

## Detailed Remediation Steps

> **Note**: This is an auto-generated template. Review dimension scores and customize based on actual skill content.


### Phase 1: Anti-Pattern Quality - Priority: High

**Target**: Increase D3 from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add NEVER/ALWAYS Constraints

Add explicit anti-pattern warnings to prevent common mistakes.

**File**: `skills/cfn-template-compare/SKILL.md`

**Action**: Add section with BAD vs GOOD examples.


### Phase 2: Progressive Disclosure - Priority: High

**Target**: Increase D5 from 11/15 to 13/15 (+2 points)

#### Step 2.1: Create Reference Files

Move detailed content to `references/` directory.

**Action**: Extract deep-dive content into separate files, keep SKILL.md as navigation hub.


### Phase 3: Practical Usability - Priority: High

**Target**: Increase D8 from 11/15 to 13/15 (+2 points)

#### Step 3.1: Add Code Examples

Add executable code blocks with language tags.

**File**: `skills/cfn-template-compare/SKILL.md`

**Action**: Include bash/typescript examples with clear syntax highlighting.

---

## Verification Commands

```bash
# Re-run evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh cfn-template-compare --json --store

# Check target score achieved
sh skills/skill-quality-auditor/scripts/evaluate.sh cfn-template-compare --json | jq '.total >= 102'
```

---

## Success Criteria

| Criterion | Measurement |
|-----------|-------------|
| Overall Score | >= 102/120 |
| Grade | >= B+ |
| D3: Anti-Pattern Quality | >= 13/15 |
| D5: Progressive Disclosure | >= 13/15 |
| D8: Practical Usability | >= 13/15 |

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
git checkout HEAD~1 -- skills/cfn-template-compare/SKILL.md
```

---

## Notes

**Rating**: 6/10 (auto-generated template - requires customization)

**Assessment**: This is a baseline remediation plan. Review actual skill content to add specific, actionable steps with code examples.
