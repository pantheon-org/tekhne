---
name: test-driven-development
description: Guides TDD (test-driven development) with red-green-refactor workflows, test-first feature delivery, bug reproduction through failing tests, behavior-focused assertions, and refactoring safety. Use when writing unit tests, implementing new functions, adding test coverage, fixing regressions, changing APIs, or restructuring code under test — especially when a user says "write tests first", "TDD", or "test before code".
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

### Example: Red-Green-Refactor Cycle (TypeScript)

**RED — write the failing test first:**

```typescript
// add.test.ts
import { add } from "./add";

test("add returns the sum of two numbers", () => {
  expect(add(2, 3)).toBe(5);
});
// ❌ Fails: cannot find module './add'
```

**GREEN — implement the minimum code to pass:**

```typescript
// add.ts
export function add(a: number, b: number): number {
  return a + b;
}
// ✅ Test passes
```

**REFACTOR — improve without breaking green:**

```typescript
// add.ts — no logic change needed here, but you might rename,
// extract constants, or improve types while the suite stays green.
export const add = (a: number, b: number): number => a + b;
// ✅ Still green
```

### Example: Elixir variant

```elixir
# test/math_test.exs  (RED)
test "add/2 returns the sum" do
  assert Math.add(2, 3) == 5
end

# lib/math.ex  (GREEN)
defmodule Math do
  def add(a, b), do: a + b
end
```

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

```typescript
// BAD: production code written first, test added after
export function greet(name: string) { return `Hello, ${name}!`; }
// ... then test written to match existing impl

// GOOD: test written first, impl follows
test("greet returns a personalised greeting", () => {
  expect(greet("Ada")).toBe("Hello, Ada!");
});
// ❌ red → write greet() → ✅ green
```

### NEVER test implementation details instead of behavior

WHY: implementation-coupled tests break during valid refactors.

```typescript
// BAD: asserting internal call sequence
expect(mockFormatter.format).toHaveBeenCalledBefore(mockLogger.log);

// GOOD: assert observable output
expect(result).toBe("Hello, Ada!");
```

### NEVER combine multiple behaviors into one test case

WHY: failures become ambiguous and debugging slows down.

```typescript
// BAD: one test covers create/login/update/delete flow
test("user lifecycle", () => { /* 40 lines */ });

// GOOD: one behavior per test
test("createUser returns a user with the given name", () => { ... });
test("login returns a session token for valid credentials", () => { ... });
```

### NEVER use arbitrary sleeps in unit tests

WHY: fixed waits create flaky and slow suites.

```typescript
// BAD
await sleep(1000);
expect(result).toBeDefined();

// GOOD: synchronize with deterministic signals
await expect(promise).resolves.toBeDefined();
```

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
