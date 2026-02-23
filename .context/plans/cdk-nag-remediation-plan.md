---
plan_date: 2026-02-23
skill_name: cdk-nag
source_audit: .context/audits/cdk-nag-audit-2026-02-22.md
status: completed
completed_date: 2026-02-23
---

# Remediation Plan: cdk-nag

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 91/120 (75%) | 102/120 (85%) |
| **Grade** | C+ | B+ |
| **Priority** | Medium | - |
| **Effort** | Medium (M) | - |

**Focus Areas**: Anti-pattern quality (D3), Practical usability (D8), Pattern recognition (D7)

**Verdict**: Targeted improvements recommended. Good baseline with room for enhancement.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Anti-pattern quality moderate | D3 (10/15) | Medium | Mistakes may be repeated |
| Moderate practical usability | D8 (10/15) | Medium | Commands could be clearer |
| Pattern recognition moderate | D7 (7/10) | Medium | Skill may not activate |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 1.1: Enhance anti-patterns

**File**: `skills/cdk-nag/SKILL.md`

Add more specific NEVER statements with BAD/GOOD examples.

---

### Phase 2: Practical Usability (D8) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 2.1: Add Quick Commands

Add more executable command examples.

---

### Phase 3: Pattern Recognition (D7) - Priority: Medium

**Target**: Increase from 7/10 to 9/10 (+2 points)

#### Step 3.1: Expand triggers

Add more trigger phrases to frontmatter.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh cdk-nag --json
bunx markdownlint-cli2 "skills/cdk-nag/**/*.md"
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| D8 Practical Usability | Score >= 13/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| Overall Score | >= 102/120 (B+) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Anti-patterns | S | 30 min |
| Phase 2: Commands | S | 30 min |
| Phase 3: Triggers | S | 20 min |
| **Total** | **S** | **1.5 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/cdk-nag/SKILL.md
```

## Notes

- Rating: **7/10** - Already follows Format B template well
- Has detailed code examples
- Has Estimated Effort table, Dependencies, Rollback Plan
- Minor: Could add Notes section for consistency
