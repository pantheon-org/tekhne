# Skill Evaluation Report: commanderjs

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/commanderjs/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 90/120 (75.0%) |
| **Grade** | C+ |
| **Pattern** | Navigation Hub |
| **Lines** | 202 |
| **References** | 6 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 17 | 20 | Mostly expert content with some redundancy |
| D2: Mindset + Procedures | 12 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 10 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 10 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 10 | 15 | Has references, hub could be smaller |
| D6: Freedom Calibration | 12 | 15 | Constraint balance |
| D7: Pattern Recognition | 6 | 10 | Trigger keywords present |
| D8: Practical Usability | 13 | 15 | Practical examples included |

---

## Critical Issues

### 1. Markdown quality-gate issues

- MD040: Fenced code block without language

- **Impact**: Fails lint checks, blocks clean validation.

---

## Top 3 Recommended Improvements

### Priority 1: Fix markdown lint issues

Address MD036 and MD040 violations for clean validation.

---

## Files Inventory

```text
skills/commanderjs/
├── SKILL.md
└── references/actions-handlers.md
└── references/commands-structure.md
└── references/core-basics.md
└── references/options-flags.md
└── references/practices-patterns.md
└── references/typescript-setup.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts commanderjs
bunx markdownlint-cli2 "skills/commanderjs/SKILL.md"
```

---

## Conclusion

`commanderjs` scores **90/120 (C+)**.
