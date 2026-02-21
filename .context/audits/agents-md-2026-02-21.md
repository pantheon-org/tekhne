# Skill Evaluation Report: agents-md

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/agents-md/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 97/120 (80.8%) |
| **Grade** | B |
| **Pattern** | Navigation Hub |
| **Lines** | 234 |
| **References** | 7 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 19 | 20 | High expert signal, minimal redundancy |
| D2: Mindset + Procedures | 10 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 11 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 15 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 10 | 15 | Has references, hub could be smaller |
| D6: Freedom Calibration | 12 | 15 | Constraint balance |
| D7: Pattern Recognition | 10 | 10 | Trigger keywords present |
| D8: Practical Usability | 10 | 15 | Practical examples included |

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
skills/agents-md/
├── SKILL.md
└── references/anti-patterns.md
└── references/api-template.md
└── references/database-template.md
└── references/design-system-template.md
└── references/discovery-commands.md
└── references/testing-template.md
└── references/troubleshooting.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts agents-md
bunx markdownlint-cli2 "skills/agents-md/SKILL.md"
```

---

## Conclusion

`agents-md` scores **97/120 (B)**.
