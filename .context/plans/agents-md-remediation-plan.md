---
plan_date: 2026-02-23
skill_name: agents-md
source_audit: .context/audits/agents-md-audit-2026-02-22.md
---

# Remediation Plan: agents-md

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 97/120 (80%) | 108/120 (90%) |
| **Grade** | B | A |
| **Priority** | Low | - |
| **Effort** | Small (S) | - |

**Focus Areas**: Procedural clarity (D2), Progressive disclosure (D5), Anti-pattern quality (D3)

**Verdict**: Solid baseline with targeted improvements. Already high-scoring skill.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Procedural clarity gaps | D2 (10/15) | Medium | Workflow not fully deterministic |
| Progressive disclosure needed | D5 (10/15) | Medium | SKILL.md could be condensed |
| Anti-pattern quality | D3 (11/15) | Medium | Could be more specific |

## Detailed Remediation Steps

### Phase 1: Procedural Clarity (D2) - Priority: Medium

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 1.1: Add deterministic workflow

**File**: `skills/agents-md/SKILL.md`

Add explicit workflow with entry/exit conditions.

---

### Phase 2: Progressive Disclosure (D5) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 2.1: Condense SKILL.md

Keep SKILL.md as navigation hub under 100 lines. Move detailed examples to references.

---

### Phase 3: Anti-Pattern Quality (D3) - Priority: Medium

**Target**: Increase from 11/15 to 14/15 (+3 points)

#### Step 3.1: Enhance anti-patterns

Add NEVER statements with WHY and BAD/GOOD examples.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh agents-md --json
bunx markdownlint-cli2 "skills/agents-md/**/*.md"
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D2 Mindset + Procedures | Score >= 13/15 |
| D5 Progressive Disclosure | Score >= 14/15 |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| SKILL.md line count | <= 100 lines |
| Overall Score | >= 108/120 (A) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Procedures | S | 30 min |
| Phase 2: Disclosure | S | 30 min |
| Phase 3: Anti-patterns | S | 30 min |
| **Total** | **S** | **1.5 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/agents-md/SKILL.md
```

## Notes

- Rating: **8/10** - Already strong, needs minor enhancements
- Good baseline score (B grade)
- Actions are clear and actionable
- Could add more code examples
