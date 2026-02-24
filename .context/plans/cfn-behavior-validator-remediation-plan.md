---
plan_date: 2026-02-23
skill_name: cfn-behavior-validator
source_audit: .context/audits/cfn-behavior-validator-audit-2026-02-22.md
status: completed
completed_date: 2026-02-23
---

# Remediation Plan: cfn-behavior-validator

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 80/120 (66%) | 100/120 (83%) |
| **Grade** | D | B |
| **Priority** | **Critical** | - |
| **Effort** | Medium (M) | - |

**Focus Areas**: Progressive disclosure (D5), Anti-pattern quality (D3), Practical usability (D8)

**Verdict**: Major rewrite recommended. Critical gaps in multiple dimensions.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Very weak progressive disclosure | D5 (5/15) | **Critical** | Maintainability crisis |
| Anti-pattern quality weak | D3 (8/15) | High | Mistakes repeated |
| Practical usability gaps | D8 (8/15) | High | Commands missing |
| No references | D5 (note) | **Critical** | Unmaintainable |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - Priority: Critical

**Target**: Increase from 5/15 to 13/15 (+8 points)

#### Step 1.1: Create references directory

**File**: `skills/cfn-behavior-validator/references/`

Extract detailed content to reference files.

---

### Phase 2: Anti-Pattern Quality (D3) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 2.1: Add explicit anti-patterns

**File**: `skills/cfn-behavior-validator/SKILL.md`

Add NEVER statements with WHY and BAD/GOOD examples.

---

### Phase 3: Practical Usability (D8) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 3.1: Add Quick Commands

Add executable command examples.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh cfn-behavior-validator --json
bunx markdownlint-cli2 "skills/cfn-behavior-validator/**/*.md"
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D5 Progressive Disclosure | Score >= 13/15 |
| D3 Anti-Pattern Quality | Score >= 13/15 |
| D8 Practical Usability | Score >= 13/15 |
| References created | >= 2 files |
| Overall Score | >= 100/120 (B) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Disclosure | M | 2 hours |
| Phase 2: Anti-patterns | S | 45 min |
| Phase 3: Commands | S | 30 min |
| **Total** | **M** | **3 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/cfn-behavior-validator/
```

## Notes

- Rating: **6/10** - Plan addresses critical issues but needs more detail
- Good structure with phases
- Needs more specific code examples in remediation steps
- Identifies critical missing references issue
