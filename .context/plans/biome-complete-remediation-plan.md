---
plan_date: 2026-02-23
skill_name: biome-complete
source_audit: .context/audits/biome-complete-audit-2026-02-22.md
---

# Remediation Plan: biome-complete

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 79/120 (65%) |
| **Current Grade** | D |
| **Target Score** | 96/120 (B) |
| **Priority** | **Critical** |
| **Estimated Effort** | 5-6 hours |

## Critical Issues to Address

| Issue | Severity | Dimension |
| --- | --- | --- |
| Anti-pattern quality very weak | **Critical** | D3 (8/15) |
| Practical usability gaps | **Critical** | D8 (8/15) |
| Pattern recognition lacking | **High** | D7 (6/10) |
| Knowledge delta moderate | High | D1 (15/20) |
| No references | **Critical** | D5 note |

## Detailed Remediation Steps

### D3: Anti-Pattern Quality (Current: 8/15, Target: 13/15)

**Problem**: Anti-patterns are very weak; no concrete examples.

**Actions**:
1. Add comprehensive NEVER statements:
   - NEVER mix Biome with ESLint/Prettier on same files
   - NEVER skip biome.json configuration
   - NEVER ignore migration warnings
2. Add BAD/GOOD examples for:
   - Configuration mistakes
   - Rule override anti-patterns
   - Migration failures
3. Explain WHY and consequences for each

**Files to Edit**:
- `skills/biome-complete/SKILL.md` - Add anti-patterns section
- `skills/biome-complete/references/anti-patterns.md` - Create

### D8: Practical Usability (Current: 8/15, Target: 13/15)

**Problem**: Missing executable commands.

**Actions**:
1. Add copy/paste commands for:
   - Installation: `bunx @biomejs/biome init`
   - Linting: `bunx @biomejs/biome check .`
   - Formatting: `bunx @biomejs/biome format . --write`
   - Migration from ESLint
2. Add expected outputs
3. Add verification steps

**Files to Edit**:
- `skills/biome-complete/SKILL.md` - Add Quick Commands section
- `skills/biome-complete/references/commands.md` - Create

### D7: Pattern Recognition (Current: 6/10, Target: 9/10)

**Problem**: Trigger keywords insufficient.

**Actions**:
1. Expand frontmatter description with:
   - "biome.json configuration"
   - "migrate from ESLint"
   - "migrate from Prettier"
   - "fast linting"
   - "Rust-based linter"
2. Add "use when" phrases

**Files to Edit**:
- `skills/biome-complete/SKILL.md` - Update frontmatter

### D1: Knowledge Delta (Current: 15/20, Target: 18/20)

**Problem**: Content may be generic; needs project-specific rules.

**Actions**:
1. Replace generic basics with repository-contextual rules
2. Add Biome-specific gotchas and edge cases
3. Add production caveats

**Files to Edit**:
- `skills/biome-complete/SKILL.md` - Enhance knowledge content

### D5: Progressive Disclosure (Current: 12/15, Target: 14/15)

**Problem**: File has 59 lines but 0 references; needs structure.

**Actions**:
1. Create references directory
2. Move detailed configuration to `references/configuration.md`
3. Move migration guides to `references/migration.md`
4. Keep SKILL.md as navigation hub

**Files to Edit**:
- `skills/biome-complete/SKILL.md` - Condense to hub
- `skills/biome-complete/references/configuration.md` - Create
- `skills/biome-complete/references/migration.md` - Create

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh biome-complete --json
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
skills/skill-quality-auditor/scripts/validate-review-format.sh .context/audits/biome-complete-audit-2026-02-22.md
```

## Success Criteria

- [ ] Score >= 96/120 (B)
- [ ] D3 (Anti-Pattern Quality) >= 13/15
- [ ] D8 (Practical Usability) >= 13/15
- [ ] D7 (Pattern Recognition) >= 9/10
- [ ] D1 (Knowledge Delta) >= 18/20
- [ ] At least 2 reference files created
- [ ] All anti-patterns have BAD/GOOD examples
- [ ] All commands are copy/paste ready
