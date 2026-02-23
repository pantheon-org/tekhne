---
name: test-driven-development
description: Master Test-Driven Development with deterministic red-green-refactor workflows, test-first feature delivery, bug reproduction through failing tests, behavior-focused assertions, and refactoring safety; use when implementing new functions, changing APIs, fixing regressions, or restructuring code under test.
keywords:
  - tdd
  - test-driven development
  - red-green-refactor
  - write failing test first
  - test first
  - unit testing
  - behavior testing
  - arrange act assert
  - test isolation
  - refactor safely
---

# Test-Driven Development

Navigation hub for applying TDD in day-to-day implementation work.

## When to Use

- "Write tests first for this feature."
- "How should I do red-green-refactor here?"
- "I need to reproduce a bug with a failing test first."
- "I want to refactor safely while preserving behavior."

## When Not to Use

- End-to-end scenario design across full systems.
- Performance benchmarking and load testing.
- Security testing workflows.

## Scope

### In Scope

- Unit test design and behavior-driven assertions.
- Red-Green-Refactor cycle execution.
- Mock/stub isolation strategy.
- Naming and organization conventions for maintainable tests.

### Out of Scope

- Full E2E harness setup and browser automation pipelines.
- Infra-only test framework bootstrapping.
- Non-deterministic perf profiling.

## Workflow

1. Write the smallest failing test that expresses one behavior.
2. Run tests and verify failure is for the expected reason.
3. Implement minimal code to make the test pass.
4. Refactor while keeping the suite green.
5. Repeat for next behavior/edge case.

## Quick Commands

```bash
# Run full suite
bun test
```

```bash
# Watch mode while iterating
bun test --watch
```

```bash
# Run a specific file
bun test path/to/file.test.ts
```

```bash
# Elixir variant
mix test
```

## Anti-Patterns

### NEVER implement feature code before writing a failing test

WHY: skipping RED phase removes behavior-first design pressure.
BAD: write production code then add tests after. GOOD: test first, then minimal implementation.

### NEVER test implementation details instead of behavior

WHY: implementation-coupled tests break during valid refactors.
BAD: assert private calls/internal sequence. GOOD: assert observable outcomes and contract behavior.

### NEVER combine multiple behaviors into one test case

WHY: failures become ambiguous and debugging slows down.
BAD: one test covers create/login/update/delete flow. GOOD: one behavior per test.

### NEVER use arbitrary sleeps in unit tests

WHY: fixed waits create flaky and slow suites.
BAD: `await sleep(1000)`. GOOD: synchronize with deterministic signals or direct assertions.

## Quick Reference

| Topic | Reference |
| --- | --- |
| Red-Green-Refactor cycle | [references/cycle-write-test-first.md](references/cycle-write-test-first.md) |
| Verify failing tests first | [references/cycle-verify-test-fails-first.md](references/cycle-verify-test-fails-first.md) |
| AAA and behavior-first design | [references/design-aaa-pattern.md](references/design-aaa-pattern.md) |
| Avoid implementation-detail tests | [references/design-test-behavior-not-implementation.md](references/design-test-behavior-not-implementation.md) |
| Isolation and dependency injection | [references/isolate-use-dependency-injection.md](references/isolate-use-dependency-injection.md) |
| Flakiness and speed | [references/perf-fix-flaky-tests.md](references/perf-fix-flaky-tests.md) |

## Verification

```bash
# Evaluate skill quality
sh skills/skill-quality-auditor/scripts/evaluate.sh test-driven-development --json
```

```bash
# Lint this skill docs
bunx markdownlint-cli2 "skills/test-driven-development/**/*.md"
```

## References

- [Kent Beck - Test Driven Development](https://www.amazon.com/Test-Driven-Development-Kent-Beck/dp/0321146530)
- [Testing Implementation Details (Kent C. Dodds)](https://kentcdodds.com/blog/testing-implementation-details)
