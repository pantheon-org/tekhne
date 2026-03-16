---
name: testable-design
description: Design code for testability using boundary isolation, dependency injection, and testable architecture patterns. Use when designing test strategies, writing unit tests, improving test coverage, refactoring untestable or hard-to-test code, working with mocking or test doubles, or applying TDD practices.
---

# Testable Design

Architecture and design patterns for testable code, focusing on boundary isolation and dependency injection.

## When to Use

- Designing test strategies for new features
- Refactoring untestable code with hard dependencies
- Improving test coverage by isolating layers
- Evaluating whether code is testable before implementation
- Designing boundaries that enable fast unit tests

## When Not to Use

- Class-level SOLID violations (use solid-principles)
- Architectural boundary decisions (use clean-architecture)
- Choosing structural patterns (use design-patterns)

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

**BAD:** new PostgresRepository() in OrderService constructor.  
**GOOD:** Depend on IRepository interface; inject implementation.

### NEVER test implementation details

**BAD:** Test that method X calls method Y internally.  
**GOOD:** Test public behavior: given input, expect output.

### NEVER use real infrastructure in unit tests

**BAD:** Unit test connects to real database.  
**GOOD:** Unit test uses mock repository; integration test uses real database.

### NEVER skip tests because code is "hard to test"

**BAD:** Skip test, ship untested code.  
**GOOD:** Refactor code to be testable (dependency injection, layer isolation).

### NEVER tangle business logic with infrastructure

**BAD:** OrderService contains SQL queries and HTTP response formatting.  
**GOOD:** OrderService orchestrates pure entities; adapters handle infrastructure.

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

### Testable Design Files

- Boundary verification: [references/test-boundary-verification.md](references/test-boundary-verification.md)
- Testable design: [references/test-testable-design.md](references/test-testable-design.md)
- Layer isolation: [references/test-layer-isolation.md](references/test-layer-isolation.md)
- Tests are architecture: [references/test-tests-are-architecture.md](references/test-tests-are-architecture.md)

### Related Patterns

For dependency inversion (DIP), see solid-principles.  
For boundary design, see clean-architecture.  
For Humble Object pattern, see design-patterns.
- [Growing Object-Oriented Software, Guided by Tests](http://www.growing-object-oriented-software.com/)
- [Working Effectively with Legacy Code (Michael Feathers)](https://www.oreilly.com/library/view/working-effectively-with/0131177052/)
- [Test-Driven Development by Example (Kent Beck)](https://www.oreilly.com/library/view/test-driven-development/0321146530/)
