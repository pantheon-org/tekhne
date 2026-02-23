---
name: software-design-principles
description: Apply software design principles across architecture and implementation using deterministic decision workflows, SOLID checks, structural patterns, and anti-pattern detection; use when reviewing designs, refactoring modules, or resolving maintainability and coupling risks.
---

# Software Design Principles

Navigation hub for strategic architecture, tactical design, and code quality decisions.

## When to Use

- Designing a new module, service boundary, or cross-team interface.
- Reviewing code for maintainability, coupling, and extensibility risks.
- Refactoring legacy areas with repeated design problems.
- Evaluating competing approaches with explicit tradeoffs.

## When Not to Use

- One-line bug fixes with no design decision.
- Purely operational tasks (deployment/scripts) without architecture impact.

## Design Decision Workflow

1. Step 1: Identify decision type.
Output: classify as architectural, tactical, or foundational.
2. Step 2: Apply strategic checks for architecture decisions.
Output: boundary/dependency direction and ADR rationale.
3. Step 3: Apply SOLID checks for tactical decisions.
Output: compliance notes and required refactors.
4. Step 4: Select structural pattern only if it reduces complexity.
Output: pattern choice with explicit win condition.
5. Step 5: Validate anti-patterns.
Output: violations listed with BAD/GOOD corrective action.
6. Step 6: Document tradeoffs and limitations.
Output: alternatives considered, decision, and risks.

## SOLID Principles Summary

| Principle | One-liner | Detail |
| --- | --- | --- |
| SRP | One reason to change | [references/usecase-single-responsibility.md](references/usecase-single-responsibility.md) |
| OCP | Extend without modifying stable behavior | [references/comp-common-closure.md](references/comp-common-closure.md) |
| LSP | Subtypes preserve base behavior contracts | [references/detailed-examples.md](references/detailed-examples.md) |
| ISP | Depend only on methods you use | [references/adapt-gateway-abstraction.md](references/adapt-gateway-abstraction.md) |
| DIP | Depend on abstractions, not concrete details | [references/dep-interface-ownership.md](references/dep-interface-ownership.md) |

## Anti-Patterns

### NEVER design for imagined future requirements

WHY: YAGNI violations create complexity without proven value.
Consequence: slower delivery and harder maintenance.
BAD: add abstraction "in case" of possible future DB migration. GOOD: solve current need and refactor when trigger appears.

### NEVER allow circular dependencies

WHY: cycles break modular reasoning and complicate builds.
Consequence: fragile deployment and high change risk.
BAD: module A imports B and B imports A. GOOD: extract shared contract/module and invert dependencies.

### NEVER use god classes/services

WHY: violates single responsibility and creates merge hotspots.
Consequence: low testability and unclear ownership.
BAD: one class handles auth, persistence, notifications, and reporting. GOOD: split into focused collaborators.

### NEVER optimize before measurement

WHY: unmeasured optimization often trades clarity for no gain.
Consequence: needless complexity and hidden bugs.
BAD: add cache because function "might be slow." GOOD: measure baseline, optimize when threshold is exceeded.

### NEVER hard-code environment configuration and secrets

WHY: hard-coded values block secure deployment and portability.
Consequence: security leaks and environment drift.
BAD: inline passwords/URLs in source. GOOD: use environment/config providers.

### NEVER bypass interface contracts when integrating dependencies

WHY: direct concrete coupling reduces substitution and testability.
Consequence: brittle tests and expensive refactors.
BAD: instantiate concrete infra type in domain workflow. GOOD: depend on interface/port and inject implementation.

### NEVER bypass TypeScript strictness in design-critical paths

WHY: weak typing hides invalid states and contract violations.
Consequence: runtime errors that static checks should catch.
BAD: broad `any` and repeated `@ts-ignore`. GOOD: model uncertain values as `unknown` and narrow explicitly.

## Quick Commands

```bash
# Find design-risk hotspots
rg -n "any|@ts-ignore|new [A-Z].*Service\(|import .*infrastructure" src
```

```bash
# Inspect dependency graph
nx graph
```

```bash
# Re-check affected scope
nx affected -t lint,test,build --base=origin/main
```

## Quick Reference

### Strategic Architecture

- Dependency direction: [references/dep-inward-only.md](references/dep-inward-only.md)
- Acyclic dependencies: [references/dep-acyclic-dependencies.md](references/dep-acyclic-dependencies.md)
- Entity purity: [references/entity-pure-business-rules.md](references/entity-pure-business-rules.md)
- Rich entities: [references/entity-rich-not-anemic.md](references/entity-rich-not-anemic.md)

### Tactical Design

- Use-case isolation: [references/usecase-single-responsibility.md](references/usecase-single-responsibility.md)
- Explicit dependencies: [references/usecase-explicit-dependencies.md](references/usecase-explicit-dependencies.md)
- Structural guidance: [references/anti-patterns-and-frameworks.md](references/anti-patterns-and-frameworks.md)

### Quality and Testing

- Boundary testing: [references/test-boundary-verification.md](references/test-boundary-verification.md)
- Testable design: [references/test-testable-design.md](references/test-testable-design.md)
- Worked examples: [references/detailed-examples.md](references/detailed-examples.md)

## References

- [Martin Fowler Architecture](https://martinfowler.com/architecture/)
- [Refactoring Guru](https://refactoring.guru/)
