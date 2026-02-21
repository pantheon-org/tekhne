# Skill Evaluation Report: cfn-template-compare

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/cfn-template-compare/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 101/120 (84.1%) |
| **Grade** | B |
| **Pattern** | Navigation Hub |
| **Lines** | 459 |
| **References** | 2 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 18 | 20 | High expert signal, minimal redundancy |
| D2: Mindset + Procedures | 12 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 11 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 13 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 10 | 15 | Has references, hub could be smaller |
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
skills/cfn-template-compare/
├── SKILL.md
└── references/compare-cfn-templates.md
└── references/markdown-report-example.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts cfn-template-compare
bunx markdownlint-cli2 "skills/cfn-template-compare/SKILL.md"
```

---

## Conclusion

`cfn-template-compare` scores **101/120 (B)**.
