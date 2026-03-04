---
name: clean-architecture
description: Apply Clean Architecture principles to define layer boundaries, identify dependency violations, and structure domain vs infrastructure code. Use when designing service boundaries, separating business logic from infrastructure, evaluating hexagonal/onion/ports-and-adapters architecture, structuring module layout, or resolving dependency inversion and circular dependency issues.
---

# Clean Architecture

Strategic architecture principles for boundaries, dependencies, and layered system design.

## When to Use

- Designing service boundaries or module structure
- Evaluating dependency direction and circular dependencies
- Structuring new applications or refactoring monoliths
- Assessing where to place entities, use cases, adapters, and frameworks
- Deciding when to introduce architectural boundaries

## When Not to Use

- Class-level design decisions (use solid-principles)
- Choosing structural patterns like Factory or Adapter (use design-patterns)
- Testing strategy configuration (use testable-design)

## Architecture Layers

**Dependency Rule:** Dependencies point inward only. Inner layers know nothing about outer layers.

Layers (inner → outer): **Entities** → **Use Cases** → **Interface Adapters** → **Frameworks & Drivers**

See [references/dep-inward-only.md](references/dep-inward-only.md) for layer definitions and examples.

## Workflow

### Step 1: Identify Decision Type

**Output:** Classify as entity, use case, adapter, or framework decision.

Ask:

- Does this logic belong to core business rules? → Entity
- Does it orchestrate business workflows? → Use Case
- Does it translate between layers? → Adapter
- Is it external infrastructure? → Framework

### Step 2: Apply Dependency Direction Checks

**Output:** Dependency violations with corrective actions.

Checklist:

- [ ] Dependencies point inward (no outer layers importing inner layers)
- [ ] No circular dependencies between modules
- [ ] Clear ownership of interfaces (defined by inner layers, implemented by outer layers)
- [ ] Entities remain pure (no framework or infrastructure imports)

**Example:**

```text
Violation: Entity imports ORM decorator from infrastructure.
Refactor: Define entity as plain object; map to ORM in repository (adapter layer).
```

### Step 3: Design Use Cases

**Output:** Use case interface with input/output ports.

Template:

```typescript
// Use Case (application layer)
interface CreateOrderUseCase {
  execute(input: CreateOrderInput): Promise<CreateOrderOutput>
}

// Input/Output ports (defined by use case)
interface CreateOrderInput {
  userId: string
  items: OrderItem[]
}

interface CreateOrderOutput {
  orderId: string
  status: string
}
```

Use cases:

- Have one responsibility (single workflow)
- Depend on abstractions (ports), not concrete implementations
- Orchestrate entities and gateways
- Contain no presentation logic (formatting, HTTP, UI)

### Step 4: Define Boundaries with Adapters

**Output:** Adapter interfaces (ports) and implementations.

- **Controllers:** HTTP requests → use case input
- **Presenters:** Use case output → HTTP response
- **Gateways:** Use case calls → database/API operations
- **Repositories:** Entity persistence → database operations

**Example:**

```typescript
// Port (defined by use case layer)
interface IOrderRepository {
  save(order: Order): Promise<void>
  findById(id: string): Promise<Order | null>
}

// Adapter (infrastructure layer)
class PostgresOrderRepository implements IOrderRepository {
  async save(order: Order): Promise<void> {
    // Map entity to ORM, persist
  }
}
```

### Step 5: Place Framework Code at Edges

**Output:** Framework integrations isolated in outermost layer.

Keep frameworks (Express, NestJS, TypeORM, React) in the infrastructure/adapter layer:

- DI container at application edge
- ORM configurations in infrastructure
- Web server in infrastructure
- Domain remains framework-free

**Example:**

```text
BAD: Entity uses @Entity decorator from TypeORM.
GOOD: Entity is plain TypeScript; repository maps to TypeORM in infrastructure.
```

### Step 6: Document Boundary Decisions

**Output:** ADR with rationale, alternatives, and risks.

Template:

```text
Decision: Extract authentication into separate bounded context.
Rationale: Auth has independent lifecycle and team ownership.
Alternatives: 
  - Keep in monolith (simpler, tightly coupled)
  - Partial boundary (YAGNI, easier to extract later)
Chosen: Full bounded context (clear ownership, independent deployment)
Risks: Network calls add latency; requires distributed transaction handling.
```

## Anti-Patterns

### NEVER allow circular dependencies

**BAD:** Module A imports B and B imports A.  
**GOOD:** Extract shared contract/module and invert dependencies.

### NEVER let entities depend on frameworks

**BAD:** Entity imports ORM decorators, framework types, or infrastructure.  
**GOOD:** Entities are plain objects; adapters handle framework mapping.

### NEVER put business logic in controllers

**BAD:** Controller validates, calculates, and persists data.  
**GOOD:** Controller calls use case; use case orchestrates business logic.

### NEVER bypass interface contracts

**BAD:** Use case instantiates concrete PostgresRepository.  
**GOOD:** Use case depends on IRepository interface; DI provides implementation.

### NEVER design boundaries for imagined future requirements

**BAD:** Add full hexagonal architecture "in case" of future DB migration.  
**GOOD:** Solve current need; refactor when trigger appears (YAGNI).

## Quick Commands

```bash
# Find dependency direction violations
rg -n "import.*infrastructure.*from.*domain|import.*adapter.*from.*entity" src
```

```bash
# Find circular dependencies
nx graph
```

```bash
# Find framework leakage into domain
rg -n "@Entity|@Injectable|@Component" src/domain
```

## Quick Reference

**Dependencies:** [inward-only](references/dep-inward-only.md) · [acyclic](references/dep-acyclic-dependencies.md) · [data crossing boundaries](references/dep-data-crossing-boundaries.md) · [no framework imports](references/dep-no-framework-imports.md)

**Components:** [screaming architecture](references/comp-screaming-architecture.md) · [stable dependencies](references/comp-stable-dependencies.md)

**Boundaries:** [cost awareness](references/bound-boundary-cost-awareness.md) · [defer decisions](references/bound-defer-decisions.md) · [service internal architecture](references/bound-service-internal-architecture.md)

**Entities:** [purity](references/entity-pure-business-rules.md) · [rich not anemic](references/entity-rich-not-anemic.md) · [encapsulate invariants](references/entity-encapsulate-invariants.md) · [value objects](references/entity-value-objects.md) · [no persistence awareness](references/entity-no-persistence-awareness.md)

**Use Cases:** [isolation](references/usecase-single-responsibility.md) · [explicit dependencies](references/usecase-explicit-dependencies.md) · [orchestrates not implements](references/usecase-orchestrates-not-implements.md) · [input/output ports](references/usecase-input-output-ports.md) · [no presentation logic](references/usecase-no-presentation-logic.md) · [transaction boundary](references/usecase-transaction-boundary.md)

**Adapters:** [gateway abstraction](references/adapt-gateway-abstraction.md) · [thin controller](references/adapt-controller-thin.md) · [mapper translation](references/adapt-mapper-translation.md) · [presenter formats](references/adapt-presenter-formats.md)

**Frameworks:** [DI at edge](references/frame-di-container-edge.md) · [domain purity](references/frame-domain-purity.md) · [ORM in infrastructure](references/frame-orm-in-infrastructure.md) · [web in infrastructure](references/frame-web-in-infrastructure.md)

## References

- [Clean Architecture (Uncle Bob)](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Hexagonal Architecture (Alistair Cockburn)](https://alistair.cockburn.us/hexagonal-architecture/)
- [Screaming Architecture](https://blog.cleancoder.com/uncle-bob/2011/09/30/Screaming-Architecture.html)
