# Skill Evaluation Report: test-driven-development

**Review Date**: 2026-02-21
**Reviewer**: automated audit
**Skill Location**: `skills/test-driven-development/SKILL.md`

---

## Executive Summary

| Metric | Value |
| --- | --- |
| **Total Score** | 99/120 (82.5%) |
| **Grade** | B |
| **Pattern** | Navigation Hub |
| **Lines** | 337 |
| **References** | 42 files |

---

## Dimension Scores

| Dimension | Score | Max | Assessment |
| --- | ---: | ---: | --- |
| D1: Knowledge Delta | 18 | 20 | High expert signal, minimal redundancy |
| D2: Mindset + Procedures | 12 | 15 | Procedural guidance present |
| D3: Anti-Pattern Quality | 13 | 15 | Anti-pattern coverage |
| D4: Specification Compliance | 10 | 15 | Frontmatter compliant |
| D5: Progressive Disclosure | 10 | 15 | Has references, hub could be smaller |
| D6: Freedom Calibration | 15 | 15 | Constraint balance |
| D7: Pattern Recognition | 6 | 10 | Trigger keywords present |
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
skills/test-driven-development/
├── SKILL.md
└── references/assert-custom-matchers.md
└── references/assert-error-messages.md
└── references/assert-no-assertions-antipattern.md
└── references/assert-snapshot-testing.md
└── references/assert-specific-assertions.md
└── references/cycle-maintain-test-list.md
└── references/cycle-minimal-code-to-pass.md
└── references/cycle-refactor-after-green.md
└── references/cycle-small-increments.md
└── references/cycle-verify-test-fails-first.md
└── references/cycle-write-test-first.md
└── references/data-avoid-mystery-guests.md
└── references/data-builder-pattern.md
└── references/data-minimal-setup.md
└── references/data-unique-identifiers.md
└── references/data-use-factories.md
└── references/design-aaa-pattern.md
└── references/design-avoid-logic-in-tests.md
└── references/design-descriptive-test-names.md
└── references/design-one-assertion-per-test.md
└── references/design-test-behavior-not-implementation.md
└── references/design-test-edge-cases.md
└── references/isolate-deterministic-tests.md
└── references/isolate-mock-external-dependencies.md
└── references/isolate-no-shared-state.md
└── references/isolate-prefer-stubs-over-mocks.md
└── references/isolate-use-dependency-injection.md
└── references/org-file-structure.md
└── references/org-group-by-behavior.md
└── references/org-parameterized-tests.md
└── references/org-setup-teardown.md
└── references/org-test-utilities.md
└── references/perf-avoid-network-calls.md
└── references/perf-avoid-sleep.md
└── references/perf-fast-unit-tests.md
└── references/perf-fix-flaky-tests.md
└── references/perf-parallelize-tests.md
└── references/strat-coverage-targets.md
└── references/strat-e2e-critical-paths.md
└── references/strat-integration-boundaries.md
└── references/strat-mutation-testing.md
└── references/strat-test-pyramid.md
```

---

## Verification

```bash
bun run skills/skill-quality-auditor/scripts/evaluate.ts test-driven-development
bunx markdownlint-cli2 "skills/test-driven-development/SKILL.md"
```

---

## Conclusion

`test-driven-development` scores **99/120 (B)**.
