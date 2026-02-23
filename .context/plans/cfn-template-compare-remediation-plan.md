---
plan_date: 2026-02-23
skill_name: cfn-template-compare
source_audit: .context/audits/cfn-template-compare-audit-2026-02-22.md
---

# Remediation Plan: cfn-template-compare

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 88/120 (73%) | 102/120 (85%) |
| **Grade** | C | B+ |
| **Priority** | High | - |
| **Effort** | Medium (M) | - |

**Focus Areas**: Anti-pattern quality (D3), Progressive disclosure (D5), Practical usability (D8)

**Verdict**: Priority improvements required. Multiple dimensions need enhancement.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Anti-pattern quality weak | D3 (8/15) | High | Mistakes repeated |
| Progressive disclosure moderate | D5 (10/15) | Medium | Maintainability |
| Practical usability gaps | D8 (10/15) | Medium | Commands missing |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add explicit anti-patterns

**File**: `skills/cfn-template-compare/SKILL.md`

Add NEVER statements with WHY and BAD/GOOD examples.

---

### Phase 2: Progressive Disclosure (D5) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 2.1: Create references

Extract detailed content to references/ directory.

---

### Phase 3: Practical Usability (D8) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 3.1: Add Quick Commands

Add executable command examples.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh cfn-template-compare --json
bunx markdownlint-cli2 "skills/cfn-template-compare/**/*.md"
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D3 Anti-Pattern Quality | Score >= 13/15 |
| D5 Progressive Disclosure | Score >= 13/15 |
| D8 Practical Usability | Score >= 13/15 |
| Overall Score | >= 102/120 (B+) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Anti-patterns | M | 1 hour |
| Phase 2: Disclosure | S | 30 min |
| Phase 3: Commands | S | 30 min |
| **Total** | **M** | **2 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/cfn-template-compare/SKILL.md
```

## Notes

- Rating: **7/10** - Good structure, follows Format B template
- Has detailed code examples
- Has Estimated Effort table, Dependencies, Rollback Plan
- Minor: Could add Notes section
