# Skill Evaluation Report: ui-debug-workflow

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/ui-debug-workflow/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 85/120 (70.8%) |
| **Grade** | C |
| **Pattern** | Monolithic |
| **Lines** | 308 |
| **References** | 0 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 15 | 20 | Mostly expert content with some redundancy |
| D2: Mindset + Procedures | 12 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 10 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 10 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 7 | 15 | Moderate size, consider references/ partitioning |
| D6: Freedom Calibration | 10 | 15 | Constraint balance |
| D7: Pattern Recognition | 6 | 10 | Trigger keywords present |
| D8: Practical Usability | 15 | 15 | Practical examples included |

---

## Critical Issues

### 1. Progressive disclosure needs improvement

- `skills/ui-debug-workflow/SKILL.md` (308 lines)
- **Impact**: Large single file reduces discoverability and increases maintenance cost.

### 2. Markdown quality-gate issues

- MD036: Emphasis used as heading
- MD040: Fenced code block without language

- **Impact**: Fails lint checks, blocks clean validation.

---

## Top 3 Recommended Improvements

### Priority 1: Split into navigation hub + references

Move detailed sections to `references/` and keep SKILL.md as concise navigation.

### Priority 2: Fix markdown lint issues

Address MD036 and MD040 violations for clean validation.

---

## Files Inventory

```text
skills/ui-debug-workflow/
├── SKILL.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts ui-debug-workflow
bunx markdownlint-cli2 "skills/ui-debug-workflow/SKILL.md"
```

---

## Conclusion

`ui-debug-workflow` scores **85/120 (C)**.
