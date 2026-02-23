---
plan_date: 2026-02-23
skill_name: agents-md
source_audit: .context/audits/agents-md-audit-2026-02-22.md
---

# Remediation Plan: agents-md

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 97/120 (80%) |
| **Current Grade** | B |
| **Target Score** | 108/120 (A) |
| **Priority** | Low |
| **Estimated Effort** | 2-3 hours |

## Critical Issues to Address

| Issue | Severity | Dimension |
| --- | --- | --- |
| Procedural clarity gaps | Medium | D2 (10/15) |
| Progressive disclosure needed | Medium | D5 (10/15) |
| Anti-pattern quality | Medium | D3 (11/15) |
| Practical usability | Medium | D8 (10/15) |

## Detailed Remediation Steps

### D2: Mindset + Procedures (Current: 10/15, Target: 13/15)

**Problem**: Workflow not deterministic; missing decision points.

**Actions**:
1. Rewrite workflow as ordered steps with explicit outcomes
2. Add "when to use / when not to use" section
3. Add guardrails for ambiguous inputs (e.g., "should I add AGENTS.md?")

**Files to Edit**:
- `skills/agents-md/SKILL.md` - Restructure workflow section

### D5: Progressive Disclosure (Current: 10/15, Target: 14/15)

**Problem**: SKILL.md has 234 lines; only 7 references; content should be extracted.

**Actions**:
1. Keep SKILL.md as navigation hub (<100 lines)
2. Move detailed examples to existing references
3. Add concise summaries with links

**Files to Edit**:
- `skills/agents-md/SKILL.md` - Condense
- `skills/agents-md/references/*.md` - Expand with moved content

### D3: Anti-Pattern Quality (Current: 11/15, Target: 14/15)

**Problem**: Anti-patterns need more specificity.

**Actions**:
1. Add NEVER statements with WHY
2. Add BAD/GOOD examples for:
   - Over-documenting simple projects
   - Missing project context
   - Outdated AGENTS.md
3. Tie to repository-specific risks

**Files to Edit**:
- `skills/agents-md/SKILL.md` - Enhance anti-patterns section

### D8: Practical Usability (Current: 10/15, Target: 13/15)

**Problem**: Missing executable commands.

**Actions**:
1. Add copy/paste commands for AGENTS.md creation
2. Add validation commands
3. Add expected outputs

**Files to Edit**:
- `skills/agents-md/SKILL.md` - Add Quick Commands section

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh agents-md --json
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

- [ ] Score >= 108/120 (A)
- [ ] D2 (Mindset + Procedures) >= 13/15
- [ ] D5 (Progressive Disclosure) >= 14/15
- [ ] D3 (Anti-Pattern Quality) >= 14/15
- [ ] SKILL.md < 100 lines
- [ ] All procedures have explicit entry/exit conditions
