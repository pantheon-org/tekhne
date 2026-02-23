---
plan_date: 2026-02-23
skill_name: test-driven-development
source_audit: .context/audits/test-driven-development-audit-2026-02-22.md
status: completed
completed_date: 2026-02-23
---

# Remediation Plan: test-driven-development

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 99/120 (82%) | 108/120 (90%) |
| **Grade** | B | A |
| **Priority** | Medium | - |
| **Effort** | Medium (M) | - |

**Focus Areas**: Pattern recognition (D7), Specification compliance (D4), Progressive disclosure (D5)

**Verdict**: Targeted improvements recommended. Already strong baseline (B grade) - focus on specificity and structure.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Weak trigger keywords | D7 (6/10) | High | Skill may not activate when needed |
| Moderate spec compliance | D4 (10/15) | Medium | Routing confusion |
| Moderate progressive disclosure | D5 (10/15) | Medium | Maintainability concerns |

## Detailed Remediation Steps

### Phase 1: Pattern Recognition (D7) - Priority: High

**Target**: Increase from 6/10 to 9/10 (+3 points)

#### Step 1.1: Expand frontmatter description

**File**: `skills/test-driven-development/SKILL.md`

```yaml
---
name: test-driven-development
description: |
  Master Test-Driven Development (TDD) with comprehensive guidance on the Red-Green-Refactor cycle.
  Use when: writing new functions, adding features, fixing bugs, refactoring code, or making API changes.
  
  Keywords: TDD, red green refactor, test-driven, unit tests, test isolation, mocking,
  failing test first, test design, AAA pattern, test coverage
---
```

#### Step 1.2: Add "Use When" section

```markdown
## Use When

- "Write tests first for this feature"
- "How do I do TDD?"
- "Red-green-refactor workflow"
- "Test-driven development approach"
- "Write a failing test"
- NOT for: End-to-end testing, performance testing
```

---

### Phase 2: Specification Compliance (D4) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 2.1: Tighten frontmatter

**File**: `skills/test-driven-development/SKILL.md`

Ensure all required fields present and precise.

#### Step 2.2: Add explicit scope boundaries

```markdown
## Scope

### In Scope
- Unit test design and patterns
- Integration test strategies  
- Mock/stub creation
- Red-Green-Refactor workflow
- Test naming conventions

### Out of Scope
- End-to-end testing (Cypress, Playwright)
- Performance testing
- Security testing
- Test infrastructure setup
```

---

### Phase 3: Progressive Disclosure (D5) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 3.1: Create references structure

Current: SKILL.md 336 lines, references/ 42 files (already good)

#### Step 3.2: Update SKILL.md as hub

```markdown
## Quick Reference

| Topic | Reference |
|-------|-----------|
| Red-Green-Refactor | [references/red-green-refactor.md](references/red-green-refactor.md) |
| Test Naming | [references/naming-conventions.md](references/naming-conventions.md) |
| Mocking | [references/mocking.md](references/mocking.md) |
| Anti-Patterns | [references/anti-patterns.md](references/anti-patterns.md) |
```

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh test-driven-development --json
bunx markdownlint-cli2 "skills/test-driven-development/**/*.md"
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D7 Pattern Recognition | Score >= 9/10 |
| D4 Specification Compliance | Score >= 13/15 |
| D5 Progressive Disclosure | Score >= 13/15 |
| Overall Score | >= 108/120 (A) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Triggers | S | 30 min |
| Phase 2: Spec Compliance | S | 20 min |
| Phase 3: Disclosure | S | 30 min |
| **Total** | **M** | **1.5 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/test-driven-development/SKILL.md
```

## Notes

- Rating: **7/10** - Good plan structure with detailed remediation steps
- Already has many references (42 files) - good progressive disclosure baseline
- Code examples included in phases
- Timeline table included
- Minor: Could add more BAD/GOOD examples in anti-patterns
