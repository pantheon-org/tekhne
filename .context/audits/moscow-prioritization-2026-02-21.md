# Skill Evaluation Report: moscow-prioritization

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/moscow-prioritization/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 80/120 (66.6%) |
| **Grade** | D |
| **Pattern** | Monolithic |
| **Lines** | 754 |
| **References** | 0 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 16 | 20 | Mostly expert content with some redundancy |
| D2: Mindset + Procedures | 12 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 9 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 10 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 5 | 15 | Large monolithic file, needs navigation-hub refactor |
| D6: Freedom Calibration | 12 | 15 | Constraint balance |
| D7: Pattern Recognition | 6 | 10 | Trigger keywords present |
| D8: Practical Usability | 10 | 15 | Practical examples included |

---

## Critical Issues

### 1. Progressive disclosure needs improvement

- `skills/moscow-prioritization/SKILL.md` (754 lines)
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
skills/moscow-prioritization/
├── SKILL.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts moscow-prioritization
bunx markdownlint-cli2 "skills/moscow-prioritization/SKILL.md"
```

---

## Conclusion

`moscow-prioritization` scores **80/120 (D)**.
