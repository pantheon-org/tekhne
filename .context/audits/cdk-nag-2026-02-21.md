# Skill Evaluation Report: cdk-nag

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/cdk-nag/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 84/120 (70.0%) |
| **Grade** | C |
| **Pattern** | Navigation Hub |
| **Lines** | 147 |
| **References** | 6 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 16 | 20 | Mostly expert content with some redundancy |
| D2: Mindset + Procedures | 10 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 8 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 10 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 13 | 15 | Has references, hub could be smaller |
| D6: Freedom Calibration | 10 | 15 | Constraint balance |
| D7: Pattern Recognition | 6 | 10 | Trigger keywords present |
| D8: Practical Usability | 11 | 15 | Practical examples included |

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
skills/cdk-nag/
├── SKILL.md
└── references/implementation-guide.md
└── references/integration-patterns.md
└── references/rule-evolution.md
└── references/rule-packs.md
└── references/suppression-guide.md
└── references/troubleshooting.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts cdk-nag
bunx markdownlint-cli2 "skills/cdk-nag/SKILL.md"
```

---

## Conclusion

`cdk-nag` scores **84/120 (C)**.
