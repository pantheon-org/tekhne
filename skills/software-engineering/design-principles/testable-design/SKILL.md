---
name: testable-design
description: Design code for testability using boundary isolation, dependency injection, and testable architecture patterns. Use when designing test strategies, writing unit tests, improving test coverage, refactoring untestable or hard-to-test code, working with mocking or test doubles, or applying TDD practices.
---

# Testable Design

Architecture and design patterns for testable code, focusing on boundary isolation and dependency injection.

## Philosophy

- Treat difficulty in testing as feedback about the design, not as a testing problem.
- Fix coupling rather than compromising the architecture to make a test pass.
- Test behaviour at architectural boundaries, not the internals behind them.
- Prefer small, fast unit tests over pure logic, and a few integration tests at each adapter.

## When to Use

- Designing test strategies for new features
- Refactoring untestable code with hard dependencies
- Improving test coverage by isolating layers
- Evaluating whether code is testable before implementation
- Designing boundaries that enable fast unit tests
- Reviewing a test suite that relies on static mocking, real databases or skipped tests

## When Not to Use

- Class-level SOLID violations (use solid-principles)
- Architectural boundary decisions (use clean-architecture)
- Choosing structural patterns (use design-patterns)
- Writing a test-first workflow from scratch (use test-driven-development)

## Testable Design Principles

### Tests Are Architecture

If code is hard to test, it's a design problem, not a testing problem.

### Boundary Verification

Test at architectural boundaries, not internal implementation details:

- **Unit tests:** Test pure business logic (entities, use cases) in isolation
- **Integration tests:** Test boundary adapters (repositories, controllers, gateways)
- **End-to-end tests:** Test complete user workflows across boundaries

### Layer Isolation

Each layer should be testable in isolation:

- **Entities:** Pure functions, no dependencies
- **Use Cases:** Depend on interfaces (ports), not concrete implementations
- **Adapters:** Test with fake use cases or real infrastructure
- **Infrastructure:** Test with integration tests, not unit tests

## Workflow

### Step 1: Identify Hard-to-Test Code

**Output:** List of testability blockers.

**Example:**

```text
Testability blocker: OrderService instantiates PostgresRepository in constructor.
Problem: Cannot unit test OrderService without a real database.
Refactor: Inject IOrderRepository interface; provide mock in tests.
```

### Step 2: Apply Dependency Injection

**Output:** Dependencies injected via constructor or method parameters.

Template:

```typescript
// Before: hard to test (concrete dependency)
class OrderService {
  private repo = new PostgresOrderRepository()
  
  async createOrder(input: OrderInput): Promise<Order> {
    // Logic using this.repo
  }
}

// After: testable (injected interface)
class OrderService {
  constructor(private repo: IOrderRepository) {}
  
  async createOrder(input: OrderInput): Promise<Order> {
    // Logic using this.repo (mockable in tests)
  }
}
```

### Step 3: Isolate Side Effects

**Output:** Pure business logic separated from side effects.

Use the Humble Object pattern:

- **Humble object:** Thin adapter with minimal logic (controller, presenter, gateway)
- **Testable object:** Pure logic with no infrastructure dependencies (use case, entity)

**Example:**

```typescript
// Humble object (minimal logic, hard to test)
class HttpController {
  constructor(private useCase: CreateOrderUseCase) {}
  
  async handle(req: Request, res: Response): Promise<void> {
    const input = { userId: req.body.userId, items: req.body.items }
    const output = await this.useCase.execute(input)
    res.json(output)
  }
}

// Testable object (pure logic, easy to test)
class CreateOrderUseCase {
  constructor(private repo: IOrderRepository) {}
  
  async execute(input: CreateOrderInput): Promise<CreateOrderOutput> {
    // Business logic here (unit testable with mock repo)
  }
}
```

### Step 4: Design Test Doubles

**Output:** Mocks, stubs, or fakes for dependencies.

Choose appropriate test double:

- **Stub:** Returns fixed data (for queries)
- **Mock:** Verifies behavior (for commands)
- **Fake:** Lightweight implementation (in-memory DB)

**Example:**

```typescript
// Stub (for queries)
class StubOrderRepository implements IOrderRepository {
  async findById(id: string): Promise<Order | null> {
    return new Order({ id, status: 'pending' })
  }
}

// Mock (for commands)
class MockOrderRepository implements IOrderRepository {
  saveWasCalled = false
  
  async save(order: Order): Promise<void> {
    this.saveWasCalled = true
  }
}

// Fake (lightweight alternative)
class InMemoryOrderRepository implements IOrderRepository {
  private orders = new Map<string, Order>()
  
  async save(order: Order): Promise<void> {
    this.orders.set(order.id, order)
  }
  
  async findById(id: string): Promise<Order | null> {
    return this.orders.get(id) || null
  }
}
```

### Step 5: Verify Boundary Tests

**Output:** Test coverage at each architectural boundary.

Check:

- [ ] Entities are unit tested (pure logic, no dependencies)
- [ ] Use cases are unit tested (with mocked ports)
- [ ] Adapters are integration tested (with real or fake infrastructure)
- [ ] Controllers are integration tested (with real use cases or fakes)

**Example:**

```text
Unit test: CreateOrderUseCase with mock IOrderRepository
Integration test: PostgresOrderRepository with real database
End-to-end test: POST /orders with real HTTP server and database
```

## Anti-Patterns

### NEVER instantiate concrete dependencies in business logic

**WHY:** A hard-wired dependency cannot be replaced in a test, so the class needs real infrastructure to run.

**BAD:** `new PostgresRepository()` in the `OrderService` constructor.
**GOOD:** Depend on the `IRepository` interface; inject the implementation.

### NEVER test implementation details

**WHY:** Tests coupled to internal calls break on every refactor while saying nothing about behaviour.

**BAD:** Test that method X calls method Y internally.
**GOOD:** Test public behaviour: given input, expect output.

### NEVER use real infrastructure in unit tests

**WHY:** Real databases and networks make unit tests slow, flaky and unable to pinpoint a failure.

**BAD:** Unit test connects to a real database.
**GOOD:** Unit test uses a mock repository; integration test uses the real database.

### NEVER skip tests because code is "hard to test"

**WHY:** Hard-to-test code is a design signal; skipping the test hides the coupling and ships untested code.

**BAD:** Skip the test, ship untested code.
**GOOD:** Refactor the code to be testable (dependency injection, layer isolation).

### NEVER tangle business logic with infrastructure

**WHY:** Mixed logic and infrastructure forces every business rule test to set up SQL, HTTP or filesystems.

**BAD:** `OrderService` contains SQL queries and HTTP response formatting.
**GOOD:** `OrderService` orchestrates pure entities; adapters handle infrastructure.

### NEVER depend on static calls or singletons for behaviour

**WHY:** Statics and singletons are hidden dependencies that need static-mocking tools and shared-state resets in every test.

**BAD:** `InventoryChecker.isAvailable(items)` and `NotificationService.getInstance()` inside the processor.
**GOOD:** Inject an inventory interface and a notification port, and pass instances.

### NEVER read the current time or random numbers directly in logic

**WHY:** Direct access to the clock or randomness makes results non-deterministic, so tests cannot assert exact values.

**BAD:** `order.setProcessedAt(new Date())` inside the service.
**GOOD:** Inject a `Clock` and call `clock.now()`; tests supply a fixed time.

### NEVER leave dependency rules unenforced

**WHY:** Without automated checks, shortcuts such as framework imports in the domain accumulate until refactoring becomes impossible.

**BAD:** Rely on code review to notice `import prisma` in the domain layer.
**GOOD:** Run architecture tests (for example dependency-cruiser or ArchUnit) in CI so violations fail the build.

### NEVER compromise the architecture for testability

**WHY:** Test-only hooks and exposed internals hide the real coupling and leave the design worse than before.

**BAD:** Make a private method public, or add a test flag, so a test can reach it.
**GOOD:** Fix the coupling by extracting a collaborator and injecting it.

## Quick Commands

```bash
# Find hard-to-test code (concrete instantiation)
rg -n "new [A-Z].*Repository\(|new [A-Z].*Service\(|new [A-Z].*Gateway\(" src
```

```bash
# Find side effects in business logic
rg -n "fetch\(|axios\.|fs\.|process\.env" src/domain src/application
```

```bash
# Run tests with coverage
npm run test:coverage
```

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Boundary verification | [references/test-boundary-verification.md](references/test-boundary-verification.md) | Adding architecture tests that enforce dependency rules in CI |
| Testable design | [references/test-testable-design.md](references/test-testable-design.md) | Removing hidden dependencies (statics, singletons, clock) |
| Layer isolation | [references/test-layer-isolation.md](references/test-layer-isolation.md) | Deciding how each layer is tested in isolation |
| Tests are architecture | [references/test-tests-are-architecture.md](references/test-tests-are-architecture.md) | Reviewing test suites that mock internals or sit apart from the design |

### Related Patterns

For dependency inversion (DIP), see solid-principles.
For boundary design, see clean-architecture.
For Humble Object pattern, see design-patterns.

### Further Reading

- [Growing Object-Oriented Software, Guided by Tests](http://www.growing-object-oriented-software.com/)
- [Working Effectively with Legacy Code (Michael Feathers)](https://www.oreilly.com/library/view/working-effectively-with/0131177052/)
- [Test-Driven Development by Example (Kent Beck)](https://www.oreilly.com/library/view/test-driven-development/0321146530/)
