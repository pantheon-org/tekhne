---
name: software-design-principles
description: Apply SOLID principles, detect design anti-patterns, and evaluate architectural tradeoffs for code reviews, design decisions, and refactoring. Use when reviewing code for maintainability issues, evaluating architecture decisions, identifying code smells and technical debt, or refactoring modules with coupling or testability problems.
---

# Software Design Principles

Navigation hub for strategic architecture, tactical design, and code quality decisions.

## When to Use

- Reviewing code for maintainability, coupling, testability, or code smells.
- Evaluating competing architectural or design approaches with explicit tradeoffs.
- Designing a new module, service boundary, or cross-team interface.
- Refactoring legacy areas with repeated design problems or technical debt.
- Assessing design decisions before implementation to catch issues early.

## When Not to Use

- One-line bug fixes with no design decision.
- Purely operational tasks (deployment/scripts) without architecture impact.
- Line-by-line code style or formatting review (use linters instead).

## Design Decision Workflow

### Step 1: Identify Decision Type
**Output:** Classify as architectural, tactical, or foundational.

Example classification:
- **Architectural:** Service boundaries, dependency direction, module structure
- **Tactical:** Class design, method extraction, interface definition
- **Foundational:** Language features, build tooling, testing strategy

### Step 2: Apply Strategic Checks (Architecture Decisions)
**Output:** Boundary/dependency direction assessment + ADR rationale.

Checklist:
- [ ] Dependencies point inward (no outer layers importing inner layers)
- [ ] No circular dependencies between modules
- [ ] Clear ownership of contracts/interfaces
- [ ] Entities remain pure (no infrastructure leakage)

**Example ADR snippet:**
```
Decision: Extract shared authentication contract into separate module.
Rationale: Breaks circular dependency between auth-service and user-service.
Alternatives: Merge modules (loses separation), add adapter layer (adds complexity).
Risk: Requires migration of existing consumers.
```

### Step 3: Apply SOLID Checks (Tactical Decisions)
**Output:** Compliance assessment + required refactors.

| Principle | Check | Refactor Signal |
| --- | --- | --- |
| SRP | Does this class have one reason to change? | Multiple concerns → split into focused classes |
| OCP | Can you extend behavior without modifying existing code? | Adding features requires editing stable code → use abstraction |
| LSP | Do subtypes preserve base behavior contracts? | Subtype breaks parent assumptions → redesign hierarchy |
| ISP | Do clients depend only on methods they use? | Clients ignore many interface methods → split interface |
| DIP | Do you depend on abstractions, not concrete details? | Direct concrete coupling → introduce interface/port |

**Example compliance note:**
```
SRP violation: UserService handles auth, persistence, and notifications.
Refactor: Extract NotificationService and PersistenceGateway.
Validation: Each class now has one reason to change.
```

### Step 4: Select Structural Pattern (Only If It Reduces Complexity)
**Output:** Pattern choice with explicit win condition.

Ask before choosing a pattern:
- Does this pattern solve a concrete problem in the current design?
- What complexity does it remove vs. add?
- Can the team maintain it?

**Example decision:**
```
Pattern: Strategy (not Factory).
Win condition: Eliminates if/else chains for payment processor selection.
Cost: One extra interface + implementations. Benefit: Easy to add new processors.
```

### Step 5: Validate Anti-Patterns
**Output:** Violations listed with BAD/GOOD corrective action.

Run anti-pattern checks (see section below) and document findings:
```
Anti-pattern: Hard-coded environment config.
Violation: Database URL in source code.
BAD: const DB_URL = "postgres://prod.example.com"
GOOD: const DB_URL = process.env.DATABASE_URL
```

### Step 6: Document Tradeoffs and Limitations
**Output:** Alternatives considered, decision, and risks.

Template:
```
Decision: Use event-driven architecture for order processing.
Alternatives: 
  - Synchronous RPC (simpler, tightly coupled, harder to scale)
  - Batch processing (decoupled, eventual consistency delays)
Chosen: Event-driven (loose coupling, handles scale, adds complexity)
Risks: Eventual consistency requires idempotent handlers; harder to debug.
Validation: Peer review before implementation.
```

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

**WHY:** YAGNI violations create complexity without proven value.  
**Consequence:** slower delivery and harder maintenance.

**BAD:** Add abstraction "in case" of possible future DB migration.  
**GOOD:** Solve current need and refactor when trigger appears.

### NEVER allow circular dependencies

**WHY:** Cycles break modular reasoning and complicate builds.  
**Consequence:** Fragile deployment and high change risk.

**BAD:** Module A imports B and B imports A.  
**GOOD:** Extract shared contract/module and invert dependencies.

### NEVER use god classes/services

**WHY:** Violates single responsibility and creates merge hotspots.  
**Consequence:** Low testability and unclear ownership.

**BAD:** One class handles auth, persistence, notifications, and reporting.  
**GOOD:** Split into focused collaborators with clear boundaries.

### NEVER optimize before measurement

**WHY:** Unmeasured optimization often trades clarity for no gain.  
**Consequence:** Needless complexity and hidden bugs.

**BAD:** Add cache because function "might be slow."  
**GOOD:** Measure baseline, optimize when threshold is exceeded.

### NEVER hard-code environment configuration and secrets

**WHY:** Hard-coded values block secure deployment and portability.  
**Consequence:** Security leaks and environment drift.

**BAD:** Inline passwords/URLs in source.  
**GOOD:** Use environment/config providers.

### NEVER bypass interface contracts when integrating dependencies

**WHY:** Direct concrete coupling reduces substitution and testability.  
**Consequence:** Brittle tests and expensive refactors.

**BAD:** Instantiate concrete infra type in domain workflow.  
**GOOD:** Depend on interface/port and inject implementation.

### NEVER bypass TypeScript strictness in design-critical paths

**WHY:** Weak typing hides invalid states and contract violations.  
**Consequence:** Runtime errors that static checks should catch.

**BAD:** Broad `any` and repeated `@ts-ignore`.  
**GOOD:** Model uncertain values as `unknown` and narrow explicitly.

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
