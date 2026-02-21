# Skill Evaluation Report: nx-executors

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/nx-executors/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 103/120 (85.8%) |
| **Grade** | B+ |
| **Pattern** | Monolithic |
| **Lines** | 206 |
| **References** | 0 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 17 | 20 | Mostly expert content with some redundancy |
| D2: Mindset + Procedures | 12 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 12 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 15 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 10 | 15 | Structure assessment |
| D6: Freedom Calibration | 12 | 15 | Constraint balance |
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
skills/nx-executors/
├── SKILL.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts nx-executors
bunx markdownlint-cli2 "skills/nx-executors/SKILL.md"
```

---

## Conclusion

`nx-executors` scores **103/120 (B+)**.
