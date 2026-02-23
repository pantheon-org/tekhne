---
plan_date: 2026-02-23
skill_name: bdd-testing
source_audit: .context/audits/bdd-testing-audit-2026-02-22.md
---

# Remediation Plan: bdd-testing

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 84/120 (70%) |
| **Current Grade** | C |
| **Target Score** | 102/120 (B+) |
| **Priority** | High |
| **Estimated Effort** | 4-5 hours |

## Critical Issues to Address

| Issue | Severity | Dimension |
| --- | --- | --- |
| Anti-pattern quality weak | **High** | D3 (8/15) |
| Practical usability gaps | **High** | D8 (8/15) |
| Pattern recognition lacking | Medium | D7 (6/10) |
| Freedom calibration | Medium | D6 (10/15) |

## Detailed Remediation Steps

### D3: Anti-Pattern Quality (Current: 8/15, Target: 13/15)

**Problem**: Anti-patterns are weak; missing WHY and consequences.

**Actions**:
1. Add concrete NEVER statements:
   - NEVER write implementation details in Gherkin
   - NEVER skip the Three Amigos session
   - NEVER use technical jargon in feature files
2. Add BAD/GOOD examples for each anti-pattern:
   - BAD: "When I click the submit button" (implementation)
   - GOOD: "When I submit the form" (behavior)
3. Explain WHY and consequences for each

**Files to Edit**:
- `skills/bdd-testing/SKILL.md` - Add comprehensive anti-patterns section
- `skills/bdd-testing/references/anti-patterns.md` - Create if needed

### D8: Practical Usability (Current: 8/15, Target: 13/15)

**Problem**: Missing executable commands and examples.

**Actions**:
1. Add copy/paste commands for:
   - Running Cucumber tests
   - Generating step definitions
   - Validating feature files
2. Add expected outputs for each command
3. Add completion verification checklist

**Files to Edit**:
- `skills/bdd-testing/SKILL.md` - Add Quick Commands section
- `skills/bdd-testing/references/commands.md` - Create if needed

### D7: Pattern Recognition (Current: 6/10, Target: 9/10)

**Problem**: Trigger keywords insufficient.

**Actions**:
1. Expand frontmatter description with keywords:
   - "behavior-driven development"
   - "Gherkin syntax"
   - "Cucumber tests"
   - "Given When Then"
   - "feature files"
   - "step definitions"
2. Add "use when" phrases

**Files to Edit**:
- `skills/bdd-testing/SKILL.md` - Update frontmatter

### D6: Freedom Calibration (Current: 10/15, Target: 13/15)

**Problem**: Over-constrained or under-specified in places.

**Actions**:
1. Keep hard constraints for:
   - Gherkin syntax compliance
   - Step definition patterns
2. Make flexible where safe:
   - Tool choice (Cucumber vs others)
   - Feature file organization
3. Add fallback paths for missing context

**Files to Edit**:
- `skills/bdd-testing/SKILL.md` - Review constraint language

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh bdd-testing --json
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
skills/skill-quality-auditor/scripts/validate-review-format.sh .context/audits/bdd-testing-audit-2026-02-22.md
```

## Success Criteria

- [ ] Score >= 102/120 (B+)
- [ ] D3 (Anti-Pattern Quality) >= 13/15
- [ ] D8 (Practical Usability) >= 13/15
- [ ] D7 (Pattern Recognition) >= 9/10
- [ ] D6 (Freedom Calibration) >= 13/15
- [ ] All anti-patterns have BAD/GOOD examples
- [ ] All commands are copy/paste ready
