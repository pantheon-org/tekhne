# Skill Evaluation Report: typescript-advanced

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/typescript-advanced/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 87/120 (72.5%) |
| **Grade** | C |
| **Pattern** | Navigation Hub |
| **Lines** | 88 |
| **References** | 48 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 16 | 20 | Mostly expert content with some redundancy |
| D2: Mindset + Procedures | 12 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 8 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 10 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 15 | 15 | Excellent hub + references architecture |
| D6: Freedom Calibration | 12 | 15 | Constraint balance |
| D7: Pattern Recognition | 6 | 10 | Trigger keywords present |
| D8: Practical Usability | 8 | 15 | Practical examples included |

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
skills/typescript-advanced/
├── SKILL.md
└── references/compiler-module-resolution.md
└── references/compiler-performance.md
└── references/compiler-strict-mode.md
└── references/compiler-tsconfig.md
└── references/docs-adr-templates.md
└── references/docs-framework-docs.md
└── references/docs-jsdoc-patterns.md
└── references/docs-typedoc-config.md
└── references/guards-assertion-functions.md
└── references/guards-basic.md
└── references/guards-branded-types.md
└── references/guards-discriminated-unions.md
└── references/guards-exhaustiveness.md
└── references/guards-generic.md
└── references/guards-inference-infer.md
└── references/guards-inference-return.md
└── references/patterns-advanced-generics.md
└── references/patterns-api-client.md
└── references/patterns-branded-types.md
└── references/patterns-builder.md
└── references/patterns-deep-readonly.md
└── references/patterns-dependency-injection.md
└── references/patterns-event-emitter.md
└── references/patterns-form-validation.md
└── references/patterns-plugin-system.md
└── references/patterns-recursive-types.md
└── references/patterns-state-machine.md
└── references/patterns-type-safe-module.md
└── references/practices-illegal-states.md
└── references/practices-module-patterns.md
└── references/practices-runtime-validation.md
└── references/practices-type-first.md
└── references/types-conditional.md
└── references/types-generics.md
└── references/types-index-signatures.md
└── references/types-mapped.md
└── references/types-narrowing.md
└── references/types-template-literals.md
└── references/types-type-assertions.md
└── references/types-unions-intersections.md
└── references/utilities-custom-mapped-types.md
└── references/utilities-extract-exclude.md
└── references/utilities-key-remapping.md
└── references/utilities-nonnullable-awaited.md
└── references/utilities-partial-required.md
└── references/utilities-pick-omit.md
└── references/utilities-readonly-record.md
└── references/utilities-returntype-parameters.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts typescript-advanced
bunx markdownlint-cli2 "skills/typescript-advanced/SKILL.md"
```

---

## Conclusion

`typescript-advanced` scores **87/120 (C)**.
