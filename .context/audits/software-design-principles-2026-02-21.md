# Skill Evaluation Report: software-design-principles

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/software-design-principles/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 97/120 (80.8%) |
| **Grade** | B |
| **Pattern** | Navigation Hub |
| **Lines** | 382 |
| **References** | 44 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 18 | 20 | High expert signal, minimal redundancy |
| D2: Mindset + Procedures | 10 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 10 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 15 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 10 | 15 | Has references, hub could be smaller |
| D6: Freedom Calibration | 12 | 15 | Constraint balance |
| D7: Pattern Recognition | 10 | 10 | Trigger keywords present |
| D8: Practical Usability | 12 | 15 | Practical examples included |

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
skills/software-design-principles/
├── SKILL.md
└── references/adapt-anti-corruption-layer.md
└── references/adapt-controller-thin.md
└── references/adapt-gateway-abstraction.md
└── references/adapt-mapper-translation.md
└── references/adapt-presenter-formats.md
└── references/anti-patterns-and-frameworks.md
└── references/bound-boundary-cost-awareness.md
└── references/bound-defer-decisions.md
└── references/bound-humble-object.md
└── references/bound-main-component.md
└── references/bound-partial-boundaries.md
└── references/bound-service-internal-architecture.md
└── references/comp-common-closure.md
└── references/comp-common-reuse.md
└── references/comp-reuse-release-equivalence.md
└── references/comp-screaming-architecture.md
└── references/comp-stable-dependencies.md
└── references/dep-acyclic-dependencies.md
└── references/dep-data-crossing-boundaries.md
└── references/dep-interface-ownership.md
└── references/dep-inward-only.md
└── references/dep-no-framework-imports.md
└── references/dep-stable-abstractions.md
└── references/detailed-examples.md
└── references/entity-encapsulate-invariants.md
└── references/entity-no-persistence-awareness.md
└── references/entity-pure-business-rules.md
└── references/entity-rich-not-anemic.md
└── references/entity-value-objects.md
└── references/frame-di-container-edge.md
└── references/frame-domain-purity.md
└── references/frame-logging-abstraction.md
└── references/frame-orm-in-infrastructure.md
└── references/frame-web-in-infrastructure.md
└── references/test-boundary-verification.md
└── references/test-layer-isolation.md
└── references/test-testable-design.md
└── references/test-tests-are-architecture.md
└── references/usecase-explicit-dependencies.md
└── references/usecase-input-output-ports.md
└── references/usecase-no-presentation-logic.md
└── references/usecase-orchestrates-not-implements.md
└── references/usecase-single-responsibility.md
└── references/usecase-transaction-boundary.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts software-design-principles
bunx markdownlint-cli2 "skills/software-design-principles/SKILL.md"
```

---

## Conclusion

`software-design-principles` scores **97/120 (B)**.
