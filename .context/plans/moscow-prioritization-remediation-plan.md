---
plan_date: 2026-02-23
skill_name: moscow-prioritization
source_audit: .context/audits/moscow-prioritization-audit-2026-02-22.md
status: completed
completed_date: 2026-02-23
---

# Remediation Plan: moscow-prioritization

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 88/120 (73%) | 100/120 (83%) |
| **Grade** | C | B |
| **Priority** | High | - |
| **Effort** | Small (S) | - |

**Focus Areas**: Progressive disclosure (D5), Anti-pattern quality (D3), Practical usability (D8)

**Verdict**: Priority improvements required. Multiple dimensions need enhancement.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Progressive disclosure moderate | D5 (10/15) | High | Maintainability |
| Anti-pattern quality moderate | D3 (10/15) | Medium | Mistakes may be repeated |
| Practical usability gaps | D8 (10/15) | Medium | Commands could be clearer |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - Priority: High

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 1.1: Create references

Extract detailed content to references/.

---

### Phase 2: Anti-Pattern Quality (D3) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 2.1: Enhance anti-patterns

Add more specific NEVER statements with BAD/GOOD examples.

---

### Phase 3: Practical Usability (D8) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 3.1: Add Quick Commands

Add executable command examples.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh moscow-prioritization --json
bunx markdownlint-cli2 "skills/moscow-prioritization/**/*.md"
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D5 Progressive Disclosure | Score >= 14/15 |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| D8 Practical Usability | Score >= 13/15 |
| References created | >= 2 files |
| Overall Score | >= 100/120 (B) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Disclosure | S | 20 min |
| Phase 2: Anti-patterns | S | 20 min |
| Phase 3: Commands | S | 15 min |
| **Total** | **S** | **1 hour** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/moscow-prioritization/SKILL.md
```

## Notes

- Rating: **7/10** - Already follows Format B template well
- Has detailed code examples
- Has Estimated Effort table, Dependencies, Rollback Plan
- Minor: Could add Notes section for consistency
