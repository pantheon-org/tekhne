---
plan_date: 2026-02-23
skill_name: software-design-principles
source_audit: .context/audits/software-design-principles-audit-2026-02-22.md
status: completed
completed_date: 2026-02-23
---

# Remediation Plan: software-design-principles

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 97/120 (80%) |
| **Current Grade** | B |
| **Target Score** | 105+/120 (B+ or higher) |
| **Priority** | Medium - Targeted improvements recommended |
| **Estimated Effort** | Medium (2-3 hours) |

## Critical Issues to Address

| Issue | Severity | Dimension | Current | Target |
| --- | --- | --- | ---: | ---: |
| Moderate procedural clarity | Medium | D2 | 10/15 | 13/15 |
| Moderate anti-pattern quality | Medium | D3 | 10/15 | 13/15 |
| Moderate progressive disclosure | Medium | D5 | 10/15 | 13/15 |

## Detailed Remediation Steps

### Phase 1: Procedural Clarity (D2) - Medium Priority

**Goal**: Increase D2 score from 10/15 to 13/15

**File**: `skills/software-design-principles/SKILL.md`

**Step 1.1**: Restructure workflow as deterministic sequence

Add explicit workflow section:

```markdown
## Design Decision Workflow

### Step 1: Identify Decision Type (Required)

Classify the design decision:
- **Architectural**: System structure, component boundaries
- **Tactical**: Implementation patterns, class design
- **Foundational**: Code quality, naming, organization

**Output**: Decision classification.

### Step 2: Apply Strategic Principles (Conditional: Architectural)

If architectural decision:
1. Define business constraints first
2. Identify scalability requirements
3. Consider team structure (Conway's Law)
4. Evaluate build vs. buy
5. Document ADR (Architecture Decision Record)

**Output**: ADR or design document.

### Step 3: Apply SOLID Principles (Conditional: Tactical)

If tactical decision, evaluate against SOLID:

| Principle | Check |
| --- | --- |
| Single Responsibility | Can you describe the class in one sentence? |
| Open/Closed | Can behavior extend without modification? |
| Liskov Substitution | Do subclasses fulfill parent contracts? |
| Interface Segregation | Do clients depend only on needed methods? |
| Dependency Inversion | Do high-level modules depend on abstractions? |

**Output**: SOLID compliance checklist.

### Step 4: Apply Structural Patterns (Conditional: Tactical)

Consider applicable patterns:
- Strategy: Varying algorithms
- Factory: Complex object creation
- Observer: Event notification
- Decorator: Adding responsibilities
- Adapter: Interface compatibility

**Output**: Pattern selection rationale.

### Step 5: Validate Against Anti-Patterns (Required)

Check each anti-pattern in this skill.
Refactor if violations detected.

**Output**: Anti-pattern compliance confirmation.

### Step 6: Document Tradeoffs (Required)

Record:
- What alternatives were considered
- Why the chosen approach wins
- Known limitations and mitigations

**Output**: Tradeoff documentation.
```

**Step 1.2**: Add scope control section

```markdown
## When to Use / When Not to Use

### Use This Skill When:

- Starting a new feature or module design
- Refactoring existing code
- Evaluating architectural proposals
- Making technology selections
- Conducting design reviews

### Do Not Use This Skill When:

- Making minor code changes (use code review patterns)
- Fixing bugs (use debugging workflows)
- Working on one-liner fixes
- The decision is purely operational
```

---

### Phase 2: Anti-Pattern Quality (D3) - Medium Priority

**Goal**: Increase D3 score from 10/15 to 13/15

**File**: `skills/software-design-principles/SKILL.md`

**Step 2.1**: Enhance existing anti-patterns with NEVER/WHY/BAD-GOOD

Add explicit anti-pattern section:

```markdown
## Anti-Patterns

### NEVER Design for Imagined Future Requirements

- **WHY**: YAGNI violations create unnecessary complexity.
- **Consequence**: Maintenance burden, delayed delivery, over-engineering.
- **BAD**: "Let's add an abstraction layer in case we switch databases someday."
- **GOOD**: "We use PostgreSQL. If we switch, we'll refactor then. Current cost: 0."

### NEVER Allow Circular Dependencies

- **WHY**: Creates tight coupling, prevents independent testing/deployment.
- **Consequence**: Build failures, runtime errors, impossible to reason about.
- **BAD**: Module A imports B, B imports A.
- **GOOD**: Extract shared logic to module C; A and B both import C.

### NEVER Use God Classes

- **WHY**: Violates Single Responsibility Principle.
- **Consequence**: Untestable code, merge conflicts, impossible to understand.
- **BAD**: A `UserManager` class handling authentication, preferences, notifications, and reporting.
- **GOOD**: Separate `Authenticator`, `UserPreferences`, `NotificationService`, `UserReporter`.

### NEVER Prematurely Optimize

- **WHY**: Optimization without measurement leads to complexity without benefit.
- **Consequence**: Harder to read code, potential new bugs, wasted effort.
- **BAD**: "I'll cache this because it might be slow."
- **GOOD**: "I measured this function at 50ms. Caching reduces to 5ms. Worth it."

### NEVER Hard-Code Configuration

- **WHY**: Prevents environment-specific behavior and secure secret management.
- **Consequence**: Security vulnerabilities, deployment complications.
- **BAD**: `const dbPassword = "myPassword123";`
- **GOOD**: `const dbPassword = process.env.DB_PASSWORD;`

### NEVER Skip Interface Contracts

- **WHY**: Interfaces enable testing and future flexibility.
- **Consequence**: Tight coupling, difficult mocking, hard to change implementations.
- **BAD**: Direct instantiation: `const service = new ConcreteService();`
- **GOOD**: Dependency injection: `constructor(private service: IService) {}`
```

**Step 2.2**: Tie anti-patterns to repository-specific risks

```markdown
### Project-Specific Anti-Patterns

#### NEVER Bypass TypeScript Strict Mode

- **WHY**: Type safety prevents runtime errors and improves IDE support.
- **Consequence**: Loss of type safety guarantees, potential runtime errors.
- **BAD**: Using `any` or `@ts-ignore` to silence errors.
- **GOOD**: Fix the type definition or use `unknown` with type guards.
```

---

### Phase 3: Progressive Disclosure (D5) - Medium Priority

**Goal**: Increase D5 score from 10/15 to 13/15

**Current State**:

- SKILL.md: 382 lines
- references/: 44 files (already good reference structure)

**File**: `skills/software-design-principles/SKILL.md`

**Step 3.1**: Create summary hub structure

Restructure SKILL.md to serve as navigation hub:

```markdown
## Quick Reference

### Strategic Architecture
- [Architecture Decision Records](references/architecture-decision-records.md)
- [Strategic vs Tactical Decisions](references/strategic-tactical-split.md)

### SOLID Principles
- [Single Responsibility](references/solid/single-responsibility.md)
- [Open/Closed](references/solid/open-closed.md)
- [Liskov Substitution](references/solid/liskov-substitution.md)
- [Interface Segregation](references/solid/interface-segregation.md)
- [Dependency Inversion](references/solid/dependency-inversion.md)

### Structural Patterns
- [Pattern Catalog](references/patterns/README.md)
- [Strategy Pattern](references/patterns/strategy.md)
- [Factory Pattern](references/patterns/factory.md)
- [Observer Pattern](references/patterns/observer.md)

### Code Quality Fundamentals
- [Naming Conventions](references/naming-conventions.md)
- [Function Design](references/function-design.md)
- [Error Handling](references/error-handling.md)
```

**Step 3.2**: Extract verbose content

Move detailed explanations from SKILL.md to references:

- Move full SOLID explanations to individual reference files
- Move pattern examples to pattern reference files
- Keep only one-paragraph summaries in SKILL.md

**Step 3.3**: Add concise summaries with links

```markdown
## SOLID Principles Summary

| Principle | One-Liner | Detail |
| --- | --- | --- |
| S | One reason to change | [Full Guide](references/solid/single-responsibility.md) |
| O | Extend, don't modify | [Full Guide](references/solid/open-closed.md) |
| L | Subtypes must be substitutable | [Full Guide](references/solid/liskov-substitution.md) |
| I | Many specific interfaces | [Full Guide](references/solid/interface-segregation.md) |
| D | Depend on abstractions | [Full Guide](references/solid/dependency-inversion.md) |
```

---

## Verification Commands

```bash
# Run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh software-design-principles --json

# Check line count reduction
wc -l skills/software-design-principles/SKILL.md

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills

# Validate markdown
bunx markdownlint-cli2 "skills/software-design-principles/**/*.md"
```

## Success Criteria

| Criterion | Metric |
| --- | --- |
| Total Score | >= 105/120 |
| Grade | B+ or higher |
| D2: Mindset + Procedures | >= 13/15 |
| D3: Anti-Pattern Quality | >= 13/15 |
| D5: Progressive Disclosure | >= 13/15 |
| Anti-patterns with BAD/GOOD | >= 6 |
| SKILL.md line count | < 300 lines |

## Timeline

| Phase | Duration | Dependencies |
| --- | --- | --- |
| Phase 1: Procedural Clarity | 1 hour | None |
| Phase 2: Anti-Pattern Quality | 1 hour | None |
| Phase 3: Progressive Disclosure | 1 hour | Phase 2 |
| Verification | 30 min | All phases |

**Total Estimated Time**: 3.5 hours

## Notes

- Rating: **8/10** - Already follows Format B template very well with detailed phases and code examples
- Strong structure with 3 phases and Timeline table
- Addresses SOLID principles and design patterns comprehensively
- Has Estimated Effort in Executive Summary
- Code examples in remediation steps are specific and actionable
