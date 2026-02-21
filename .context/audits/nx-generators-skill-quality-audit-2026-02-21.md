# Skill Evaluation Report: nx-generators

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/nx-generators/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 96/120 (80.0%) |
| **Grade** | B |
| **Pattern** | Monolithic |
| **Lines** | 289 |
| **References** | 0 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 16 | 20 | Mostly expert content with some redundancy |
| D2: Mindset + Procedures | 12 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 8 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 15 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 10 | 15 | Structure assessment |
| D6: Freedom Calibration | 10 | 15 | Constraint balance |
| D7: Pattern Recognition | 10 | 10 | Trigger keywords present |
| D8: Practical Usability | 15 | 15 | Practical examples included |

---

## Critical Issues

### 1. Markdown quality-gate issues

- MD036: Emphasis used as heading
- MD040: Fenced code block without language

- **Impact**: Fails lint checks, blocks clean validation.

---

## Top 3 Recommended Improvements

### Priority 1: Fix markdown lint issues

Address MD036 and MD040 violations for clean validation.

---

## Files Inventory

```text
skills/nx-generators/
├── SKILL.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts nx-generators
bunx markdownlint-cli2 "skills/nx-generators/SKILL.md"
```

---

## Conclusion

`nx-generators` scores **96/120 (B)**.
