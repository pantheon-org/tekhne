---
plan_date: 2026-02-23
skill_name: biome-complete
source_audit: .context/audits/biome-complete-audit-2026-02-22.md
---

# Remediation Plan: biome-complete

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 79/120 (65%) | 96/120 (80%) |
| **Grade** | D | B |
| **Priority** | **Critical** | - |
| **Effort** | Medium (M) | - |

**Focus Areas**: Anti-pattern quality (D3), Practical usability (D8), Pattern recognition (D7)

**Verdict**: Priority improvements required. Critical gaps in multiple dimensions.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Anti-pattern quality very weak | D3 (8/15) | **Critical** | Common mistakes repeated |
| Practical usability gaps | D8 (8/15) | **Critical** | Commands missing |
| Pattern recognition lacking | D7 (6/10) | **High** | Skill may not activate |
| Knowledge delta moderate | D1 (15/20) | High | Generic guidance |
| No references | D5 note | **Critical** | Maintainability |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - Priority: Critical

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add comprehensive NEVER statements

**File**: `skills/biome-complete/SKILL.md`

```markdown
## Anti-Patterns

### NEVER Mix Biome with ESLint/Prettier on Same Files

**WHY**: Causes rule conflicts and duplicate linting.

**BAD**: Running both biome and eslint on same files.
**GOOD**: Choose one tool per file type.

### NEVER Skip biome.json Configuration

**WHY**: Default config may not match project needs.

### NEVER Ignore Migration Warnings

**WHY**: Migration warnings indicate breaking changes.
```

---

### Phase 2: Practical Usability (D8) - Priority: Critical

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 2.1: Add copy/paste commands

**File**: `skills/biome-complete/SKILL.md`

````markdown
## Quick Commands

### Initialize Biome
```bash
bunx @biomejs/biome init
```

### Lint Files
```bash
bunx @biomejs/biome check .
```

### Format Files
```bash
bunx @biomejs/biome format . --write
```
````

---

### Phase 3: Pattern Recognition (D7) - Priority: High

**Target**: Increase from 6/10 to 9/10 (+3 points)

#### Step 3.1: Expand frontmatter

**File**: `skills/biome-complete/SKILL.md`

```yaml
---
name: biome-complete
description: |
  Complete Biome toolchain guidance. Use when: configuring biome.json, applying linting rules,
  formatting code, migrating from ESLint/Prettier.
  
  Keywords: Biome, biome.json, linting, formatting, ESLint, Prettier, migration
---
```

---

### Phase 4: Progressive Disclosure (D5) - Priority: Critical

**Target**: Increase from 12/15 to 14/15 (+2 points)

#### Step 4.1: Create references directory

Create `skills/biome-complete/references/` with configuration and migration guides.

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh biome-complete --json
bunx markdownlint-cli2 "skills/biome-complete/**/*.md"
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D3 Anti-Pattern Quality | Score >= 13/15 |
| D8 Practical Usability | Score >= 13/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| References created | >= 2 files |
| Overall Score | >= 96/120 (B) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Anti-patterns | M | 1 hour |
| Phase 2: Commands | S | 30 min |
| Phase 3: Triggers | S | 20 min |
| Phase 4: Disclosure | S | 30 min |
| **Total** | **M** | **2.5 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/biome-complete/
```

## Notes

- Rating: **7/10** - Good structure with critical issues identified
- Code examples in anti-patterns
- Timeline table included
- Addresses critical missing references issue
