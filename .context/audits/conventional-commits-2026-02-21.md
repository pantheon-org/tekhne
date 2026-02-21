# Skill Evaluation Report: conventional-commits

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/conventional-commits/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 87/120 (72.5%) |
| **Grade** | C |
| **Pattern** | Monolithic |
| **Lines** | 637 |
| **References** | 0 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 16 | 20 | Mostly expert content with some redundancy |
| D2: Mindset + Procedures | 12 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 8 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 13 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 5 | 15 | Large monolithic file, needs navigation-hub refactor |
| D6: Freedom Calibration | 12 | 15 | Constraint balance |
| D7: Pattern Recognition | 8 | 10 | Trigger keywords present |
| D8: Practical Usability | 13 | 15 | Practical examples included |

---

## Critical Issues

### 1. Progressive disclosure needs improvement

- `skills/conventional-commits/SKILL.md` (637 lines)
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
skills/conventional-commits/
├── SKILL.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts conventional-commits
bunx markdownlint-cli2 "skills/conventional-commits/SKILL.md"
```

---

## Conclusion

`conventional-commits` scores **87/120 (C)**.
