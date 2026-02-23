---
plan_date: 2026-02-23
skill_name: commanderjs
source_audit: .context/audits/commanderjs-audit-2026-02-22.md
---

# Remediation Plan: commanderjs

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 90/120 (75%) | 102/120 (85%) |
| **Grade** | C+ | B+ |
| **Priority** | Medium | - |
| **Effort** | Small-Medium (S-M) | - |

**Focus Areas**: Pattern recognition (D7), Progressive disclosure (D5), Anti-pattern quality (D3)

**Verdict**: Targeted improvements recommended. Good baseline with room for enhancement.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Pattern recognition moderate | D7 (7/10) | Medium | Skill may not activate |
| Progressive disclosure moderate | D5 (10/15) | Medium | Maintainability |
| Anti-pattern quality moderate | D3 (10/15) | Medium | Mistakes may be repeated |

## Detailed Remediation Steps

### Phase 1: Pattern Recognition (D7) - Priority: Medium

**Target**: Increase from 7/10 to 9/10 (+2 points)

#### Step 1.1: Expand trigger keywords

**File**: `skills/commanderjs/SKILL.md`

```yaml
---
name: commanderjs
description: |
  Complete Commander.js CLI framework guidance. Use when: building CLI tools, parsing command-line arguments,
  implementing subcommands, handling options/flags, creating interactive CLIs.
  
  Keywords: Commander.js, CLI, command-line, arguments, options, flags, subcommands,
  action handlers, version, help text, TypeScript, program
---
```

---

### Phase 2: Progressive Disclosure (D5) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 2.1: Create references

Current: SKILL.md 328 lines, 0 references. Extract detailed content.

---

### Phase 3: Anti-Pattern Quality (D3) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 3.1: Enhance anti-patterns

Add more specific NEVER statements with BAD/GOOD examples.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh commanderjs --json
bunx markdownlint-cli2 "skills/commanderjs/**/*.md"
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D7 Pattern Recognition | Score >= 9/10 |
| D5 Progressive Disclosure | Score >= 13/15 |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| References created | >= 2 files |
| Overall Score | >= 102/120 (B+) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Triggers | S | 20 min |
| Phase 2: Disclosure | S | 30 min |
| Phase 3: Anti-patterns | S | 30 min |
| **Total** | **S** | **1.5 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/commanderjs/SKILL.md
```

## Notes

- Rating: **7/10** - Already follows Format B template well
- Has detailed code examples
- Has Estimated Effort table, Dependencies, Rollback Plan
- Minor: Could add Notes section
