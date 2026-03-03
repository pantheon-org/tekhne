---
name: software-design-principles
version: 1.0.0
description: Comprehensive software design guidance covering strategic architecture decisions, tactical implementation principles (SOLID, structural patterns), and code quality fundamentals
keywords:
  - architecture
  - clean architecture
  - clean code
  - SOLID
  - design principles
  - design patterns
  - code quality
  - separation of concerns
  - dependency management
  - software engineering
  - system design
  - code organization
triggers:
  - architecture design
  - system design
  - clean architecture
  - clean code
  - SOLID principles
  - design patterns
  - code review
  - refactoring
  - code quality
  - software design
authors:
  - OpenCode
last_updated: 2026-02-11
---

# Software Design Principles

Comprehensive guidance for designing maintainable, flexible, and robust software systems. This skill covers both strategic architecture decisions and tactical implementation principles.

## When to Use This Skill

- Designing system architecture or planning major system changes
- Making high-level technical decisions about structure and patterns
- Implementing modules, functions, and components with solid design
- Reviewing code for architectural and design quality
- Refactoring existing code to improve design
- Resolving design conflicts or technical debt

## Skill Organization

This skill is organized into three levels of software design:

1. **Strategic Design** - System architecture, boundaries, and long-term structure
2. **Tactical Design** - Code-level principles for implementation (SOLID, structural patterns)
3. **Code Quality** - Clean code fundamentals and best practices

## Rule Classification System

Each principle is categorized by impact level and assigned a structured ID for precise reference:

| Priority | Category | Impact | Rule ID Prefix | Example IDs |
|----------|----------|--------|----------------|-------------|
| 1 | Dependency Rules | CRITICAL | `dep-` | `dep-inward-only`, `dep-acyclic-dependencies` |
| 2 | Entity Design | CRITICAL | `entity-` | `entity-pure-business-rules`, `entity-rich-not-anemic` |
| 3 | Use Case Isolation | HIGH | `usecase-` | `usecase-single-responsibility`, `usecase-explicit-dependencies` |
| 4 | SOLID Principles | HIGH | `solid-` | `solid-srp`, `solid-dip` |
| 5 | Component Cohesion | HIGH | `comp-` | `comp-screaming-architecture`, `comp-common-closure` |
| 6 | Boundary Definition | MEDIUM | `bound-` | `bound-humble-object`, `bound-partial-boundaries` |
| 7 | Framework Isolation | MEDIUM | `frame-` | `frame-domain-purity`, `frame-orm-in-infrastructure` |
| 8 | Interface Adapters | MEDIUM | `adapt-` | `adapt-controller-thin`, `adapt-anti-corruption-layer` |
| 9 | Structural Patterns | MEDIUM | `struct-` | `struct-composition-over-inheritance`, `struct-law-of-demeter` |
| 10 | Core Principles | MEDIUM | `core-` | `core-dry`, `core-kiss`, `core-yagni` |
| 11 | Testing Architecture | LOW | `test-` | `test-boundary-verification`, `test-testable-design` |

## MANDATORY Loading Triggers

⚠️ **CRITICAL RULES** - For these violations, MANDATORY - READ ENTIRE FILE before applying:

### Dependency Direction Violations [CRITICAL]
- **MANDATORY - READ** `references/dep-inward-only.md` when dependency points outward from domain
- **MANDATORY - READ** `references/dep-acyclic-dependencies.md` when circular dependencies detected  
- **MANDATORY - READ** `references/dep-stable-abstractions.md` when concrete depends on volatile
- **MANDATORY - READ** `references/dep-interface-ownership.md` when interface placed in wrong module
- **MANDATORY - READ** `references/dep-no-framework-imports.md` when domain imports framework code

### Entity Design Violations [CRITICAL]
- **MANDATORY - READ** `references/entity-pure-business-rules.md` when entity has infrastructure concerns
- **MANDATORY - READ** `references/entity-rich-not-anemic.md` when entity is data-only with no behavior
- **MANDATORY - READ** `references/entity-encapsulate-invariants.md` when business rules are external
- **MANDATORY - READ** `references/entity-value-objects.md` when primitives used instead of value objects

### 🎯 **HIGH IMPACT RULES** - Load when specific patterns detected:

### Use Case Violations [HIGH]
- **LOAD** `references/usecase-single-responsibility.md` when use case handles multiple concerns
- **LOAD** `references/usecase-explicit-dependencies.md` when hidden dependencies exist
- **LOAD** `references/usecase-orchestrates-not-implements.md` when use case implements business logic

### Component Cohesion Issues [HIGH]
- **LOAD** `references/comp-screaming-architecture.md` when architecture doesn't reveal intent
- **LOAD** `references/comp-common-closure.md` when components change for different reasons

### 📋 **CONTEXT MANAGEMENT** - Do NOT Load Unless Specifically Needed:

**⛔ NEVER load references speculatively - Only load when violation is confirmed:**

#### Do NOT Load by Category:
- **`bound-*` files** - Only for confirmed boundary design violations (not general architecture review)
- **`frame-*` files** - Only when framework coupling is detected (not general framework discussions)  
- **`adapt-*` files** - Only for interface adapter problems (not general interface design)
- **`test-*` files** - Only during testing architecture reviews (not general testing discussions)

#### Context Bloat Prevention Rules:
1. **Load ONE rule at a time** - Don't batch load multiple references
2. **Confirm violation first** - Identify specific rule ID before loading reference
3. **Skip if already familiar** - Don't load references for patterns you already understand well
4. **Use rule ID, not full reference** - Quote rule ID (`dep-inward-only`) instead of loading file for simple violations

#### When to Skip Loading:
- **Routine SOLID violations** - Use rule IDs directly for common SRP/OCP issues
- **Basic DRY/KISS problems** - Apply principles without loading references
- **Simple refactoring tasks** - Load only if complex architectural decisions needed
- **Code review comments** - Use structured output format without loading references

## Output Format

### Standard Violation Format

When identifying violations, use this format: `file:line - [rule-id] Description`

**Basic Examples**:
- `user-service.ts:45 - [solid-srp] Class handles both validation and persistence`
- `order-entity.ts:12 - [entity-pure-business-rules] Entity imports database client`
- `payment-usecase.ts:28 - [dep-inward-only] Use case depends on infrastructure layer`

### Enhanced Reporting Formats

**With Priority Level**:
```
CRITICAL: src/domain/user.ts:23 - [dep-inward-only] Domain layer importing database adapter
HIGH: src/components/UserForm.tsx:67 - [solid-srp] Component handles validation, API calls, and routing
MEDIUM: src/utils/validator.ts:12 - [struct-law-of-demeter] Reaching through multiple object properties
```

**Multi-line Violations**:
```
src/resolvers/user-resolver.ts:45-89 - [solid-srp] GraphQL resolver violates SRP:
  - Line 45-52: Authentication logic
  - Line 53-67: Data validation
  - Line 68-78: Business rules processing
  - Line 79-89: Email notification sending
```

**Batch Report Format**:
```
Design Principle Violations Summary:
================================
Total violations: 12
CRITICAL: 3, HIGH: 5, MEDIUM: 3, LOW: 1

By Category:
- SOLID Principles: 7 violations
- Dependency Rules: 3 violations  
- Structural Principles: 2 violations

Files with violations:
- src/user-service.ts (4 violations)
- src/order-processor.ts (3 violations)
- src/payment-handler.ts (2 violations)
```

**JSON Output Format** (for automated tools):
```json
{
  "violations": [
    {
      "file": "src/user-service.ts",
      "line": 45,
      "rule_id": "solid-srp",
      "priority": "HIGH",
      "description": "Class handles both validation and persistence",
      "category": "SOLID Principles"
    }
  ],
  "summary": {
    "total": 12,
    "by_priority": {"CRITICAL": 3, "HIGH": 5, "MEDIUM": 3, "LOW": 1}
  }
}
```

For detailed explanations of each rule, see the [references/](references/) directory.

---

## Decision Trees for Complex Scenarios

Use these decision trees to navigate complex design situations and determine which principles to apply first:

### 🔄 Primary Decision Tree: Where to Start

```
Is this a NEW system or component?
├── YES → Start with Strategic Design (Part 1)
│   ├── Define boundaries first → Dependency Direction [CRITICAL]
│   ├── Design entities → Entity Design [CRITICAL] 
│   └── Plan use cases → Use Case Isolation [HIGH]
└── NO → Existing system issues
    ├── Performance/coupling problems?
    │   ├── YES → Focus on Dependency Direction [CRITICAL]
    │   └── NO → Continue to behavior analysis
    ├── Business logic scattered?
    │   ├── YES → Focus on Entity Design [CRITICAL] + SRP [HIGH]
    │   └── NO → Continue to code quality check
    └── Hard to test/change?
        ├── YES → Apply SOLID Principles (Part 2) [HIGH]
        └── NO → Focus on Structural Principles (Part 2) [MEDIUM]
```

### 🎯 Code Review Decision Tree

```
What type of violation detected?
├── Imports/Dependencies
│   ├── Framework code in domain → dep-inward-only [CRITICAL]
│   ├── Circular dependencies → dep-acyclic-dependencies [CRITICAL]
│   └── Unstable dependencies → dep-stable-abstractions [HIGH]
├── Class/Component Design
│   ├── Multiple reasons to change → solid-srp [HIGH]
│   ├── Modification for extension → solid-ocp [HIGH]
│   └── Anemic entities → entity-rich-not-anemic [CRITICAL]
├── Interface Problems  
│   ├── Fat interfaces → solid-isp [HIGH]
│   ├── Tight coupling → struct-law-of-demeter [HIGH]
│   └── Inappropriate intimacy → struct-tell-dont-ask [HIGH]
└── Business Logic Issues
    ├── Logic outside entities → entity-pure-business-rules [CRITICAL]
    ├── Invariants not enforced → entity-encapsulate-invariants [CRITICAL]
    └── Mixed concerns → usecase-single-responsibility [HIGH]
```

### 🚨 Crisis Triage Decision Tree

```
System in crisis - what's the biggest problem?
├── Can't deploy/build
│   ├── Circular deps → dep-acyclic-dependencies [CRITICAL] - Fix FIRST
│   └── Framework coupling → dep-inward-only [CRITICAL] - Fix FIRST
├── Can't add features (everything breaks)
│   ├── God classes → solid-srp [HIGH] - Refactor immediately  
│   ├── Tight coupling → solid-ocp + solid-dip [HIGH]
│   └── No tests possible → entity-pure-business-rules [CRITICAL]
├── Performance issues
│   ├── N+1 queries → struct-law-of-demeter [HIGH]
│   ├── Over-fetching → solid-isp [HIGH]
│   └── Inefficient boundaries → bound-humble-object [MEDIUM]
└── Security vulnerabilities
    ├── Data exposure → struct-encapsulation [MEDIUM]
    ├── Input validation missing → entity-encapsulate-invariants [CRITICAL]  
    └── Authorization bypassed → usecase-explicit-dependencies [HIGH]
```

### 💡 Refactoring Strategy Decision Tree

```
Planning refactoring - what's the approach?
├── Big Bang (complete rewrite)
│   ├── Start: Strategic Design principles [CRITICAL]
│   ├── Then: Entity Design [CRITICAL] 
│   └── Finally: Tactical principles [HIGH-MEDIUM]
├── Strangler Fig (gradual replacement)
│   ├── New boundaries → bound-anti-corruption-layer [MEDIUM]
│   ├── Interface design → solid-isp + solid-dip [HIGH]
│   └── Migration safety → usecase-transaction-boundary [HIGH] 
├── Extract Service (microservice extraction)
│   ├── Define boundary → usecase-single-responsibility [HIGH]
│   ├── Clean interfaces → solid-isp [HIGH]
│   └── Data consistency → usecase-transaction-boundary [HIGH]
└── Legacy Improvement (incremental fixes)
    ├── Boy Scout Rule → Fix one violation per change
    ├── Priority order → CRITICAL → HIGH → MEDIUM → LOW
    └── Focus areas → Most changed files first
```

---

## Part 1: Strategic Design (System Architecture)

Strategic design focuses on system structure, boundaries, and long-term architectural decisions. Use these principles when designing systems, defining boundaries, or planning major changes.

### Clean Architecture Principles

Based on Robert C. Martin's Clean Architecture. The core idea: **dependencies point inward only**.

#### 1. Dependency Direction [CRITICAL]

**Rule IDs**: `dep-inward-only`, `dep-acyclic-dependencies`, `dep-stable-abstractions`, `dep-interface-ownership`, `dep-no-framework-imports`, `dep-data-crossing-boundaries`

⚠️ **BEFORE APPLYING** - Read references for specific violations:
- **Before applying** `dep-inward-only` → **READ** `references/dep-inward-only.md`
- **Before applying** `dep-acyclic-dependencies` → **READ** `references/dep-acyclic-dependencies.md`  
- **Before applying** `dep-stable-abstractions` → **READ** `references/dep-stable-abstractions.md`
- **Before applying** `dep-interface-ownership` → **READ** `references/dep-interface-ownership.md`
- **Before applying** `dep-no-framework-imports` → **READ** `references/dep-no-framework-imports.md`

**Rule**: Dependencies must point inward toward business logic. Outer layers depend on inner layers, never the reverse.

```
┌─────────────────────────────────────┐
│   Frameworks & Drivers (UI, DB)     │  ← Outer (least stable)
├─────────────────────────────────────┤
│   Interface Adapters (Controllers)  │
├─────────────────────────────────────┤
│   Use Cases (Application Logic)     │
├─────────────────────────────────────┤
│   Entities (Business Rules)         │  ← Inner (most stable)
└─────────────────────────────────────┘

Dependencies point: Outer → Inner ONLY
```

**Rules**:
- Entities depend on nothing
- Use cases depend only on entities
- Interface adapters depend on use cases and entities
- Frameworks depend on interface adapters
- No framework imports in inner layers
- Pass simple data structures across boundaries (DTOs)
- Maintain acyclic dependencies
- Stable abstractions principle (abstract things are stable)

**⛔ NEVER DO - Dependency Direction Anti-Patterns**:

1. **NEVER import framework code in domain entities** because it couples business rules to volatile implementation details and makes testing impossible
2. **NEVER let entities depend on use cases** because entities are the stable core and use cases are more changeable application concerns  
3. **NEVER import database ORMs in use cases** because it violates the dependency rule and makes business logic depend on data access details
4. **NEVER pass entity objects directly to UI components** because it exposes internal business logic to presentation concerns and creates coupling
5. **NEVER let inner layers know about outer layer implementations** because it breaks the dependency rule and creates fragile architecture
6. **NEVER create circular dependencies between layers** because it makes the system impossible to understand, test, and maintain
7. **NEVER import HTTP libraries in business logic** because business rules should be independent of delivery mechanisms
8. **NEVER let domain services depend on infrastructure services** because domain logic must remain pure and infrastructure-agnostic

**Example: Dependency Inversion at Boundary**

```typescript
// ❌ BAD: Use case depends on database implementation
export const getUserUseCase = (userId: string) => {
  const db = new PostgresDatabase(); // Direct dependency on infrastructure
  return db.query(`SELECT * FROM users WHERE id = ${userId}`);
};

// ✅ GOOD: Use case depends on abstraction (interface)
// Inner layer defines the interface
interface UserRepository {
  findById(id: string): Promise<User | null>;
}

// Use case depends on abstraction
export const getUserUseCase = (
  userId: string,
  userRepo: UserRepository // Injected dependency
): Promise<User | null> => {
  return userRepo.findById(userId);
};

// Outer layer implements the interface
class PostgresUserRepository implements UserRepository {
  async findById(id: string): Promise<User | null> {
    // Database implementation details
    return this.db.query('SELECT * FROM users WHERE id = $1', [id]);
  }
}
```

**Interface Ownership Rule**: Inner layers define the interfaces they need. Outer layers implement them.

#### 2. Entity Design [CRITICAL]

**Rule IDs**: `entity-pure-business-rules`, `entity-rich-not-anemic`, `entity-no-persistence-awareness`, `entity-encapsulate-invariants`, `entity-value-objects`

⚠️ **BEFORE APPLYING** - Read references for specific violations:
- **Before applying** `entity-pure-business-rules` → **READ** `references/entity-pure-business-rules.md`
- **Before applying** `entity-rich-not-anemic` → **READ** `references/entity-rich-not-anemic.md`
- **Before applying** `entity-encapsulate-invariants` → **READ** `references/entity-encapsulate-invariants.md`
- **Before applying** `entity-value-objects` → **READ** `references/entity-value-objects.md`

**Entities** contain pure business rules and critical business data. They are the heart of your system.

**Rules**:
- **Pure business rules** - No persistence, UI, or framework concerns
- **No persistence awareness** - Entities don't know about databases, ORMs, or storage
- **Encapsulate invariants** - Enforce business rules within the entity
- **Value objects** - Immutable objects representing concepts (Money, Email, Address)
- **Rich domain models** - Behavior + data, not anemic POJOs

**⛔ NEVER DO - Entity Design Anti-Patterns**:

1. **NEVER create anemic entities with only getters/setters** because it scatters business logic across services and violates encapsulation
2. **NEVER import database annotations in domain entities** because entities should be persistence-ignorant and focused on business rules
3. **NEVER let entities know about HTTP requests or responses** because domain logic must be independent of delivery mechanisms
4. **NEVER expose entity internals through public fields** because it breaks encapsulation and allows invalid state changes
5. **NEVER create entities that violate their own business invariants** because entities must always maintain valid business state
6. **NEVER mix persistence logic with business logic in entities** because it violates single responsibility and makes testing difficult
7. **NEVER create god entities that know about everything** because large entities become unmaintainable and violate cohesion
8. **NEVER use primitive types for domain concepts** because value objects provide better type safety and express business meaning

**Example: Rich Entity**

```typescript
// ❌ BAD: Anemic entity (just data)
interface Order {
  id: string;
  items: OrderItem[];
  status: string;
  total: number;
}

// Business logic scattered in services
const canCancelOrder = (order: Order): boolean => {
  return order.status === 'pending' || order.status === 'confirmed';
};

// ✅ GOOD: Rich entity (data + behavior)
class Order {
  private constructor(
    private id: string,
    private items: OrderItem[],
    private status: OrderStatus,
    private total: Money
  ) {}

  // Factory method enforces invariants
  static create(items: OrderItem[]): Order {
    if (items.length === 0) {
      throw new Error('Order must have at least one item');
    }
    const total = items.reduce((sum, item) => sum.add(item.price), Money.zero());
    return new Order(generateId(), items, OrderStatus.Pending, total);
  }

  // Business rules encapsulated in entity
  canCancel(): boolean {
    return this.status === OrderStatus.Pending || this.status === OrderStatus.Confirmed;
  }

  cancel(): void {
    if (!this.canCancel()) {
      throw new Error(`Cannot cancel order in ${this.status} status`);
    }
    this.status = OrderStatus.Cancelled;
  }

  getTotal(): Money {
    return this.total;
  }
}

// Value object
class Money {
  private constructor(private amount: number, private currency: string) {}

  static zero(): Money {
    return new Money(0, 'USD');
  }

  add(other: Money): Money {
    if (this.currency !== other.currency) {
      throw new Error('Cannot add money with different currencies');
    }
    return new Money(this.amount + other.amount, this.currency);
  }
}
```

#### 3. Use Case Isolation [HIGH]

**Rule IDs**: `usecase-single-responsibility`, `usecase-explicit-dependencies`, `usecase-orchestrates-not-implements`, `usecase-transaction-boundary`, `usecase-no-presentation-logic`

🎯 **BEFORE APPLYING** - Read references for specific violations:
- **Before applying** `usecase-single-responsibility` → **READ** `references/usecase-single-responsibility.md`
- **Before applying** `usecase-explicit-dependencies` → **READ** `references/usecase-explicit-dependencies.md`
- **Before applying** `usecase-orchestrates-not-implements` → **READ** `references/usecase-orchestrates-not-implements.md`

**Use Cases** orchestrate application-specific business rules. Each use case represents one thing the system does.

**Rules**:
- **Single responsibility** - One use case per user action/system operation
- **Input/Output ports** - Define interfaces for what comes in and goes out
- **Orchestrates, doesn't implement** - Delegates work to entities and repositories
- **No presentation logic** - Don't format responses for UI
- **Explicit dependencies** - All dependencies injected via constructor
- **Transaction boundary** - Use case defines transactional boundaries

**Example: Use Case Pattern**

```typescript
// Input port (request)
interface CreateOrderRequest {
  userId: string;
  items: { productId: string; quantity: number }[];
}

// Output port (response)
interface CreateOrderResponse {
  orderId: string;
  total: number;
  currency: string;
}

// Dependencies (interfaces defined by use case)
interface OrderRepository {
  save(order: Order): Promise<void>;
}

interface ProductRepository {
  findById(id: string): Promise<Product | null>;
}

// Use case orchestrates the operation
export const createOrderUseCase = async (
  request: CreateOrderRequest,
  orderRepo: OrderRepository,
  productRepo: ProductRepository
): Promise<CreateOrderResponse> => {
  // 1. Load products
  const products = await Promise.all(
    request.items.map(item => productRepo.findById(item.productId))
  );

  // 2. Create order items (delegate to entity)
  const orderItems = products.map((product, idx) => 
    OrderItem.create(product, request.items[idx].quantity)
  );

  // 3. Create order entity (business rules enforced here)
  const order = Order.create(request.userId, orderItems);

  // 4. Persist
  await orderRepo.save(order);

  // 5. Return response
  return {
    orderId: order.getId(),
    total: order.getTotal().getAmount(),
    currency: order.getTotal().getCurrency()
  };
};
```

#### 4. Architecture Decision Making [HIGH]

When designing systems, follow this process:

**1. Understand Context**
- Business goals and constraints
- Technical constraints (performance, scale, security)
- Team expertise and organizational structure
- Existing systems and integration points

**2. Gather Requirements**
- Functional requirements (what the system must do)
- Non-functional requirements (performance, scalability, security, maintainability)
- Quality attributes (availability, reliability, usability)

**3. Identify Constraints**
- Time and budget
- Technology stack
- Regulatory compliance
- Team size and skills

**4. Consider Alternatives**
- Evaluate multiple approaches
- Document trade-offs for each option
- Use Architecture Decision Records (ADRs)

**5. Design System Structure**
- Define major components and their responsibilities
- Establish boundaries and interfaces
- Plan for failure and recovery
- Consider testing strategy

**6. Document Decisions**
- Use ADRs for significant decisions
- Explain context, decision, and consequences
- Keep documentation close to code

**Architecture Decision Record Template**:

```markdown
# ADR-001: Use Clean Architecture for Core Domain

## Status
Accepted

## Context
We need to design the order management system with long-term maintainability. 
The domain logic is complex and will evolve frequently. We need to isolate 
business rules from infrastructure changes.

## Decision
We will use Clean Architecture with clear boundaries between entities, use 
cases, and infrastructure. Dependencies will point inward toward domain logic.

## Consequences
**Positive**:
- Business logic testable without infrastructure
- Easy to swap databases or frameworks
- Clear separation of concerns

**Negative**:
- More initial boilerplate
- Learning curve for team
- More files and indirection

## Alternatives Considered
- Traditional layered architecture (rejected: too much coupling)
- Microservices (rejected: premature for our scale)
```

#### 5. Common Architecture Patterns [MEDIUM]

**Layered Architecture**
```
┌─────────────────┐
│  Presentation   │
├─────────────────┤
│    Business     │
├─────────────────┤
│  Persistence    │
└─────────────────┘
```
- Simple, widely understood
- Each layer depends on layer below
- Risk: business logic leaks into presentation

**Hexagonal Architecture (Ports & Adapters)**
```
        ┌─────────────────┐
        │   HTTP API      │ ← Adapter
        └────────┬────────┘
                 │
    ┌────────────▼────────────┐
    │    Application Core     │ ← Port (interface)
    │   (Business Logic)      │
    └────────────┬────────────┘
                 │
        ┌────────▼────────┐
        │    Database     │ ← Adapter
        └─────────────────┘
```
- Business logic at center
- Adapters on outside connect to infrastructure
- Easy to test (mock adapters)

**Event-Driven Architecture**
```
Service A  ──→  Event Bus  ──→  Service B
                    │
                    └──────────→  Service C
```
- Decoupled services communicate via events
- Scalable and flexible
- Complexity in eventual consistency

**Choose based on**:
- System complexity
- Team size and expertise
- Scalability requirements
- Need for flexibility

#### 6. Architecture Anti-Patterns [HIGH]

**Premature Optimization**
- Designing for scale before you need it
- Over-engineering simple problems
- Solution: YAGNI - solve today's problem well

**Resume-Driven Architecture**
- Using technology because it's trendy
- Ignoring team expertise and context
- Solution: Choose boring technology that fits

**Distributed Monolith**
- Microservices with tight coupling
- Shared databases across services
- Solution: Ensure true service boundaries

**Big Ball of Mud**
- No clear structure or boundaries
- Everything depends on everything
- Solution: Introduce layers and enforce boundaries gradually

**Analysis Paralysis**
- Endless planning without building
- Trying to make perfect decisions upfront
- Solution: Make reversible decisions quickly, use ADRs, iterate

---

## Part 2: Tactical Design (Implementation Principles)

Tactical design focuses on code-level decisions: how to structure modules, classes, and functions for maintainability and flexibility.

### SOLID Principles

The five fundamental principles for object-oriented and functional design. These manage dependencies and responsibilities at the code level.

#### 1. Single Responsibility Principle (SRP) [HIGH]

**Rule ID**: `solid-srp`

🎯 **BEFORE APPLYING** - For SRP violations: **READ** `references/solid-srp.md`

**Definition**: A module should have one, and only one, reason to change.

**In Practice**: Each function/class should do one thing and do it well. If you need to change a module for multiple different reasons, it has too many responsibilities.

**Example: TypeScript**

```typescript
// ❌ BAD: Multiple responsibilities
class UserService {
  createUser(data: UserData) {
    // 1. Validate data
    if (!data.email.includes('@')) throw new Error('Invalid email');
    
    // 2. Hash password
    const hashedPassword = bcrypt.hash(data.password);
    
    // 3. Save to database
    db.users.insert({ ...data, password: hashedPassword });
    
    // 4. Send welcome email
    emailService.send(data.email, 'Welcome!');
    
    // 5. Log analytics
    analytics.track('user_created', data.email);
  }
}
// This class has 5 reasons to change!

// ✅ GOOD: Single responsibility per module
const validateUserData = (data: UserData): void => {
  if (!data.email.includes('@')) throw new Error('Invalid email');
};

const hashPassword = (password: string): string => {
  return bcrypt.hash(password);
};

const saveUser = (user: User): Promise<void> => {
  return db.users.insert(user);
};

const sendWelcomeEmail = (email: string): void => {
  emailService.send(email, 'Welcome!');
};

const trackUserCreation = (email: string): void => {
  analytics.track('user_created', email);
};

// Orchestrator composes single-responsibility functions
const createUser = async (data: UserData): Promise<void> => {
  validateUserData(data);
  const hashedPassword = hashPassword(data.password);
  const user = { ...data, password: hashedPassword };
  await saveUser(user);
  sendWelcomeEmail(user.email);
  trackUserCreation(user.email);
};
```

**Example: Elixir**

```elixir
# ❌ BAD: God module with multiple responsibilities
defmodule UserService do
  def create_user(data) do
    with :ok <- validate_email(data.email),
         {:ok, hash} <- hash_password(data.password),
         {:ok, user} <- insert_user(data, hash),
         :ok <- send_email(user.email),
         :ok <- track_event(user.email) do
      {:ok, user}
    end
  end
  
  defp validate_email(email), do: # validation logic
  defp hash_password(password), do: # hashing logic
  defp insert_user(data, hash), do: # database logic
  defp send_email(email), do: # email logic
  defp track_event(email), do: # analytics logic
end

# ✅ GOOD: Each module has one responsibility
defmodule UserValidator do
  def validate(data) do
    if String.contains?(data.email, "@"), do: :ok, else: {:error, :invalid_email}
  end
end

defmodule PaymentProcessor do
  def process(method, amount), do: PaymentMethod.process(method, amount)
end
```

**Real-World Violation Examples:**

```typescript
// ❌ COMMON VIOLATION: GraphQL resolver doing too much
class UserResolver {
  async createUser(args: CreateUserArgs, context: Context) {
    // Validation logic (should be separate)
    if (!args.email || !args.email.includes('@')) {
      throw new Error('Invalid email');
    }
    if (!args.password || args.password.length < 8) {
      throw new Error('Password too short');
    }
    if (!args.name || args.name.trim().length === 0) {
      throw new Error('Name required');
    }
    
    // Business logic (should be separate)  
    const existingUser = await this.userService.findByEmail(args.email);
    if (existingUser) {
      throw new Error('User already exists');
    }
    
    // Encryption logic (should be separate)
    const salt = await bcrypt.genSalt(10);
    const hashedPassword = await bcrypt.hash(args.password, salt);
    
    // Database logic (should be separate)
    const user = await this.userService.create({
      ...args,
      password: hashedPassword,
      createdAt: new Date(),
      lastLogin: null
    });
    
    // Email logic (should be separate)
    const emailTemplate = await this.loadTemplate('welcome');
    const personalizedEmail = emailTemplate.replace('{{name}}', user.name);
    await this.emailService.send({
      to: user.email,
      subject: 'Welcome to our platform!',
      html: personalizedEmail
    });
    
    // Analytics logic (should be separate)
    await this.analytics.track('user_created', {
      userId: user.id,
      email: user.email,
      source: context.source || 'direct'
    });
    
    // Audit logging (should be separate)
    await this.auditLogger.log({
      action: 'USER_CREATED',
      userId: user.id,
      adminId: context.adminId,
      timestamp: new Date(),
      metadata: { email: user.email }
    });
    
    return user;
  }
  
  private loadTemplate(name: string) { /* template loading logic */ }
}

// ❌ COMMON VIOLATION: React component doing everything
const UserDashboard = ({ userId }: { userId: string }) => {
  const [user, setUser] = useState(null);
  const [posts, setPosts] = useState([]);
  const [analytics, setAnalytics] = useState(null);
  const [notifications, setNotifications] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  useEffect(() => {
    // Data fetching logic (should be custom hook or service)
    const fetchUserData = async () => {
      try {
        setLoading(true);
        const [userData, userPosts, userAnalytics, userNotifications] = 
          await Promise.all([
            api.get(`/users/${userId}`),
            api.get(`/users/${userId}/posts`),
            api.get(`/users/${userId}/analytics`),
            api.get(`/users/${userId}/notifications`)
          ]);
        
        setUser(userData);
        setPosts(userPosts);
        setAnalytics(userAnalytics);
        setNotifications(userNotifications);
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };
    
    fetchUserData();
  }, [userId]);
  
  // Validation logic (should be separate)
  const validatePost = (content: string): boolean => {
    if (!content || content.trim().length === 0) return false;
    if (content.length > 280) return false;
    if (content.includes('<script>')) return false;
    return true;
  };
  
  // Business logic (should be separate)
  const handleCreatePost = async (content: string) => {
    if (!validatePost(content)) {
      alert('Invalid post content');
      return;
    }
    
    try {
      const post = await api.post('/posts', { content, userId });
      setPosts([post, ...posts]);
      
      // Analytics logic (should be separate)
      analytics.track('post_created', { userId, postLength: content.length });
      
      // Notification logic (should be separate)  
      if (user.followers?.length > 100) {
        await api.post('/notifications/broadcast', {
          type: 'new_post',
          userId,
          postId: post.id
        });
      }
    } catch (err) {
      alert('Failed to create post');
    }
  };
  
  // UI logic mixed with business logic
  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;
  if (!user) return <div>User not found</div>;
  
  return (
    <div>
      {/* User profile rendering */}
      <div>{user.name}</div>
      
      {/* Posts rendering */}
      <div>{posts.map(post => <div key={post.id}>{post.content}</div>)}</div>
      
      {/* Analytics rendering */}
      <div>Views: {analytics.views}, Likes: {analytics.likes}</div>
      
      {/* Notifications rendering */}
      <div>{notifications.map(n => <div key={n.id}>{n.message}</div>)}</div>
    </div>
  );
};
```

**Impact of These Violations:**
- Hard to test (too many responsibilities in one place)
- Hard to debug (errors could come from any of the mixed concerns)
- Hard to maintain (changes to one concern affect others)
- Hard to reuse (tightly coupled logic)
- Violates separation of concerns fundamentally

**Benefits**:
- Predictable behavior when using polymorphism
- Reduces bugs from unexpected subtype behavior
- Enables safe refactoring

**⛔ NEVER DO - Single Responsibility Anti-Patterns**:

1. **NEVER create classes that have multiple reasons to change** because it makes the code fragile and unpredictable when requirements evolve
2. **NEVER mix business logic with presentation concerns** because business rules should be independent of how they're displayed
3. **NEVER combine validation, persistence, and business logic in one class** because each has different change drivers and should be isolated
4. **NEVER create god classes that handle everything** because they become unmaintainable, untestable, and violate cohesion principles
5. **NEVER let a single function perform multiple unrelated operations** because it reduces reusability and makes testing complex
6. **NEVER mix database access with HTTP handling** because data access and web concerns change for different reasons
7. **NEVER combine synchronous business logic with asynchronous I/O operations** because it makes testing and error handling complicated
8. **NEVER create utility classes that do too many different things** because utilities should be focused and easily discoverable

#### 4. Interface Segregation Principle (ISP) [HIGH]

**Rule ID**: `solid-isp`

**Definition**: Clients should not be forced to depend on interfaces they don't use.

**In Practice**: Create small, focused interfaces rather than large, monolithic ones. Each client should only know about the methods it needs.

**Example: TypeScript**

```typescript
// ❌ BAD: Fat interface with too many methods
interface Worker {
  work(): void;
  eat(): void;
  sleep(): void;
  generateReport(): void;
  attendMeeting(): void;
}

class HumanWorker implements Worker {
  work() { /* ... */ }
  eat() { /* ... */ }
  sleep() { /* ... */ }
  generateReport() { /* ... */ }
  attendMeeting() { /* ... */ }
}

class RobotWorker implements Worker {
  work() { /* ... */ }
  eat() { throw new Error('Robots do not eat!'); } // Forced to implement!
  sleep() { throw new Error('Robots do not sleep!'); } // Forced to implement!
  generateReport() { /* ... */ }
  attendMeeting() { /* ... */ }
}

// ✅ GOOD: Segregated interfaces
interface Workable {
  work(): void;
}

interface Feedable {
  eat(): void;
}

interface Sleepable {
  sleep(): void;
}

interface Reportable {
  generateReport(): void;
}

interface Attendable {
  attendMeeting(): void;
}

class HumanWorker implements Workable, Feedable, Sleepable, Reportable, Attendable {
  work() { /* ... */ }
  eat() { /* ... */ }
  sleep() { /* ... */ }
  generateReport() { /* ... */ }
  attendMeeting() { /* ... */ }
}

class RobotWorker implements Workable, Reportable, Attendable {
  work() { /* ... */ }
  generateReport() { /* ... */ }
  attendMeeting() { /* ... */ }
  // No need to implement eat() or sleep()!
}

// Functions depend only on what they need
const performWork = (worker: Workable) => {
  worker.work();
};

const feedWorker = (worker: Feedable) => {
  worker.eat();
};
```

**Example: Elixir**

```elixir
# ❌ BAD: Single behavior with too many callbacks
defmodule Worker do
  @callback work() :: :ok
  @callback eat() :: :ok
  @callback sleep() :: :ok
  @callback generate_report() :: :ok
  @callback attend_meeting() :: :ok
end

defmodule RobotWorker do
  @behaviour Worker
  
  def work, do: :ok
  def eat, do: raise "Robots do not eat!" # Forced to implement!
  def sleep, do: raise "Robots do not sleep!" # Forced to implement!
  def generate_report, do: :ok
  def attend_meeting, do: :ok
end

# ✅ GOOD: Multiple focused behaviors
defmodule Workable do
  @callback work() :: :ok
end

defmodule Feedable do
  @callback eat() :: :ok
end

defmodule Sleepable do
  @callback sleep() :: :ok
end

defmodule Reportable do
  @callback generate_report() :: :ok
end

defmodule Attendable do
  @callback attend_meeting() :: :ok
end

defmodule HumanWorker do
  @behaviour Workable
  @behaviour Feedable
  @behaviour Sleepable
  @behaviour Reportable
  @behaviour Attendable
  
  def work, do: :ok
  def eat, do: :ok
  def sleep, do: :ok
  def generate_report, do: :ok
  def attend_meeting, do: :ok
end

defmodule RobotWorker do
  @behaviour Workable
  @behaviour Reportable
  @behaviour Attendable
  
  def work, do: :ok
  def generate_report, do: :ok
  def attend_meeting, do: :ok
  # No need to implement eat() or sleep()!
end
```

**Benefits**:
- Clients only depend on methods they use
- Reduces coupling between components
- Easier to implement interfaces (fewer methods)
- More flexible and composable designs

#### 5. Dependency Inversion Principle (DIP) [HIGH]

**Rule ID**: `solid-dip`

**Definition**: High-level modules should not depend on low-level modules. Both should depend on abstractions.

**In Practice**: Don't create dependencies directly in your code. Inject dependencies (interfaces) from the outside. This is the foundation of Clean Architecture.

**Example: TypeScript**

```typescript
// ❌ BAD: High-level module depends on low-level module
class UserService {
  private db = new PostgresDatabase(); // Direct dependency!

  async getUser(id: string): Promise<User> {
    return this.db.query(`SELECT * FROM users WHERE id = ${id}`);
  }
}
// Cannot test without real database
// Cannot swap database implementation

// ✅ GOOD: Depend on abstraction, inject dependency
interface UserRepository {
  findById(id: string): Promise<User | null>;
}

class UserService {
  constructor(private userRepo: UserRepository) {} // Injected abstraction

  async getUser(id: string): Promise<User> {
    const user = await this.userRepo.findById(id);
    if (!user) throw new Error('User not found');
    return user;
  }
}

// Low-level module implements abstraction
class PostgresUserRepository implements UserRepository {
  constructor(private db: PostgresDatabase) {}

  async findById(id: string): Promise<User | null> {
    return this.db.query(`SELECT * FROM users WHERE id = $1`, [id]);
  }
}

// Composition root wires dependencies
const db = new PostgresDatabase();
const userRepo = new PostgresUserRepository(db);
const userService = new UserService(userRepo);

// Easy to test with mock
class MockUserRepository implements UserRepository {
  async findById(id: string): Promise<User | null> {
    return { id, name: 'Test User', email: 'test@example.com' };
  }
}

const testUserService = new UserService(new MockUserRepository());
```

**Example: Elixir**

```elixir
# ❌ BAD: Direct dependency on implementation
defmodule UserService do
  def get_user(id) do
    # Directly calls Postgres module
    PostgresRepo.find_user(id)
  end
end

# ✅ GOOD: Depend on behavior, inject implementation
defmodule UserRepository do
  @callback find_by_id(id :: String.t()) :: {:ok, User.t()} | {:error, term()}
end

defmodule UserService do
  def get_user(id, repo \\ PostgresUserRepo) do
    case repo.find_by_id(id) do
      {:ok, user} -> {:ok, user}
      {:error, _} -> {:error, :not_found}
    end
  end
end

# Implementation
defmodule PostgresUserRepo do
  @behaviour UserRepository
  
  def find_by_id(id) do
    Repo.get(User, id)
    |> case do
      nil -> {:error, :not_found}
      user -> {:ok, user}
    end
  end
end

# Easy to test with mock
defmodule MockUserRepo do
  @behaviour UserRepository
  
  def find_by_id(id) do
    {:ok, %User{id: id, name: "Test User"}}
  end
end

# In tests
UserService.get_user("123", MockUserRepo)
```

**Benefits**:
- High-level logic doesn't depend on implementation details
- Easy to test (inject mocks)
- Easy to swap implementations (different databases, APIs, etc.)
- Follows Clean Architecture dependency rule

---

### Structural Design Principles

These principles transcend programming paradigms and apply to both object-oriented and functional code.

#### 1. Composition Over Inheritance [MEDIUM]

**Rule ID**: `struct-composition-over-inheritance`

**Principle**: Favor composing behaviors from small, focused pieces over building class hierarchies.

**Why**: Inheritance creates tight coupling and rigid hierarchies. Composition is flexible and composable.

**Example: TypeScript**

```typescript
// ❌ BAD: Deep inheritance hierarchy
class Animal {
  move() { console.log('Moving...'); }
}

class Mammal extends Animal {
  breathe() { console.log('Breathing...'); }
}

class Dog extends Mammal {
  bark() { console.log('Barking...'); }
}

class Cat extends Mammal {
  meow() { console.log('Meowing...'); }
}
// What if we need a flying dog? Multiple inheritance not supported!
// Hierarchy becomes rigid and hard to change

// ✅ GOOD: Composition with interfaces
interface Movable {
  move(): void;
}

interface Breathable {
  breathe(): void;
}

interface Barkable {
  bark(): void;
}

interface Flyable {
  fly(): void;
}

const createMovable = (): Movable => ({
  move: () => console.log('Moving...')
});

const createBreathable = (): Breathable => ({
  breathe: () => console.log('Breathing...')
});

const createBarkable = (): Barkable => ({
  bark: () => console.log('Barking...')
});

const createFlyable = (): Flyable => ({
  fly: () => console.log('Flying...')
});

// Compose behaviors as needed
type Dog = Movable & Breathable & Barkable;
type FlyingDog = Movable & Breathable & Barkable & Flyable;

const createDog = (): Dog => ({
  ...createMovable(),
  ...createBreathable(),
  ...createBarkable()
});

const createFlyingDog = (): FlyingDog => ({
  ...createMovable(),
  ...createBreathable(),
  ...createBarkable(),
  ...createFlyable()
});
```

**Example: Elixir**

```elixir
# ❌ BAD: Trying to use inheritance patterns (modules as base classes)
defmodule Animal do
  def move, do: IO.puts("Moving...")
end

defmodule Mammal do
  import Animal
  def breathe, do: IO.puts("Breathing...")
end

defmodule Dog do
  import Mammal
  def bark, do: IO.puts("Barking...")
end
# Becomes hard to manage and compose

# ✅ GOOD: Composition with pipes and small functions
defmodule Movable do
  def move(entity), do: %{entity | status: "moving"}
end

defmodule Breathable do
  def breathe(entity), do: %{entity | oxygen: entity.oxygen + 1}
end

defmodule Barkable do
  def bark(entity) do
    IO.puts("Woof!")
    entity
  end
end

defmodule Flyable do
  def fly(entity), do: %{entity | altitude: entity.altitude + 10}
end

# Compose behaviors with pipes
defmodule Dog do
  def create(name) do
    %{name: name, status: "idle", oxygen: 100}
  end
  
  def act(dog) do
    dog
    |> Movable.move()
    |> Breathable.breathe()
    |> Barkable.bark()
  end
end

defmodule FlyingDog do
  def create(name) do
    %{name: name, status: "idle", oxygen: 100, altitude: 0}
  end
  
  def act(flying_dog) do
    flying_dog
    |> Movable.move()
    |> Breathable.breathe()
    |> Barkable.bark()
    |> Flyable.fly()
  end
end
```

**Advanced Elixir Example: Payment Processing Pipeline**

```elixir
# ✅ EXCELLENT: Complex composition with pipes, protocols, and behaviours

# Protocol for payment processing polymorphism
defprotocol PaymentProcessor do
  def process(payment_method, amount, metadata)
end

# Behaviour for validation steps
defmodule PaymentValidator do
  @callback validate(payment_data :: map()) :: {:ok, map()} | {:error, String.t()}
end

# Individual processors implementing protocol
defimpl PaymentProcessor, for: Map do
  def process(%{type: "credit_card"} = method, amount, metadata) do
    CreditCardProcessor.process(method, amount, metadata)
  end
  
  def process(%{type: "paypal"} = method, amount, metadata) do
    PayPalProcessor.process(method, amount, metadata)
  end
end

# Composable validation modules
defmodule AmountValidator do
  @behaviour PaymentValidator
  def validate(%{amount: amount}) when amount > 0, do: {:ok, %{amount: amount}}
  def validate(_), do: {:error, "Invalid amount"}
end

defmodule PaymentMethodValidator do
  @behaviour PaymentValidator
  def validate(%{method: %{type: type}} = data) when type in ["credit_card", "paypal"] do
    {:ok, data}
  end
  def validate(_), do: {:error, "Unsupported payment method"}
end

# Main payment pipeline using composition
defmodule PaymentPipeline do
  def process(payment_data) do
    with {:ok, validated_data} <- validate_payment(payment_data),
         {:ok, processed_data} <- process_payment(validated_data),
         {:ok, confirmed_data} <- confirm_payment(processed_data) do
      {:ok, confirmed_data}
    else
      {:error, reason} -> {:error, reason}
    end
  end
  
  # Compose validation steps with pipes
  defp validate_payment(data) do
    data
    |> AmountValidator.validate()
    |> case do
      {:ok, validated_data} -> PaymentMethodValidator.validate(validated_data)
      error -> error
    end
  end
  
  defp process_payment(%{method: method, amount: amount} = data) do
    result = PaymentProcessor.process(method, amount, %{transaction_id: generate_id()})
    {:ok, Map.put(data, :processing_result, result)}
  end
  
  defp confirm_payment(data) do
    # Additional composition with audit logging, notifications, etc.
    data
    |> AuditLogger.log()
    |> NotificationService.notify()
    |> PersistenceLayer.save()
  end
  
  defp generate_id, do: :crypto.strong_rand_bytes(16) |> Base.encode64()
end

# Usage demonstrates flexible composition
payment = %{
  method: %{type: "credit_card", number: "****1234"},
  amount: 100.00,
  customer: %{id: 123, name: "Alice"}
}

{:ok, result} = PaymentPipeline.process(payment)
```

**Benefits**:
- Flexibility: Mix and match behaviors as needed
- No rigid hierarchies: Easy to add new combinations
- Better code reuse: Small, focused pieces
- Easier to test: Test individual behaviors in isolation
- **Protocol polymorphism**: Different payment methods handled uniformly
- **Behaviour contracts**: Validation steps follow consistent interface
- **Pipeline composition**: Complex workflows built from simple steps

#### 2. Law of Demeter (Principle of Least Knowledge) [HIGH]

**Rule ID**: `struct-law-of-demeter`

**Principle**: A module should only talk to its immediate friends, not strangers.

**Rule**: Only call methods on:
1. The current object (`this`)
2. Objects passed as parameters
3. Objects created locally
4. Direct properties/fields

**Don't**: Chain calls through multiple objects (`a.getB().getC().doSomething()`)

**Example: TypeScript**

```typescript
// ❌ BAD: Violates Law of Demeter (train wreck)
class Customer {
  getWallet(): Wallet { /* ... */ }
}

class Wallet {
  getMoney(): Money { /* ... */ }
}

class Money {
  getAmount(): number { /* ... */ }
}

// This code knows too much about internal structure!
const purchase = (customer: Customer, price: number): boolean => {
  const amount = customer.getWallet().getMoney().getAmount(); // Train wreck!
  return amount >= price;
};

// ✅ GOOD: Tell, don't ask - delegate to the owning module
class Customer {
  constructor(private wallet: Wallet) {}

  canAfford(price: number): boolean {
    return this.wallet.hasEnough(price); // Delegate to wallet
  }
}

class Wallet {
  constructor(private money: Money) {}

  hasEnough(amount: number): boolean {
    return this.money.isGreaterThanOrEqual(amount); // Delegate to money
  }
}

class Money {
  constructor(private amount: number) {}

  isGreaterThanOrEqual(other: number): boolean {
    return this.amount >= other;
  }
}

// Clean code: only talks to immediate friend
const purchase = (customer: Customer, price: number): boolean => {
  return customer.canAfford(price);
};
```

**Example: Elixir**

```elixir
# ❌ BAD: Violates Law of Demeter
defmodule Purchase do
  def can_afford?(customer, price) do
    # Train wreck: reaching through multiple levels
    customer.wallet.money.amount >= price
  end
end

# ✅ GOOD: Delegate to owning module
defmodule Customer do
  defstruct [:name, :wallet]
  
  def can_afford?(customer, price) do
    Wallet.has_enough?(customer.wallet, price)
  end
end

defmodule Wallet do
  defstruct [:money]
  
  def has_enough?(wallet, amount) do
    Money.greater_than_or_equal?(wallet.money, amount)
  end
end

defmodule Money do
  defstruct [:amount, :currency]
  
  def greater_than_or_equal?(money, amount) do
    money.amount >= amount
  end
end

# Clean code
defmodule Purchase do
  def can_afford?(customer, price) do
    Customer.can_afford?(customer, price)
  end
end
```

**Advanced Elixir Example: Assignment & Worker Delegation**

```elixir
# ❌ BAD: Violates Law of Demeter with deep drilling
defmodule BadAssignmentLogic do
  def worker_city(engagement) do
    # Deep drilling through multiple levels - violates Law of Demeter
    engagement.worker.address.city
  end
  
  def worker_hourly_rate(engagement) do
    # Reaching through multiple objects
    engagement.worker.profile.billing_info.hourly_rate
  end
  
  def is_local_worker?(engagement, target_city) do
    engagement.worker.address.city == target_city
  end
end

# ✅ GOOD: Delegate to owning modules with pattern matching
defmodule Assignment do
  defstruct [:id, :worker_id, :project_id, :status]
  
  def worker_city(engagement) do
    # Delegate to the module that owns worker data
    Worker.city_for_assignment(engagement.worker_id)
  end
  
  def hourly_rate(engagement) do
    Worker.hourly_rate(engagement.worker_id)
  end
  
  def is_local_assignment?(engagement, target_city) do
    case Worker.city_for_assignment(engagement.worker_id) do
      ^target_city -> true
      _ -> false
    end
  end
end

defmodule Worker do
  defstruct [:id, :profile, :address_id]
  
  def city_for_assignment(worker_id) do
    # Single responsibility: Worker knows how to get its own city
    with {:ok, worker} <- get_worker(worker_id),
         {:ok, address} <- Address.get(worker.address_id) do
      address.city
    else
      _ -> nil
    end
  end
  
  def hourly_rate(worker_id) do
    # Worker delegates to Profile for billing information
    with {:ok, worker} <- get_worker(worker_id) do
      Profile.hourly_rate(worker.profile.id)
    end
  end
  
  defp get_worker(id), do: Repo.get(Worker, id)
end

defmodule Address do
  defstruct [:id, :street, :city, :state, :country]
  
  def get(address_id) do
    Repo.get(Address, address_id)
  end
end

defmodule Profile do
  defstruct [:id, :billing_info]
  
  def hourly_rate(profile_id) do
    # Profile owns billing information
    case Repo.get(Profile, profile_id) do
      %{billing_info: %{hourly_rate: rate}} -> rate
      _ -> nil
    end
  end
end

# Usage: Clean delegation chain
engagement = %Assignment{worker_id: 123, project_id: 456}
city = Assignment.worker_city(engagement)          # Clean, no train wrecks
rate = Assignment.hourly_rate(engagement)          # Proper delegation
local? = Assignment.is_local_assignment?(engagement, "Austin")
```

**Benefits**:
- Reduces coupling: Changes to internal structure don't break clients
- Easier to refactor: Internal implementation can change freely
- Better encapsulation: Internals remain hidden

#### 3. Tell, Don't Ask [HIGH]

**Rule ID**: `struct-tell-dont-ask`

**Principle**: Tell objects what to do, don't ask for data and make decisions for them.

**Why**: Objects should encapsulate both data and behavior. Don't pull data out and operate on it externally.

**Example: TypeScript**

```typescript
// ❌ BAD: Asking for data and making decisions
class Account {
  private balance: number = 0;

  getBalance(): number {
    return this.balance;
  }

  setBalance(amount: number): void {
    this.balance = amount;
  }
}

// External code makes decisions based on pulled data
const withdraw = (account: Account, amount: number): void => {
  if (account.getBalance() >= amount) { // Asking!
    account.setBalance(account.getBalance() - amount); // Making decisions!
  } else {
    throw new Error('Insufficient funds');
  }
};

// ✅ GOOD: Tell the object what to do
class Account {
  private balance: number = 0;

  withdraw(amount: number): void {
    if (this.balance < amount) {
      throw new Error('Insufficient funds');
    }
    this.balance -= amount;
  }

  deposit(amount: number): void {
    this.balance += amount;
  }
}

// External code just tells the account what to do
const performWithdrawal = (account: Account, amount: number): void => {
  account.withdraw(amount); // Telling, not asking
};
```

**Example: Elixir**

```elixir
# ❌ BAD: Asking for data and making decisions
defmodule Account do
  defstruct balance: 0
end

defmodule Banking do
  def withdraw(account, amount) do
    # Asking for data and making decisions externally
    if account.balance >= amount do
      %{account | balance: account.balance - amount}
    else
      {:error, :insufficient_funds}
    end
  end
end

# ✅ GOOD: Tell the module what to do
defmodule Account do
  defstruct balance: 0
  
  def withdraw(account, amount) do
    if account.balance < amount do
      {:error, :insufficient_funds}
    else
      {:ok, %{account | balance: account.balance - amount}}
    end
  end
  
  def deposit(account, amount) do
    {:ok, %{account | balance: account.balance + amount}}
  end
end

# External code just tells the account what to do
defmodule Banking do
  def perform_withdrawal(account, amount) do
    Account.withdraw(account, amount)
  end
end

# Advanced Pattern Matching Example with Assignment Confirmation
defmodule Assignment do
  defstruct [:id, :worker_id, :project_id, :status, :confirmed_at]
  
  # ✅ EXCELLENT: Pattern matching encapsulates business rules
  def confirm(%{status: :pending} = assignment) do
    # Tell the assignment to confirm itself, don't ask about status
    {:ok, %{assignment | 
            status: :confirmed, 
            confirmed_at: DateTime.utc_now()}}
  end
  
  def confirm(%{status: :confirmed}), do: {:error, :already_confirmed}
  def confirm(%{status: :cancelled}), do: {:error, :cannot_confirm_cancelled}
  def confirm(_), do: {:error, :invalid_status}
  
  # Business logic stays with the data
  def can_be_charged?(%{status: status}) when status in [:confirmed, :in_progress], do: true
  def can_be_charged?(_), do: false
end

# Usage: Tell, don't ask
case Assignment.confirm(assignment) do
  {:ok, confirmed_assignment} -> 
    # Further actions based on successful confirmation
    notify_worker(confirmed_assignment)
  {:error, reason} -> 
    handle_confirmation_error(reason)
end

# Advanced Task Example with Polymorphism
defmodule Task do
  defstruct [:id, :type, :status, :assignee_id, :rate_info]
end

defmodule TaskCharging do
  # ✅ EXCELLENT: Delegate to specialized modules based on task type
  def calculate_charge(task) do
    # Tell the appropriate module to handle charging
    case task.type do
      :hourly -> HourlyCharging.calculate(task)
      :fixed -> FixedCharging.calculate(task)
      :milestone -> MilestoneCharging.calculate(task)
    end
  end
end

defmodule HourlyCharging do
  def calculate(%{rate_info: %{hourly_rate: rate, hours_worked: hours}}) do
    {:ok, rate * hours}
  end
  def calculate(_), do: {:error, :missing_hourly_data}
end

defmodule FixedCharging do
  def calculate(%{rate_info: %{fixed_price: price}, status: :completed}) do
    {:ok, price}
  end
  def calculate(%{status: status}) when status != :completed do
    {:error, :task_not_completed}
  end
  def calculate(_), do: {:error, :missing_fixed_price}
end
```

**Benefits**:
- Better encapsulation: Logic stays with the data
- Easier to change: Business rules centralized in one place
- More object-oriented/modular design

#### 4. Encapsulation [MEDIUM]

**Rule ID**: `struct-encapsulation`

**Principle**: Hide internal details and expose a minimal, stable interface.

**Why**: Internal implementation can change without affecting clients. Reduces coupling and increases flexibility.

**Rules**:
- Make fields/properties private
- Expose behavior through public methods, not data access
- Use immutable data structures where possible
- Don't expose internal structure

**Example: TypeScript**

```typescript
// ❌ BAD: Exposing internal structure
class Order {
  public items: OrderItem[] = []; // Public array!
  public total: number = 0;

  addItem(item: OrderItem): void {
    this.items.push(item);
    this.total += item.price;
  }
}

// Client code can break invariants
const order = new Order();
order.items.push(invalidItem); // Bypassed addItem validation!
order.total = 999; // Manually changed total, now inconsistent!

// ✅ GOOD: Hide internal details, expose minimal interface
class Order {
  private items: OrderItem[] = [];

  addItem(item: OrderItem): void {
    if (!this.isValidItem(item)) {
      throw new Error('Invalid item');
    }
    this.items = [...this.items, item]; // Immutable update
  }

  removeItem(itemId: string): void {
    this.items = this.items.filter(item => item.id !== itemId);
  }

  getTotal(): number {
    return this.items.reduce((sum, item) => sum + item.price, 0);
  }

  getItemCount(): number {
    return this.items.length;
  }

  private isValidItem(item: OrderItem): boolean {
    return item.price > 0 && item.quantity > 0;
  }
}

// Client can only use public interface
const order = new Order();
order.addItem(item); // Validated
const total = order.getTotal(); // Computed from items, always consistent
```

**Example: Elixir**

```elixir
# ❌ BAD: Exposing internal structure
defmodule Order do
  defstruct items: [], total: 0
end

# Clients can break invariants
order = %Order{items: [invalid_item], total: 999} # Bypassed validation!

# ✅ GOOD: Hide internal details with module API
defmodule Order do
  # Internal structure (documented but not relied upon by clients)
  defstruct items: []
  
  # Public API
  def new(), do: %Order{}
  
  def add_item(order, item) do
    if valid_item?(item) do
      {:ok, %{order | items: [item | order.items]}}
    else
      {:error, :invalid_item}
    end
  end
  
  def remove_item(order, item_id) do
    items = Enum.reject(order.items, &(&1.id == item_id))
    {:ok, %{order | items: items}}
  end
  
  def get_total(order) do
    Enum.reduce(order.items, 0, fn item, sum -> sum + item.price end)
  end
  
  def get_item_count(order) do
    length(order.items)
  end
  
  # Private helper
  defp valid_item?(item) do
    item.price > 0 and item.quantity > 0
  end
end

# Clients use public API
{:ok, order} = Order.new()
{:ok, order} = Order.add_item(order, item)
total = Order.get_total(order)

# Advanced Encapsulation: Opaque Types and Ecto Schemas
defmodule UserId do
  # ✅ EXCELLENT: Opaque type hides internal representation
  @opaque t :: String.t()
  
  def new(id) when is_binary(id) and byte_size(id) > 0, do: {:ok, id}
  def new(_), do: {:error, :invalid_user_id}
  
  def to_string(user_id), do: user_id
  
  # Only this module can create/manipulate UserIds
  def from_integer(id) when is_integer(id), do: {:ok, Integer.to_string(id)}
  def from_integer(_), do: {:error, :invalid_integer}
end

# Ecto Schema with Controlled Access
defmodule Task do
  use Ecto.Schema
  import Ecto.Changeset
  
  # Internal structure hidden through changesets
  schema "tasks" do
    field :title, :string
    field :status, Ecto.Enum, values: [:pending, :in_progress, :completed, :cancelled]
    field :estimated_hours, :decimal
    field :actual_hours, :decimal
    field :assignee_id, :binary_id
    
    timestamps()
  end
  
  # Public API: Only way to modify tasks
  def create_changeset(attrs \\ %{}) do
    %Task{}
    |> cast(attrs, [:title, :estimated_hours, :assignee_id])
    |> validate_required([:title, :assignee_id])
    |> validate_length(:title, min: 1, max: 255)
    |> validate_number(:estimated_hours, greater_than: 0)
    |> put_change(:status, :pending)
  end
  
  def update_changeset(task, attrs) do
    task
    |> cast(attrs, [:title, :estimated_hours])
    |> validate_required([:title])
    |> validate_length(:title, min: 1, max: 255)
    |> validate_number(:estimated_hours, greater_than: 0)
  end
  
  def start_changeset(task) do
    case task.status do
      :pending ->
        change(task, status: :in_progress)
      _ ->
        change(task) |> add_error(:status, "can only start pending tasks")
    end
  end
  
  def complete_changeset(task, actual_hours) do
    case task.status do
      :in_progress ->
        change(task)
        |> put_change(:status, :completed)
        |> put_change(:actual_hours, actual_hours)
        |> validate_number(:actual_hours, greater_than: 0)
      _ ->
        change(task) |> add_error(:status, "can only complete in-progress tasks")
    end
  end
  
  # Computed fields (no direct field access)
  def hours_variance(task) do
    case {task.estimated_hours, task.actual_hours} do
      {estimated, actual} when not is_nil(estimated) and not is_nil(actual) ->
        Decimal.sub(actual, estimated)
      _ ->
        nil
    end
  end
  
  def is_overbudget?(task) do
    case hours_variance(task) do
      variance when not is_nil(variance) -> Decimal.positive?(variance)
      _ -> false
    end
  end
end

# Usage: Controlled access through changesets
changeset = Task.create_changeset(%{title: "Fix bug", estimated_hours: 2, assignee_id: user_id})
{:ok, task} = Repo.insert(changeset)

# State transitions through controlled changesets
{:ok, started_task} = 
  task
  |> Task.start_changeset()
  |> Repo.update()

{:ok, completed_task} = 
  started_task
  |> Task.complete_changeset(Decimal.new("2.5"))
  |> Repo.update()

# Safe computed access
variance = Task.hours_variance(completed_task)
overbudget? = Task.is_overbudget?(completed_task)
```

**Benefits**:
- Internal implementation can change freely
- Invariants are enforced and protected
- Reduces coupling between modules
- Easier to reason about and test

---

## Part 3: Code Quality Fundamentals

Code quality principles ensure your code is readable, maintainable, and free of common defects.

### Core Principles

#### DRY (Don't Repeat Yourself)

**Rule ID**: `core-dry`

**Principle**: Every piece of knowledge should have a single, authoritative representation.

**Why**: Duplication leads to maintenance nightmares. When logic changes, you must find and update every copy.

```typescript
// ❌ BAD: Duplicated validation logic
const validateUserEmail = (email: string): boolean => {
  return email.includes('@') && email.length > 5;
};

const validateAdminEmail = (email: string): boolean => {
  return email.includes('@') && email.length > 5; // Duplicated!
};

// ✅ GOOD: Single source of truth
const isValidEmail = (email: string): boolean => {
  return email.includes('@') && email.length > 5;
};

const validateUserEmail = (email: string): boolean => isValidEmail(email);
const validateAdminEmail = (email: string): boolean => isValidEmail(email);
```

**When to violate DRY**: When abstractions are premature or force unnatural coupling. Prefer duplication over wrong abstraction.

#### KISS (Keep It Simple, Stupid)

**Rule ID**: `core-kiss`

**Principle**: Simplicity should be a key goal. Avoid unnecessary complexity.

**Why**: Simple code is easier to understand, test, and maintain.

```typescript
// ❌ BAD: Overly complex
const getUserStatus = (user: User): string => {
  return user.isActive 
    ? (user.isPremium 
      ? (user.hasAccess 
        ? 'premium-active-access' 
        : 'premium-active-no-access')
      : (user.hasAccess 
        ? 'basic-active-access' 
        : 'basic-active-no-access'))
    : 'inactive';
};

// ✅ GOOD: Simple and clear
const getUserStatus = (user: User): string => {
  if (!user.isActive) return 'inactive';
  if (!user.hasAccess) return user.isPremium ? 'premium-no-access' : 'basic-no-access';
  return user.isPremium ? 'premium-active' : 'basic-active';
};
```

#### YAGNI (You Aren't Gonna Need It)

**Rule ID**: `core-yagni`

**Principle**: Don't implement features until you actually need them.

**Why**: Speculative features add complexity, maintenance cost, and often go unused.

```typescript
// ❌ BAD: Over-engineered for future needs
class User {
  constructor(
    private id: string,
    private name: string,
    private email: string,
    private phone?: string, // Not needed yet
    private address?: Address, // Not needed yet
    private preferences?: UserPreferences, // Not needed yet
    private socialLinks?: SocialLinks, // Not needed yet
    private paymentMethods?: PaymentMethod[] // Not needed yet
  ) {}
}

// ✅ GOOD: Only what's needed now
class User {
  constructor(
    private id: string,
    private name: string,
    private email: string
  ) {}
}
// Add fields when requirements actually materialize
```

#### Separation of Concerns

**Principle**: Separate your program into distinct sections, each addressing a separate concern.

**Why**: Changes to one concern don't affect others. Easier to understand and maintain.

```typescript
// ❌ BAD: Mixed concerns (data access + business logic + presentation)
const getUserProfile = async (userId: string): Promise<string> => {
  const user = await db.query(`SELECT * FROM users WHERE id = ${userId}`); // Data access
  if (user.isActive && user.credits > 10) { // Business logic
    return `<h1>Welcome ${user.name}!</h1>`; // Presentation
  }
  return `<h1>Account suspended</h1>`;
};

// ✅ GOOD: Separated concerns
// Data access layer
const userRepository = {
  findById: async (id: string): Promise<User> => {
    return db.query('SELECT * FROM users WHERE id = $1', [id]);
  }
};

// Business logic layer
const canAccessProfile = (user: User): boolean => {
  return user.isActive && user.credits > 10;
};

// Presentation layer
const renderProfile = (user: User, canAccess: boolean): string => {
  if (canAccess) {
    return `<h1>Welcome ${user.name}!</h1>`;
  }
  return `<h1>Account suspended</h1>`;
};

// Orchestration (use case)
const getUserProfile = async (userId: string): Promise<string> => {
  const user = await userRepository.findById(userId);
  const canAccess = canAccessProfile(user);
  return renderProfile(user, canAccess);
};
```

#### Fail Fast

**Principle**: Detect and report errors as early as possible.

**Why**: Makes debugging easier. Problems are caught close to their source.

```typescript
// ❌ BAD: Silent failures and late detection
const processOrder = (order: Order): void => {
  const items = order.items || []; // Silently handles missing items
  const total = items.reduce((sum, item) => sum + (item.price || 0), 0); // Silently handles missing price
  if (total > 0) {
    // Process order...
  }
  // Order with no items or invalid prices passes through!
};

// ✅ GOOD: Fail fast with validation
const processOrder = (order: Order): void => {
  if (!order.items || order.items.length === 0) {
    throw new Error('Order must have at least one item');
  }
  
  for (const item of order.items) {
    if (!item.price || item.price <= 0) {
      throw new Error(`Invalid price for item ${item.id}`);
    }
  }
  
  const total = order.items.reduce((sum, item) => sum + item.price, 0);
  // Process order...
};
```

### Design Patterns

Common patterns for solving recurring design problems.

#### Factory Pattern

**Rule ID**: `pattern-factory`

**Purpose**: Create objects without specifying exact class.

```typescript
interface PaymentMethod {
  process(amount: number): Promise<PaymentResult>;
}

class PaymentFactory {
  create(type: string): PaymentMethod {
    switch (type) {
      case 'credit_card':
        return new CreditCardPayment();
      case 'paypal':
        return new PayPalPayment();
      case 'crypto':
        return new CryptoPayment();
      default:
        throw new Error(`Unknown payment type: ${type}`);
    }
  }
}
```

#### Strategy Pattern

**Rule ID**: `pattern-strategy`

**Purpose**: Define family of algorithms, encapsulate each one, make them interchangeable.

```typescript
interface SortStrategy {
  sort(data: number[]): number[];
}

class QuickSort implements SortStrategy {
  sort(data: number[]): number[] {
    // Quick sort implementation
    return data;
  }
}

class MergeSort implements SortStrategy {
  sort(data: number[]): number[] {
    // Merge sort implementation
    return data;
  }
}

class DataProcessor {
  constructor(private strategy: SortStrategy) {}

  process(data: number[]): number[] {
    return this.strategy.sort(data);
  }
}

// Use different strategies
const processor1 = new DataProcessor(new QuickSort());
const processor2 = new DataProcessor(new MergeSort());
```

#### Repository Pattern

**Rule ID**: `pattern-repository`

**Purpose**: Encapsulate data access logic.

```typescript
interface UserRepository {
  findById(id: string): Promise<User | null>;
  save(user: User): Promise<void>;
  delete(id: string): Promise<void>;
}

class PostgresUserRepository implements UserRepository {
  async findById(id: string): Promise<User | null> {
    return this.db.query('SELECT * FROM users WHERE id = $1', [id]);
  }

  async save(user: User): Promise<void> {
    await this.db.query('INSERT INTO users ...', [user]);
  }

  async delete(id: string): Promise<void> {
    await this.db.query('DELETE FROM users WHERE id = $1', [id]);
  }
}
```

#### Observer Pattern

**Rule ID**: `pattern-observer`

**Purpose**: Notify multiple objects about state changes.

```typescript
interface Observer {
  update(data: any): void;
}

class Subject {
  private observers: Observer[] = [];

  attach(observer: Observer): void {
    this.observers.push(observer);
  }

  detach(observer: Observer): void {
    this.observers = this.observers.filter(o => o !== observer);
  }

  notify(data: any): void {
    for (const observer of this.observers) {
      observer.update(data);
    }
  }
}

class EmailNotifier implements Observer {
  update(data: any): void {
    console.log('Sending email notification:', data);
  }
}

class SlackNotifier implements Observer {
  update(data: any): void {
    console.log('Sending Slack notification:', data);
  }
}
```

### Code Organization

#### Naming Conventions

**Rules**:
- Use descriptive names that reveal intent
- Avoid abbreviations unless universally understood
- Use verbs for functions, nouns for classes/types
- Use consistent naming patterns

```typescript
// ❌ BAD
const d = new Date();
const usr = getUsr(id);
const calc = (a, b) => a + b;

// ✅ GOOD
const currentDate = new Date();
const user = getUserById(id);
const calculateTotal = (price, quantity) => price * quantity;
```

#### Function Guidelines

**Rules**:
- Keep functions small (< 20 lines ideally)
- Do one thing and do it well
- Minimize parameters (< 3 ideally)
- Avoid side effects when possible

```typescript
// ❌ BAD: Too many responsibilities
const processUser = (user: User, sendEmail: boolean, updateDb: boolean, logAction: boolean) => {
  // Validation
  if (!user.email.includes('@')) throw new Error('Invalid email');
  
  // Business logic
  user.status = 'active';
  
  // Side effects
  if (sendEmail) emailService.send(user.email, 'Welcome!');
  if (updateDb) db.save(user);
  if (logAction) logger.log('User processed');
  
  return user;
};

// ✅ GOOD: Single responsibility, composed
const validateUser = (user: User): void => {
  if (!user.email.includes('@')) throw new Error('Invalid email');
};

const activateUser = (user: User): User => {
  return { ...user, status: 'active' };
};

const processUser = async (user: User): Promise<User> => {
  validateUser(user);
  const activeUser = activateUser(user);
  await saveUser(activeUser);
  await sendWelcomeEmail(activeUser.email);
  logUserActivation(activeUser.id);
  return activeUser;
};
```

#### Error Handling

**Rules**:
- Use exceptions for exceptional conditions
- Handle errors at appropriate level
- Provide context in error messages
- Don't swallow errors

```typescript
// ❌ BAD: Swallowing errors
const getUser = async (id: string): Promise<User | null> => {
  try {
    return await db.findUser(id);
  } catch (error) {
    return null; // Error information lost!
  }
};

// ✅ GOOD: Propagate with context
const getUser = async (id: string): Promise<User> => {
  try {
    const user = await db.findUser(id);
    if (!user) {
      throw new Error(`User not found: ${id}`);
    }
    return user;
  } catch (error) {
    throw new Error(`Failed to get user ${id}: ${error.message}`);
  }
};
```

---

## Implementation Checklist

When implementing or reviewing code, use this checklist:

### Strategic Design
- [ ] Dependencies point inward toward business logic
- [ ] Entities contain pure business rules
- [ ] Use cases orchestrate operations
- [ ] Boundaries defined with interfaces
- [ ] Architecture decisions documented (ADRs)

### SOLID Principles
- [ ] Each module has single responsibility
- [ ] Code open for extension, closed for modification
- [ ] Subtypes substitutable for base types
- [ ] Interfaces focused and minimal
- [ ] Dependencies inverted (depend on abstractions)

### Structural Principles
- [ ] Composition used over inheritance
- [ ] Law of Demeter followed (no train wrecks)
- [ ] Tell, don't ask (behavior with data)
- [ ] Internal details encapsulated

### Code Quality
- [ ] No duplication (DRY)
- [ ] Simple solutions (KISS)
- [ ] Only necessary features (YAGNI)
- [ ] Concerns separated
- [ ] Fails fast with validation
- [ ] Descriptive naming
- [ ] Small, focused functions
- [ ] Proper error handling

---

## Common Violations and How to Fix Them

### Violation: God Class/Module

**Symptom**: One class/module with hundreds of lines doing many things.

**Fix**: Apply Single Responsibility Principle. Extract cohesive groups of functionality into separate modules.

### Violation: Tight Coupling

**Symptom**: Changing one module requires changes in many others.

**Fix**: Apply Dependency Inversion. Depend on abstractions. Use interfaces and dependency injection.

### Violation: Shotgun Surgery

**Symptom**: One change requires modifying many files.

**Fix**: Apply Separation of Concerns. Group related functionality together.

### Violation: Feature Envy

**Symptom**: Method in one class uses data/methods from another class more than its own.

**Fix**: Apply Tell, Don't Ask. Move behavior to the class that owns the data.

### Violation: Inappropriate Intimacy

**Symptom**: Two modules know too much about each other's internals.

**Fix**: Apply Encapsulation and Law of Demeter. Hide internal details behind interfaces.

---

## When to Apply These Principles

**During Design**:
- Use strategic principles (Clean Architecture, ADRs)
- Consider system boundaries and dependencies
- Choose appropriate patterns

**During Implementation**:
- Apply SOLID and structural principles
- Write clean, simple code
- Follow naming and organization conventions

**During Code Review**:
- Check for principle violations
- Look for coupling and complexity
- Suggest refactorings

**During Refactoring**:
- Use principles to guide improvements
- Break apart god classes
- Introduce abstractions
- Eliminate duplication


---

## Error Handling and Fallback Guidance

This section provides guidance for agents when workflows fail, edge cases occur, or unexpected situations arise during design principle application.

### 🚨 Common Failure Scenarios and Recovery

#### 1. Reference Loading Failures

**Problem**: Cannot load required reference file or context limit reached

**Fallback Actions**:
1. **First**: Apply the rule ID directly from memory (e.g., dep-inward-only = dependencies point inward to domain)
2. **If unclear**: Use the NEVER lists and Ask Yourself frameworks from main skill
3. **Emergency**: Focus only on CRITICAL rules (dep-*, entity-*) and skip MEDIUM/LOW priority
4. **Last resort**: Report partial analysis with clear limitations noted

**Example Recovery**:
```
⚠️ Could not load dep-inward-only.md reference
✓ Applying general principle: Domain layer must not import infrastructure
✓ Checking for framework imports in entities...
⚠️ Partial analysis - recommend manual review of boundary violations
```

#### 2. Conflicting Principle Applications

**Problem**: Two principles seem to contradict each other in specific context

**Resolution Priority**:
1. **CRITICAL** principles override **HIGH/MEDIUM/LOW**
2. **Entity Design** overrides **Tactical Design** when in conflict
3. **Dependency Direction** overrides **SOLID Principles** for architecture
4. **Business rules** override **Technical convenience**

**Fallback Process**:
```
1. Identify conflict → Document both principles
2. Check priority levels → Apply higher priority rule
3. If same priority → Apply most restrictive interpretation  
4. Document decision rationale in output
5. Flag for human review if critical business impact
```

#### 3. Unclear or Ambiguous Code Context

**Problem**: Code structure doesn't fit standard patterns or is too complex to analyze

**Fallback Actions**:
1. **Focus on obvious violations first** (imports, naming, size)
2. **Use pattern matching**: Does this look like X pattern? Apply X rules
3. **Default to conservative approach**: Flag for human review rather than force incorrect principle
4. **Provide multiple suggestions**: "This could be A or B, consider..."

**Example Response**:
```
⚠️ Complex code structure detected
✓ Clear violations: 15+ imports (dep-inward-only), 300+ lines (solid-srp)
? Uncertain: Business logic placement - could be entity or service
→ Recommend: Fix clear violations first, then architect review for structure
```

#### 4. Legacy System Complications

**Problem**: Applying modern principles to legacy code seems impossible or destructive

**Pragmatic Fallbacks**:
1. **Assess change impact**: Is this refactoring or new development?
2. **Grandfather existing violations**: Focus on preventing new violations
3. **Incremental approach**: Apply principles only to modified sections
4. **Anti-corruption layers**: Use boundaries to isolate legacy violations

**Triage Strategy**:
```
Legacy Code Decision Matrix:
- Changing frequently → Apply full principles
- Stable but needs modification → Apply minimum viable principles  
- Read-only/deprecated → Document violations, no changes
- Critical path → Apply CRITICAL principles only
```

#### 5. Performance vs Principles Trade-offs

**Problem**: Principle application would harm performance significantly

**Balance Framework**:
1. **Never sacrifice**: Entity invariants, security boundaries, data integrity
2. **Consider trade-offs**: Interface segregation, law of demeter for performance hotpaths
3. **Document exceptions**: Clear reasoning for principle violations
4. **Monitor and revisit**: Set alerts for technical debt payback

**Example Decision Process**:
```
Performance Issue: N+1 queries from strict law of demeter
✓ Option A: Keep principle, optimize with caching
✓ Option B: Controlled violation with explicit batching interface
✗ Option C: Remove all encapsulation (violates entity-pure-business-rules)
→ Choose A or B based on performance requirements, document decision
```

### 🛠️ Workflow Recovery Patterns

#### When Analysis Gets Stuck

1. **Step back to higher level**: Focus on architecture boundaries instead of code details
2. **Use decision trees**: Follow crisis triage or primary decision tree for guidance
3. **Apply 80/20 rule**: Focus on 20% of violations causing 80% of problems
4. **Time-box analysis**: Set limit, provide best effort within constraints

#### When Recommendations Are Too Complex

1. **Prioritize by impact**: CRITICAL → HIGH → MEDIUM → LOW
2. **Group related violations**: Fix all entity issues together
3. **Provide implementation order**: Dependencies first, then SOLID, then structural
4. **Offer alternatives**: "If you can't do X, at least do Y"

#### When Context Is Insufficient

1. **State assumptions clearly**: "Based on limited context, assuming X..."
2. **Request specific information**: "Need to see entity definitions for complete analysis"
3. **Provide conditional guidance**: "If this is a service, then... If entity, then..."
4. **Escalate appropriately**: "Requires architect review for business context"

### 🔄 Continuous Recovery Strategies

#### Monitoring Application Success

- **Track violation trends**: Are violations increasing or decreasing?
- **Measure change impact**: How much does principle application slow development?
- **Assess code quality metrics**: Complexity, coupling, testability improvements
- **Gather team feedback**: Are principles helping or hindering development?

#### Adapting to Organizational Context

- **Company culture**: More or less strict principle enforcement
- **Team experience**: Adjust complexity of recommendations
- **Legacy constraints**: Adapt expectations for older systems
- **Business priorities**: Balance principle purity with delivery pressure

#### Recovery from Failed Implementations

1. **Analyze failure causes**: Too complex? Wrong timing? Insufficient buy-in?
2. **Scale back scope**: Focus on critical principles only
3. **Improve communication**: Better explanation of benefits
4. **Provide training**: Help team understand principles better
5. **Adjust timeline**: Allow more time for principle adoption

---

## Agent Strategy Documentation

This section outlines different approaches for applying software design principles in various development contexts.

### 1. Architecture Review Strategy

**Purpose**: Evaluate system architecture for principle violations and structural issues

**Approach**:
1. **Dependency Analysis**: Check for `dep-*` rule violations across system boundaries
2. **Layer Validation**: Verify clean separation between entities, use cases, and adapters  
3. **Coupling Assessment**: Identify tight coupling and suggest decoupling strategies
4. **Abstraction Review**: Ensure proper use of interfaces and dependency inversion

**Focus Areas**:
- CRITICAL violations first: dependency direction, entity purity
- Architecture decision documentation using rule IDs
- Technology isolation and framework boundaries
- Component cohesion and interface design

**Output Format**:
```
Architecture Review Report:
========================
System: Payment Processing Service
Date: 2024-03-15

CRITICAL Issues (3):
- src/domain/payment.ts:12 - [dep-inward-only] Domain importing infrastructure
- src/entities/order.ts:45 - [entity-pure-business-rules] Entity contains database logic

Recommendations:
1. Create Payment interface in domain layer
2. Move database logic to infrastructure layer
3. Apply dependency inversion to decouple layers
```

### 2. Code Quality Audit Strategy

**Purpose**: Systematic review of codebase for design principle violations

**Approach**:
1. **SOLID Principle Scan**: Check all classes/modules for SRP, OCP, LSP, ISP, DIP violations
2. **Structural Pattern Review**: Apply Law of Demeter, Tell Don't Ask, Composition Over Inheritance
3. **Anti-Pattern Detection**: Identify God classes, feature envy, data clumps
4. **Complexity Analysis**: Find overly complex methods and suggest decomposition

**Focus Areas**:
- HIGH priority violations: SOLID principles, structural patterns
- Real-world violation examples with concrete fixes
- Code complexity metrics and thresholds
- Testability assessment using design principles

**Workflow**:
1. Generate violation report with priorities
2. Group violations by file and principle category
3. Provide specific refactoring recommendations
4. Suggest testing strategies for improved designs

### 3. Refactoring Guidance Strategy

**Purpose**: Guide systematic improvement of existing code using design principles

**Approach**:
1. **Smell Detection**: Identify code smells that violate design principles
2. **Refactoring Planning**: Prioritize refactoring based on principle violations
3. **Safe Refactoring**: Apply principle-guided transformations incrementally
4. **Validation**: Verify refactoring improves design principle compliance

**Refactoring Priorities**:
1. **CRITICAL**: Fix dependency direction violations (breaks architecture)
2. **HIGH**: Address SOLID violations (improves maintainability)
3. **MEDIUM**: Apply structural patterns (reduces coupling)
4. **LOW**: Clean up naming and documentation

**Example Refactoring Plan**:
```
File: src/user-service.ts (4 violations)

Phase 1 - Critical (Week 1):
- [dep-inward-only] Remove direct database imports
- Extract repository interface

Phase 2 - High (Week 2):  
- [solid-srp] Split authentication from validation
- [solid-ocp] Make validation extensible

Phase 3 - Medium (Week 3):
- [struct-tell-dont-ask] Move behavior to User entity
- [struct-law-of-demeter] Reduce property chains
```

### 4. Test-Driven Design Strategy

**Purpose**: Use design principles to guide testable code structure during TDD

**Approach**:
1. **Red Phase**: Write test that requires well-designed interface (interfaces, SRP)
2. **Green Phase**: Apply SOLID principles to create testable implementation  
3. **Refactor Phase**: Use structural principles to improve design
4. **Design Validation**: Ensure code follows dependency inversion for test doubles

**Design-Test Integration**:
- SRP makes tests focused and clear
- OCP enables easy test case addition
- DIP allows test double injection
- Tell Don't Ask leads to behavior-focused tests

### 5. Real-Time Code Review Strategy

**Purpose**: Apply design principles during active code review process

**Approach**:
1. **Violation Spotting**: Identify principle violations using rule IDs
2. **Immediate Feedback**: Provide specific, actionable design guidance
3. **Learning Moments**: Explain why violations matter and how to fix them
4. **Consistency**: Apply same standards across all code reviews

**Review Checklist**:
- [ ] Does this follow Single Responsibility Principle?
- [ ] Are dependencies pointing inward (Clean Architecture)?
- [ ] Is the code open for extension, closed for modification?
- [ ] Does it avoid feature envy and inappropriate intimacy?
- [ ] Are abstractions stable and concretions volatile?

**Integration Notes**:
- Use with code-reviewer skill for automated violation detection
- Combine with TDD skill for design-driven development
- Apply architecture-design skill for system-level guidance
- Reference documentation skills for principle explanations

---

## Integration with Other Skills

This skill works synergistically with other development skills to create a comprehensive software quality framework.

### With Code-Reviewer Skill

**Purpose**: Use design principles as code review criteria

**Integration Points**:
- **Automated Violation Detection**: Code reviewer checks for principle violations using rule IDs
- **Structured Feedback**: Output format `file:line - [rule-id] Description` for consistent reviews  
- **Priority-Based Reviews**: Focus on CRITICAL/HIGH violations first (dependency direction, SOLID principles)
- **Anti-Pattern Recognition**: Identify common violations like God classes, feature envy, inappropriate intimacy

**Example Workflow**:
1. Code-reviewer scans for rule violations (solid-srp, dep-inward-only, struct-law-of-demeter)
2. Reports violations with specific rule IDs and line numbers
3. Provides refactoring suggestions based on design principles
4. Validates fixes against principle guidelines

```
# Code review output example:
src/UserService.ts:45 - [solid-srp] Class handles authentication, validation, and email sending
src/PaymentProcessor.ts:12 - [dep-inward-only] Infrastructure layer importing from domain
src/OrderController.ts:78 - [struct-tell-dont-ask] Asking object for data instead of telling it to act
```

### With Architecture-Design Skill

**Purpose**: Apply strategic design principles to architectural decisions

**Integration Points**:
- **Clean Architecture Enforcement**: Use architectural boundary rules from references/
- **Dependency Management**: Apply dep-* rules to enforce architecture constraints
- **Layer Design**: Entity, use case, and interface adapter principles guide layer design
- **Technology Decisions**: Framework isolation principles guide technology choices

**Example Workflow**:
1. Architecture-design defines system boundaries and layers
2. Software-design-principles provides rules for maintaining boundaries
3. Dependency direction rules prevent architectural drift
4. Entity and use case principles guide core domain design

### With Test-Driven Development (TDD) Skill

**Purpose**: Design principles guide testable code structure

**Integration Points**:
- **Design for Testability**: SOLID principles make code easier to test in isolation
- **Interface Design**: Dependency inversion enables test doubles and mocking
- **Single Responsibility**: SRP makes unit tests focused and clear
- **Tell Don't Ask**: Behavior-focused design leads to better test structure

**Example Workflow**:
1. TDD red phase: Write test that requires well-designed interface
2. TDD green phase: Apply SOLID principles to create testable implementation
3. TDD refactor phase: Use structural principles to improve design
4. Design principles ensure tests remain maintainable as code evolves

```typescript
// TDD + Design Principles Example
describe('OrderProcessor', () => {
  it('should apply discount when order qualifies', () => {
    const discountCalculator = new MockDiscountCalculator();
    const orderProcessor = new OrderProcessor(discountCalculator); // DIP
    
    const order = new Order([item1, item2]);
    orderProcessor.processOrder(order); // Tell, Don't Ask
    
    expect(order.hasDiscount()).toBe(true); // SRP - Order knows its state
  });
});
```

### With Other Development Skills

**Documentalist Skill**:
- Use principles to guide API documentation structure
- Document architectural decisions using rule IDs for clarity
- Reference principle violations in technical debt documentation

**Skill Integration Best Practices**:

1. **Consistent Rule IDs**: Use structured rule IDs across all skills for cross-referencing
2. **Priority Alignment**: Focus high-priority skills on CRITICAL/HIGH principle violations
3. **Complementary Workflows**: Each skill addresses different aspects of the same quality goals
4. **Knowledge Sharing**: Principles inform architecture decisions, code reviews, and testing strategies

**Anti-Integration Patterns to Avoid**:
- Don't duplicate principle explanations across skills
- Don't create conflicting guidance between skills
- Don't ignore principle violations just because other skills don't catch them
- Don't apply principles in isolation without considering architectural context

---

## Further Resources

### Essential Books

**Architecture & Design Principles**:
- **Clean Architecture** by Robert C. Martin - Comprehensive guide to dependency direction and architectural boundaries
- **A Philosophy of Software Design** by John Ousterhout - Deep vs shallow modules, complexity management
- **Domain-Driven Design** by Eric Evans - Strategic design, bounded contexts, and domain modeling
- **Patterns of Enterprise Application Architecture** by Martin Fowler - Enterprise patterns and layer organization

**SOLID Principles & Object-Oriented Design**:
- **Clean Code** by Robert C. Martin - Code quality fundamentals and naming conventions
- **Design Patterns** by Gang of Four - Essential patterns for flexible, reusable code
- **Effective Java** by Joshua Bloch - Java-specific but universally applicable design principles
- **Object-Oriented Software Construction** by Bertrand Meyer - Deep dive into OOP principles

**Refactoring & Code Quality**:
- **Refactoring** by Martin Fowler - Systematic approach to improving code structure
- **Working Effectively with Legacy Code** by Michael Feathers - Applying principles to existing codebases
- **Code Complete** by Steve McConnell - Construction practices and quality metrics

### Essential Online Resources

**Martin Fowler's Articles**:
- [Refactoring.com](https://refactoring.com) - Refactoring catalog and techniques
- [Martin Fowler's Blog](https://martinfowler.com) - Architecture patterns, design principles
- [Dependency Inversion Principle](https://martinfowler.com/articles/dipInTheWild.html) - Real-world DIP application

**Uncle Bob's Resources**:
- [Clean Code Blog](https://blog.cleancoder.com) - Regular insights on software craftsmanship
- [Clean Architecture Blog Series](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html) - Architecture principles in detail
- [SOLID Principles Explained](https://blog.cleancoder.com/uncle-bob/2020/10/18/Solid-Relevance.html) - Modern relevance of SOLID

**Design Pattern Resources**:
- [Refactoring Guru](https://refactoring.guru) - Interactive design patterns with examples
- [Source Making](https://sourcemaking.com) - Patterns, anti-patterns, and refactoring
- [Gang of Four Patterns](https://springframework.guru/gang-of-four-design-patterns/) - Modern implementations

**Functional Programming Perspectives**:
- [Elixir School](https://elixirschool.com) - Functional programming design patterns
- [Learn You a Haskell](http://learnyouahaskell.com) - Pure functional design principles
- [Domain Modeling Made Functional](https://fsharpforfunandprofit.com) - F# functional design

### Conferences & Talks

**Essential Conference Talks**:
- "Clean Architecture and Design" by Robert C. Martin (NDC)
- "Functional Core, Imperative Shell" by Gary Bernhardt
- "Simple Made Easy" by Rich Hickey
- "The Language of the System" by Rich Hickey

### Academic Papers

**Foundational Papers**:
- "On the Criteria to Be Used in Decomposing Systems into Modules" by David Parnas
- "The Law of Demeter" by Ian Holland
- "Design Principles and Design Patterns" by Robert C. Martin

### Tools for Design Quality

**Static Analysis**:
- **SonarQube** - Multi-language design principle violation detection
- **NDepend** (.NET) - Architecture and design metrics
- **Detekt** (Kotlin) - Design smell detection

**Architecture Documentation**:
- **C4 Model** - Simple architecture diagrams
- **PlantUML** - Text-based architecture diagrams  
- **Mermaid** - Markdown-compatible diagrams

---

## Version History

- v1.0.0 (2026-02-11): Initial aggregated skill combining clean-architecture, architecture-design, solid-principles, structural-design-principles, and clean-code-principles
