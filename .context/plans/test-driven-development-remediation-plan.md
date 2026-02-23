---
plan_date: 2026-02-23
skill_name: test-driven-development
source_audit: .context/audits/test-driven-development-audit-2026-02-22.md
---

# Remediation Plan: test-driven-development

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 99/120 (82%) |
| **Current Grade** | B |
| **Target Score** | 108+/120 (A or higher) |
| **Priority** | Medium - Targeted improvements recommended |
| **Estimated Effort** | Medium (2-3 hours) |

## Critical Issues to Address

| Issue | Severity | Dimension | Current | Target |
| --- | --- | --- | ---: | ---: |
| Weak pattern recognition | High | D7 | 6/10 | 9/10 |
| Moderate specification compliance | Medium | D4 | 10/15 | 13/15 |
| Moderate progressive disclosure | Medium | D5 | 10/15 | 13/15 |

## Detailed Remediation Steps

### Phase 1: Pattern Recognition (D7) - High Priority

**Goal**: Increase D7 score from 6/10 to 9/10

**File**: `skills/test-driven-development/SKILL.md`

**Step 1.1**: Enhance frontmatter description

Update frontmatter to include high-signal keywords:

```yaml
---
name: test-driven-development
description: |
  Master Test-Driven Development (TDD) with comprehensive guidance on the Red-Green-Refactor cycle, 
  test design principles, isolation strategies, and best practices. Use when: writing new functions, 
  adding features, fixing bugs, refactoring code, or making API changes.
  
  Triggers: "write tests first", "TDD", "red green refactor", "test before code", 
  "how to do TDD", "unit test approach", "test-driven", "write failing test".
---
```

**Step 1.2**: Add explicit trigger phrases section

Add near the top of SKILL.md:

````markdown
## When to Use This Skill

Activate this skill when you encounter any of these phrases or scenarios:

### Explicit Triggers
- "Write tests first"
- "Use TDD for this feature"
- "Red-green-refactor"
- "Test-driven development"
- "Write a failing test"

### Implicit Triggers
- User asks to add a new function or feature
- User is fixing a bug (write test to reproduce first)
- User is refactoring existing code
- User asks about unit testing approach
- User mentions "test coverage" or "testing strategy"

### Decision Tree

```
Is this new code?
├── Yes → Use TDD (Red-Green-Refactor)
│   ├── Start with failing test
│   ├── Write minimal code to pass
│   └── Refactor while green
│
└── No, modifying existing code?
    ├── Bug fix? → Write failing test that reproduces bug
    ├── Refactor? → Ensure existing tests pass, then refactor
    └── Enhancement? → Write test for new behavior first
```
````

**Step 1.3**: Add concrete use-when examples

```markdown
## Use-When Examples

| User Request | TDD Approach |
| --- | --- |
| "Add a function to validate email" | Write test for valid email, then invalid, then implement |
| "Fix the null pointer bug in userService" | Write test that triggers the null pointer, fix, verify test passes |
| "Refactor the payment module" | Ensure tests exist and pass, refactor, verify tests still pass |
| "Create a new API endpoint" | Write integration test for endpoint contract first |
```

---

### Phase 2: Specification Compliance (D4) - Medium Priority

**Goal**: Increase D4 score from 10/15 to 13/15

**File**: `skills/test-driven-development/SKILL.md`

**Step 2.1**: Audit frontmatter completeness

Check for:
- `name` field
- Clear, comprehensive `description`
- Consistent formatting with other skills

**Step 2.2**: Tighten description

```yaml
---
name: test-driven-development
description: |
  Master Test-Driven Development (TDD) with comprehensive guidance on the Red-Green-Refactor cycle, 
  test design principles, isolation strategies, and best practices.
  
  Use when: writing new functions, adding features, fixing bugs, refactoring code, or making API changes.
  
  Scope: Unit tests, integration tests, test isolation, mocking strategies, test naming conventions,
  and verification workflows.
---
```

**Step 2.3**: Add explicit scope boundaries

```markdown
## Scope

### In Scope
- Unit test design and patterns
- Integration test strategies
- Mock/stub creation and usage
- Test isolation techniques
- Red-Green-Refactor workflow
- Test naming conventions
- Assertion best practices

### Out of Scope
- End-to-end testing frameworks (Cypress, Playwright)
- Performance testing
- Security testing
- Load testing
- Test infrastructure setup
```

---

### Phase 3: Progressive Disclosure (D5) - Medium Priority

**Goal**: Increase D5 score from 10/15 to 13/15

**Current State**:
- SKILL.md: 336 lines
- references/: 42 files

**File**: `skills/test-driven-development/SKILL.md`

**Step 3.1**: Restructure as navigation hub

```markdown
## Quick Reference

### Core Workflow
- [Red-Green-Refactor Cycle](references/red-green-refactor.md)
- [When to Write Tests](references/when-to-test.md)

### Test Design
- [Test Naming Conventions](references/naming-conventions.md)
- [AAA Pattern](references/aaa-pattern.md)
- [Test Data Builders](references/test-data-builders.md)

### Isolation Strategies
- [Mocking Guide](references/mocking.md)
- [Dependency Injection for Tests](references/dependency-injection.md)
- [Test Doubles](references/test-doubles.md)

### Best Practices
- [Test Smells](references/test-smells.md)
- [Coverage Guidelines](references/coverage.md)
- [Testing Anti-Patterns](references/anti-patterns.md)
```

**Step 3.2**: Extract detailed content to references

Create `skills/test-driven-development/references/red-green-refactor.md`:

```markdown
# Red-Green-Refactor Cycle

## Overview

The core TDD workflow consists of three phases:

1. **Red**: Write a failing test
2. **Green**: Write minimal code to pass
3. **Refactor**: Improve code while keeping tests green

## Detailed Steps

### Phase 1: Red (Write Failing Test)

1. Identify the next smallest piece of functionality
2. Write a test that expects this behavior
3. Run the test - it must fail
4. If test passes unexpectedly, the feature may already exist or test is wrong

**Duration**: Should take 1-5 minutes

### Phase 2: Green (Make It Pass)

1. Write the minimum code necessary to pass the test
2. Do not add extra functionality
3. Do not worry about code quality yet
4. Run the test - it must pass

**Duration**: Should take 1-10 minutes

### Phase 3: Refactor (Clean Up)

1. Review the code for improvements
2. Apply design principles (DRY, SOLID)
3. Rename variables for clarity
4. Extract methods if needed
5. Run tests after each change - must stay green

**Duration**: Variable, but typically 1-15 minutes

## Common Mistakes

- Skipping the red phase (writing test after code)
- Writing too much code in green phase
- Refactoring without tests passing first
- Not committing after each green-refactor cycle
```

**Step 3.3**: Update SKILL.md with concise summaries

```markdown
## The Red-Green-Refactor Cycle

| Phase | Action | Duration | Rule |
| --- | --- | --- | --- |
| Red | Write failing test | 1-5 min | Test MUST fail |
| Green | Minimal code to pass | 1-10 min | No extra features |
| Refactor | Improve while green | 1-15 min | Tests MUST stay green |

See [Red-Green-Refactor Guide](references/red-green-refactor.md) for detailed workflow.
```

---

### Phase 4: Enhanced Verification Section

**File**: `skills/test-driven-development/SKILL.md`

**Step 4.1**: Add verification checklist

```markdown
## Verification Checklist

After TDD session, verify:

- [ ] All tests pass (`npm test` or equivalent)
- [ ] No test was written after implementation
- [ ] Each test has a clear assertion
- [ ] Tests are independent (can run in any order)
- [ ] Test names describe the behavior being tested
- [ ] No test interdependencies (shared mutable state)
- [ ] Coverage meets project threshold
```

---

## Verification Commands

```bash
# Run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh test-driven-development --json

# Check line count
wc -l skills/test-driven-development/SKILL.md

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills

# Validate markdown
bunx markdownlint-cli2 "skills/test-driven-development/**/*.md"
```

## Success Criteria

| Criterion | Metric |
| --- | --- |
| Total Score | >= 108/120 |
| Grade | A or higher |
| D7: Pattern Recognition | >= 9/10 |
| D4: Specification Compliance | >= 13/15 |
| D5: Progressive Disclosure | >= 13/15 |
| Trigger phrases documented | >= 10 |
| Use-when examples | >= 4 |

## Timeline

| Phase | Duration | Dependencies |
| --- | --- | --- |
| Phase 1: Pattern Recognition | 1 hour | None |
| Phase 2: Specification Compliance | 30 min | None |
| Phase 3: Progressive Disclosure | 1.5 hours | Phase 2 |
| Phase 4: Verification | 30 min | Phase 3 |
| Verification | 15 min | All phases |

**Total Estimated Time**: 3.5 hours
