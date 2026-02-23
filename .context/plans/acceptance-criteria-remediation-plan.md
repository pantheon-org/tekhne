---
plan_date: 2026-02-23
skill_name: acceptance-criteria
source_audit: .context/audits/acceptance-criteria-audit-2026-02-22.md
---

# Remediation Plan: acceptance-criteria

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 90/120 (75%) |
| **Current Grade** | C+ |
| **Target Score** | 102/120 (B+) |
| **Priority** | Medium |
| **Estimated Effort** | 3-4 hours |

## Critical Issues to Address

| Issue | Severity | Dimension |
| --- | --- | --- |
| Pattern Recognition weak | Medium | D7 (6/10) |
| Anti-pattern quality lacking | Medium | D3 (11/15) |
| Practical usability gaps | Medium | D8 (10/15) |
| Progressive disclosure needed | Medium | D5 (11/15) |

## Detailed Remediation Steps

### D7: Pattern Recognition (Current: 6/10, Target: 9/10)

**Problem**: Trigger keywords insufficient for skill discovery.

**Actions**:
1. Expand frontmatter description with high-signal keywords
2. Add explicit "use when" phrases:
   - "write acceptance criteria"
   - "define done for user story"
   - "create testable requirements"
   - "INVEST criteria"
3. Add concrete trigger examples in description

**Files to Edit**:
- `skills/acceptance-criteria/SKILL.md` - Update frontmatter description

### D3: Anti-Pattern Quality (Current: 11/15, Target: 14/15)

**Problem**: Anti-patterns lack concrete BAD/GOOD examples.

**Actions**:
1. Add NEVER statements with WHY and consequences
2. Add BAD/GOOD examples for:
   - Vague criteria (e.g., "fast performance" vs "page loads in <2s")
   - Non-testable criteria
   - Missing acceptance criteria entirely
3. Tie each anti-pattern to repository-specific risks

**Files to Edit**:
- `skills/acceptance-criteria/SKILL.md` - Add anti-patterns section
- `skills/acceptance-criteria/references/anti-patterns.md` - Create if needed

### D8: Practical Usability (Current: 10/15, Target: 13/15)

**Problem**: Missing executable commands and deterministic outputs.

**Actions**:
1. Add copy/paste commands for validation
2. Add expected output examples
3. Add completion checklist

**Files to Edit**:
- `skills/acceptance-criteria/SKILL.md` - Add Quick Commands section

### D5: Progressive Disclosure (Current: 11/15, Target: 14/15)

**Problem**: SKILL.md has 169 lines; deep content should move to references.

**Actions**:
1. Keep SKILL.md as navigation hub (<100 lines)
2. Move detailed examples to `references/examples.md`
3. Add concise section summaries with links

**Files to Edit**:
- `skills/acceptance-criteria/SKILL.md` - Condense
- `skills/acceptance-criteria/references/examples.md` - Create/expand

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh acceptance-criteria --json
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
skills/skill-quality-auditor/scripts/validate-review-format.sh .context/audits/acceptance-criteria-audit-2026-02-22.md
```

## Success Criteria

- [ ] Score >= 102/120 (B+)
- [ ] D7 (Pattern Recognition) >= 9/10
- [ ] D3 (Anti-Pattern Quality) >= 14/15
- [ ] D8 (Practical Usability) >= 13/15
- [ ] SKILL.md < 100 lines
- [ ] All anti-patterns have BAD/GOOD examples
