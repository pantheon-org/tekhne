# Phase 5: Create Eval Scenarios

**Status:** Pending  
**Effort:** ~2-3 hours (iterative creation during Phase 3 authoring recommended)  
**Dependencies:** Phase 3 (can run concurrently during authoring)

## Description

Create 3-4 evaluation scenarios per skill (13 total) to validate trigger conditions and expected outputs.

**Recommendation:** Create eval scenarios iteratively during Phase 3 authoring (after each SKILL.md is drafted) rather than batching at end. This provides:
- Immediate validation of trigger conditions
- Earlier detection of scope issues
- Better alignment between instructions and expected behavior
- Reduced context switching between authoring and eval creation

## Eval Scenarios by Skill

### solid-principles/evals/ (3 scenarios)

1. **srp-god-class-refactor.md**
   - **Input:** God class handling auth, persistence, notifications
   - **Expected:** Identify SRP violations, propose split into focused classes
   - **Success:** Agent suggests UserAuthService, UserRepository, NotificationService

2. **ocp-extension-without-modification.md**
   - **Input:** Need to add new payment processor (currently if/else chain)
   - **Expected:** Propose Strategy pattern instead of modifying switch statement
   - **Success:** Agent creates PaymentProcessor interface + implementations

3. **dip-concrete-to-interface.md**
   - **Input:** Service directly instantiates concrete database client
   - **Expected:** Refactor to depend on interface, inject implementation
   - **Success:** Agent creates IDataStore interface, updates constructor

### clean-architecture/evals/ (4 scenarios)

1. **dependency-direction-violation.md**
   - **Input:** Infrastructure module imports from domain module
   - **Expected:** Identify inward dependency violation, propose inversion
   - **Success:** Agent suggests extracting interface to domain, implementing in infrastructure

2. **circular-dependency-resolution.md**
   - **Input:** AuthService imports UserService, UserService imports AuthService
   - **Expected:** Break cycle using interface extraction or event-based decoupling
   - **Success:** Agent creates shared contract module or pub/sub pattern

3. **boundary-crossing-data.md**
   - **Input:** Domain entity passed directly to HTTP response
   - **Expected:** Design proper DTO for cross-boundary data transfer
   - **Success:** Agent creates response DTO, implements mapper

4. **entity-layer-purity.md**
   - **Input:** Entity class imports database ORM decorators
   - **Expected:** Remove infrastructure coupling from entity
   - **Success:** Agent moves persistence annotations to infrastructure layer

### design-patterns/evals/ (3 scenarios)

1. **pattern-selection-decision.md**
   - **Input:** Requirements for extensible notification system
   - **Expected:** Evaluate Strategy vs Factory vs Builder, select with justification
   - **Success:** Agent documents win condition, cost, benefit for chosen pattern

2. **strategy-over-conditionals.md**
   - **Input:** Tax calculation with nested if/else for jurisdictions
   - **Expected:** Replace conditionals with Strategy pattern
   - **Success:** Agent creates TaxStrategy interface + jurisdiction implementations

3. **premature-abstraction-detection.md**
   - **Input:** Code with Factory pattern for single implementation
   - **Expected:** Identify YAGNI violation, suggest simplification
   - **Success:** Agent recommends removing pattern, using direct instantiation

### testable-design/evals/ (3 scenarios)

1. **dependency-injection-refactor.md**
   - **Input:** Service with hard-coded new DatabaseClient() call
   - **Expected:** Convert to constructor injection with interface
   - **Success:** Agent refactors to accept IDatabaseClient, updates tests

2. **boundary-test-design.md**
   - **Input:** Need to verify adapter correctly translates external API
   - **Expected:** Write boundary test that verifies contract without hitting real API
   - **Success:** Agent creates test using test double, validates mapping

3. **untestable-to-testable.md**
   - **Input:** Function with hidden dependencies (Date.now(), Math.random(), fetch())
   - **Expected:** Refactor to inject dependencies, make pure/testable
   - **Success:** Agent extracts clock/random/http interfaces, injects implementations

## File Format

Each eval scenario should follow this structure:

```markdown
---
name: <scenario-name>
skill: <skill-name>
difficulty: <easy|medium|hard>
---

# <Scenario Title>

## Context

Brief description of the situation/codebase state.

## Input

Code snippet or description of what the agent receives.

## Expected Behavior

What the agent should do, including specific outputs.

## Success Criteria

- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

## Sample Interaction

**User:** <example prompt>

**Agent:** <expected response pattern>
```

## Acceptance Criteria

- [ ] 13 total eval scenarios created (3+4+3+3)
- [ ] Each scenario has valid frontmatter
- [ ] Each scenario includes Context, Input, Expected Behavior, Success Criteria
- [ ] Scenarios test actual trigger conditions from SKILL.md
- [ ] All eval files named descriptively (kebab-case)

## Verification

```bash
# Count evals per skill
for skill in solid-principles clean-architecture design-patterns testable-design; do
  count=$(find skills/software-engineering/design-principles/$skill/evals -type f -name "*.md" | wc -l)
  echo "$skill: $count scenarios"
done

# Verify frontmatter
for eval_file in skills/software-engineering/design-principles/*/evals/*.md; do
  head -5 "$eval_file" | grep -q "^name:" || echo "Missing frontmatter in $eval_file"
done

# Total count
total=$(find skills/software-engineering/design-principles/*/evals -type f -name "*.md" | wc -l)
echo "Total: $total (expected: 13)"
```
