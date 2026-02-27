---
name: bdd-testing
description: "Write and maintain Behavior-Driven Development tests with Gherkin and Cucumber. Use when defining acceptance scenarios, writing feature files, implementing step definitions, running Three Amigos sessions, or diagnosing BDD test quality issues. Keywords: bdd, gherkin, cucumber, given when then, feature files, step definitions, acceptance criteria, three amigos, example mapping."
---

# BDD Testing

## When to Use

Use this skill when behavior needs to be specified and validated in shared, business-readable scenarios.

## When Not to Use

Do not use BDD feature files for low-level unit behavior that is internal and not stakeholder-facing.

## Core Principles

1. Specify observable behavior, not implementation details.
2. Keep a shared language across product, engineering, and QA.
3. Scenarios should be deterministic and independently executable.
4. Feature files should remain readable by non-developers.

## Deterministic Workflow

1. Align on examples in a Three Amigos discussion.
2. Write scenarios in Gherkin using Given/When/Then.
3. Implement step definitions that map to business language.
4. Execute Cucumber and review failures by scenario intent.
5. Refactor scenarios and steps to remove duplication.

## Quick Commands

### Run all feature tests

```bash
npx cucumber-js features
```

Expected result: scenario pass/fail summary and non-zero exit on failures.

### Dry-run to detect missing step definitions

```bash
npx cucumber-js --dry-run features
```

Expected result: undefined steps listed without full execution.

### Run a tagged subset

```bash
npx cucumber-js --tags "@smoke and not @wip"
```

Expected result: only matching scenarios execute.

### Generate JSON report

```bash
npx cucumber-js --format json:reports/cucumber-report.json
```

Expected result: machine-readable execution report for CI/reporting.

### Evaluate this skill quality

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh bdd-testing --json
```

Expected result: updated quality dimensions and grade.

## Constraints

### Hard Constraints

- Use business-facing language in feature files.
- Keep each scenario focused on one behavior.
- Avoid cross-scenario state coupling.

### Soft Constraints

- Scenario count per feature should stay manageable.
- Prefer explicit examples over abstract wording.
- Reuse step phrases only when semantics are identical.

## Anti-Patterns

### NEVER encode implementation details in Gherkin steps

**WHY:** Implementation-centric steps break when internals change.

**BAD:** `When I click the submit button and call validateForm()`
**GOOD:** `When I submit the form`

**Consequence:** Scenarios become brittle and unreadable to stakeholders.

### NEVER skip Three Amigos before writing key scenarios

**WHY:** Missing perspectives create ambiguous or incomplete acceptance behavior.

**BAD:** Engineering writes scenarios alone from assumptions.
**GOOD:** Product, QA, and engineering align on examples first.

**Consequence:** Rework increases and acceptance disputes appear late.

### NEVER use vague Then steps without observable outcomes

**WHY:** Unverifiable outcomes cannot fail meaningfully.

**BAD:** `Then it should work correctly`
**GOOD:** `Then I should see "Order confirmed"`

**Consequence:** Tests pass without validating user-visible behavior.

### NEVER couple scenarios with ordering dependencies

**WHY:** Scenario order dependence creates flaky suites.

**BAD:** Scenario B assumes data created by scenario A.
**GOOD:** Each scenario creates or mocks its own prerequisites.

**Consequence:** Parallel runs and CI become unstable.

## References

- `references/principles-core-philosophy.md`
- `references/gherkin-syntax.md`
- `references/principles-three-amigos.md`
- `references/principles-example-mapping.md`
- `references/cucumber-setup.md`
