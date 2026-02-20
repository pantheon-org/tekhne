---
name: test-driven-development
description: |
  Master Test-Driven Development (TDD) with comprehensive guidance on the Red-Green-Refactor cycle, test design principles, isolation strategies, and best practices. Use when: writing new functions, adding features, fixing bugs, refactoring code, or making API changes.
keywords:
  - TDD
  - test-driven development
  - red-green-refactor
  - unit testing
  - test first
  - test design
  - testing best practices
  - AAA pattern
  - arrange-act-assert
  - test isolation
  - test behavior
  - test coverage
---

# Test-Driven Development (TDD)

This skill provides comprehensive TDD guidance including the Red-Green-Refactor cycle, test design principles, isolation strategies, and 42 community best practices organized by impact level.

## When to Apply TDD

Use TDD proactively when:

- **Writing new functions or modules** - Design the API through tests
- **Adding new features** - Define expected behavior before implementation
- **Fixing bugs** - Write a failing test that reproduces the bug, then fix it
- **Refactoring existing code** - Ensure behavior is preserved
- **Making API changes** - Tests document and verify the contract

## The Red-Green-Refactor Cycle

**RED → GREEN → REFACTOR** - Repeat for every behavior or edge case.

## Critical TDD Rules

These rules prevent 40-90% of defects:

1. **Test-first forces edge case discovery** - Writing tests before implementation reveals boundary conditions you wouldn't consider during coding
2. **Verify false positives** - Always confirm tests fail for the right reason (not compilation errors)
3. **Minimal code to pass** - No speculative features (YAGNI principle)
4. **One test per behavior** - Single responsibility for edge cases, happy path, error cases
5. **Test observable behavior** - Tests verify outcomes, not internal mechanics (survives refactoring)
6. **Run full suite before commit** - Never commit with failing tests

## Before Writing a Test

Ask yourself these questions to ensure quality:

- **What's the smallest behavior I can verify?** - Focus on one thing
- **What edge case would break this assumption?** - Think boundary conditions
- **How would I explain this behavior to a teammate?** - Clarity test
- **What would make this test fail incorrectly?** - Verify it tests the right thing

## TDD Workflow Example

**Step 1: Write failing test (RED)**

```typescript
// calculate-discount.test.ts
import { calculateDiscount } from './calculate-discount';

describe('calculateDiscount', () => {
  it('should apply 10% discount to amounts over $100', () => {
    // Arrange
    const amount = 150;
    const discountRate = 0.1;

    // Act
    const result = calculateDiscount(amount, discountRate);

    // Assert
    expect(result).toBe(15);
  });
});
```

Run: `bun test calculate-discount.test.ts`
Expected: Test FAILS (function doesn't exist) → **RED**

**Step 2: Implement minimal code (GREEN)**

```typescript
// calculate-discount.ts
export const calculateDiscount = (amount: number, rate: number): number => {
  if (amount <= 100) return 0;
  return amount * rate;
};
```

Run: `bun test calculate-discount.test.ts`
Expected: Test PASSES → **GREEN**

**Step 3: Refactor (if needed)**

Clean up code, improve naming, extract functions - keep tests green.

**Step 4: Next test case (RED again)**

```typescript
it('should handle zero discount rate', () => {
  expect(calculateDiscount(200, 0)).toBe(0);
});
```

Continue the cycle.

## Stack-Specific Test Patterns

### TypeScript Frontend (Bun/Node)

```typescript
// component.test.tsx
describe('LoginForm', () => {
  it('should submit credentials on form submit', async () => {
    // Arrange
    const onSubmit = jest.fn();
    render(<LoginForm onSubmit={onSubmit} />);

    // Act
    fireEvent.change(screen.getByLabelText('Email'), {
      target: { value: 'user@example.com' },
    });
    fireEvent.change(screen.getByLabelText('Password'), {
      target: { value: 'password123' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Login' }));

    // Assert
    await waitFor(() => {
      expect(onSubmit).toHaveBeenCalledWith({
        email: 'user@example.com',
        password: 'password123',
      });
    });
  });
});
```

Run: `bun test` or `yarn test`

### Elixir Backend

```elixir
# test/my_app/calculator_test.exs
defmodule MyApp.CalculatorTest do
  use ExUnit.Case, async: true

  describe "calculate_discount/2" do
    test "applies 10% discount for amounts over $100" do
      # Arrange
      amount = 150
      rate = 0.1

      # Act
      result = Calculator.calculate_discount(amount, rate)

      # Assert
      assert result == 15.0
    end

    test "returns 0 discount for amounts at or below $100" do
      assert Calculator.calculate_discount(100, 0.1) == 0
      assert Calculator.calculate_discount(50, 0.1) == 0
    end
  end
end
```

Run: `mix test`

## Test Design Anti-Patterns

**NEVER do these:**

```typescript
// NEVER: Write implementation before tests
export const complexLogic = () => { /* 50 lines */ };
// WHY: Forces you to test code that's already designed around implementation.
// Result: Tests become brittle, miss edge cases, and merely verify existing behavior
// Impact: 40-90% more defects slip through

// NEVER: Test implementation details
it('should call the database', () => {
  const spy = jest.spyOn(db, 'query');
  getUsers();
  expect(spy).toHaveBeenCalled();
});
// WHY: This breaks when you add a cache layer (optimization). The behavior 
// (getting users) is still correct, but test fails.
// Impact: 50-80% more test maintenance burden during refactoring

// NEVER: Multiple behaviors in one test
it('should handle user workflow', () => {
  createUser();
  loginUser();
  updateProfile();
  deleteAccount();
});
// WHY: When this fails, you don't know which behavior broke. Debugging takes
// 3-5× longer. Violates single responsibility.
// Impact: Poor failure diagnostics, slow debugging

// NEVER: Use arbitrary waits
it('should load data', async () => {
  loadData();
  await sleep(1000); // Hope it's done?
  expect(data).toBeDefined();
});
// WHY: Flaky on slow CI servers, wastes time on fast machines.
// Impact: 20-40% of tests become non-deterministic, CI becomes unreliable
```

**GOOD patterns:**

```typescript
// GOOD: Testing behavior
it('should return all active users', async () => {
  const users = await getUsers({ status: 'active' });
  expect(users).toHaveLength(3);
  expect(users.every(u => u.status === 'active')).toBe(true);
});

// GOOD: Single behavior per test
it('should create user with valid email', async () => {
  const user = await createUser({ email: 'test@example.com' });
  expect(user.email).toBe('test@example.com');
});

it('should reject user with invalid email', async () => {
  await expect(createUser({ email: 'invalid' }))
    .rejects
    .toThrow('Invalid email format');
});
```

## When to Load Reference Documentation

This skill includes 42 detailed best practices in `references/` organized by impact level. Load them strategically:

### For Red-Green-Refactor Cycle Issues
**MANDATORY - READ THESE 6 FILES** when implementing or teaching TDD workflow:
- `cycle-write-test-first.md` - Test-first discipline (prevents 40-90% defects)
- `cycle-verify-test-fails-first.md` - Avoid false positives
- `cycle-minimal-code-to-pass.md` - YAGNI principle in practice
- `cycle-refactor-after-green.md` - When and how to refactor safely
- `cycle-small-increments.md` - Incremental development rhythm
- `cycle-maintain-test-list.md` - Managing test backlog

### For Test Design Problems
**LOAD ON DEMAND** when tests are unclear, brittle, or hard to maintain:
- `design-aaa-pattern.md` - Structuring tests (2-3× readability improvement)
- `design-test-behavior-not-implementation.md` - Reducing brittleness (50-80% reduction)
- `design-descriptive-test-names.md` - Clear test documentation
- `design-one-assertion-per-test.md` - Single responsibility for tests
- `design-given-when-then.md` - BDD-style alternative to AAA
- `design-avoid-logic-in-tests.md` - Keeping tests simple

### For Test Isolation Issues
**LOAD ON DEMAND** when experiencing test pollution or flakiness:
- `isolate-independent-tests.md` - Tests must not depend on each other
- `isolate-avoid-shared-state.md` - Preventing test pollution
- `isolate-use-mocks-judiciously.md` - When to mock vs use real dependencies
- `isolate-test-doubles.md` - Stubs, spies, mocks, fakes
- `isolate-dependency-injection.md` - Making dependencies explicit
- `isolate-reset-state.md` - Cleanup strategies

### For Test Data Challenges
**LOAD ON DEMAND** when test setup is complex or unclear:
- `data-use-factories.md` - Reusable test data builders
- `data-builder-pattern.md` - Fluent API for complex objects
- `data-minimal-setup.md` - Reducing noise in tests
- `data-avoid-mystery-guests.md` - Making test data explicit
- `data-unique-identifiers.md` - Preventing collisions

### For Assertion and Verification Issues
**LOAD WHEN NEEDED** for advanced verification techniques:
- `assert-specific-assertions.md` - Precise matchers (toBe, toEqual, toContain)
- `assert-error-messages.md` - Verifying exception messages
- `assert-custom-matchers.md` - Domain-specific assertions
- `assert-snapshot-testing.md` - Complex output verification (use sparingly)
- `assert-no-assertions-antipattern.md` - Every test must assert

### For Organizing Tests
**LOAD WHEN NEEDED** for project-wide test organization:
- `org-describe-blocks.md` - Grouping related tests
- `org-test-file-structure.md` - File collocation patterns
- `org-test-naming-conventions.md` - Consistent naming
- `org-shared-setup.md` - beforeEach/beforeAll patterns
- `org-test-documentation.md` - Tests as living documentation

### For Performance and Reliability
**LOAD WHEN NEEDED** when tests are slow (>100ms) or flaky:
- `perf-fast-tests.md` - Keeping tests under 100ms
- `perf-avoid-sleeps.md` - Eliminating arbitrary waits
- `perf-parallel-execution.md` - Concurrent test execution
- `perf-test-flakiness.md` - Non-deterministic test elimination
- `perf-setup-teardown.md` - Efficient resource management

### For Test Strategy Decisions
**DO NOT LOAD** unless discussing test distribution or coverage:
- `strat-test-pyramid.md` - Unit vs integration vs E2E distribution
- `strat-unit-vs-integration.md` - Choosing the right test type
- `strat-end-to-end-sparingly.md` - E2E for critical paths only
- `strat-coverage-metrics.md` - Coverage as guide, not goal
- `strat-mutation-testing.md` - Testing your tests

**Note**: All reference files include impact metrics, code examples, rationale, and research citations.

## Verification Commands

- **TypeScript/Bun**: `bun test`
- **TypeScript/Node**: `npm test` or `yarn test`
- **Elixir**: `mix test`
- **Watch mode**: `bun test --watch` or `mix test --stale`

## Key Takeaways

1. **Write tests first** - Design through tests, not as an afterthought
2. **Red → Green → Refactor** - Follow the cycle religiously
3. **Test behavior** - Not implementation details
4. **One test, one behavior** - Keep tests focused
5. **AAA pattern** - Structure tests consistently
6. **Fast feedback** - Tests should run in milliseconds
7. **No skipping tests** - Fix or remove, never skip
8. **Tests are documentation** - They explain how the code should behave

## Further Reading

- [Test Driven Development by Kent Beck](https://www.amazon.com/Test-Driven-Development-Kent-Beck/dp/0321146530)
- [Growing Object-Oriented Software, Guided by Tests](https://www.amazon.com/Growing-Object-Oriented-Software-Guided-Tests/dp/0321503627)
- [Working Effectively with Legacy Code by Michael Feathers](https://www.amazon.com/Working-Effectively-Legacy-Michael-Feathers/dp/0131177052)
- [Testing Implementation Details - Kent C. Dodds](https://kentcdodds.com/blog/testing-implementation-details)
