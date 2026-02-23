---
plan_date: 2026-02-23
skill_name: github-copilot-models
source_audit: .context/audits/github-copilot-models-audit-2026-02-22.md
---

# Remediation Plan: github-copilot-models

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 91/120 (75%) | 102/120 (85%) |
| **Grade** | C+ | B+ |
| **Priority** | Medium | - |
| **Effort** | Small (S) | - |

**Focus Areas**: Progressive disclosure (D5), Anti-pattern quality (D3), Pattern recognition (D7)

**Verdict**: Targeted improvements recommended. Good baseline with room for enhancement.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Progressive disclosure moderate | D5 (10/15) | Medium | Maintainability |
| Anti-pattern quality moderate | D3 (10/15) | Medium | Mistakes may be repeated |
| Pattern recognition moderate | D7 (7/10) | Medium | Skill may not activate |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 1.1: Create references

Extract detailed content to references/.

---

### Phase 2: Anti-Pattern Quality (D3) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 2.1: Enhance anti-patterns

Add more specific NEVER statements with BAD/GOOD examples.

---

### Phase 3: Pattern Recognition (D7) - Priority: Medium

**Target**: Increase from 7/10 to 9/10 (+2 points)

#### Step 3.1: Expand triggers

Add more trigger phrases to frontmatter.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh github-copilot-models --json
bunx markdownlint-cli2 "skills/github-copilot-models/**/*.md"
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D5 Progressive Disclosure | Score >= 14/15 |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| References created | >= 2 files |
| Overall Score | >= 102/120 (B+) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Disclosure | S | 20 min |
| Phase 2: Anti-patterns | S | 20 min |
| Phase 3: Triggers | S | 15 min |
| **Total** | **S** | **1 hour** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/github-copilot-models/SKILL.md
```

## Notes

- Rating: **7/10** - Already follows Format B template well
- Has detailed code examples
- Has Estimated Effort table, Dependencies, Rollback Plan
- Minor: Could add Notes section for consistency
