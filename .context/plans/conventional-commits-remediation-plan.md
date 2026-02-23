---
plan_date: 2026-02-23
skill_name: conventional-commits
source_audit: .context/audits/conventional-commits-audit-2026-02-22.md
---

# Remediation Plan: conventional-commits

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 87/120 (72%) | 100/120 (83%) |
| **Grade** | C | B |
| **Priority** | High | - |
| **Effort** | Medium (M) | - |

**Focus Areas**: Progressive disclosure (D5), Anti-pattern quality (D3), Pattern recognition (D7)

**Verdict**: Priority improvements required. Multiple dimensions need enhancement.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Progressive disclosure weak | D5 (9/15) | High | Maintainability concerns |
| Anti-pattern quality weak | D3 (9/15) | High | Common mistakes repeated |
| Pattern recognition moderate | D7 (7/10) | Medium | Skill may not activate |
| Moderate practical usability | D8 (10/15) | Medium | Commands could be clearer |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - Priority: High

**Target**: Increase from 9/15 to 14/15 (+5 points)

#### Step 1.1: Create references

**File**: `skills/conventional-commits/references/`

Extract detailed content to reference files.

---

### Phase 2: Anti-Pattern Quality (D3) - Priority: High

**Target**: Increase from 9/15 to 14/15 (+5 points)

#### Step 2.1: Add explicit anti-patterns

**File**: `skills/conventional-commits/SKILL.md`

```markdown
## Anti-Patterns

### NEVER Use Non-Standard Commit Types

**WHY**: Breaks conventional commits parsers and tooling.

**BAD**: `fix: fixed bug` (non-standard type)
**GOOD**: `fix: resolve login issue` (standard type)
```

---

### Phase 3: Pattern Recognition (D7) - Priority: Medium

**Target**: Increase from 7/10 to 9/10 (+2 points)

#### Step 3.1: Expand trigger keywords

Add more triggers to frontmatter.

---

### Phase 4: Practical Usability (D8) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 4.1: Add Quick Commands

Add executable command examples.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh conventional-commits --json
bunx markdownlint-cli2 "skills/conventional-commits/**/*.md"
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D5 Progressive Disclosure | Score >= 14/15 |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| References created | >= 2 files |
| Overall Score | >= 100/120 (B) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Disclosure | S | 30 min |
| Phase 2: Anti-patterns | S | 30 min |
| Phase 3: Triggers | S | 20 min |
| Phase 4: Commands | S | 20 min |
| **Total** | **S** | **1.5 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/conventional-commits/SKILL.md
```

## Notes

- Rating: **7/10** - Good structure with phases and code examples
- Already has Estimated Effort table, Dependencies, Rollback Plan
- Timeline table included
- Minor: Could add Notes section for consistency
