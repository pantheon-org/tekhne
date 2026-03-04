---
name: design-patterns
description: Select and apply structural design patterns (Strategy, Factory, Adapter, Observer, etc.) to reduce complexity in class design. Use when evaluating whether a pattern solves a concrete problem, refactoring conditional logic, dealing with too many if/else statements or code smells, resolving inheritance problems, or deciding how to structure classes.
---

# Design Patterns

Structural design patterns for solving recurring design problems with proven solutions.

## When to Use

- Eliminating complex conditional logic (if/else chains, switch statements)
- Designing plugin/extension systems
- Integrating external APIs or legacy systems
- Separating concerns with clear interfaces
- Evaluating whether a pattern solves a concrete problem

## When Not to Use

- Class-level SOLID violations (use solid-principles)
- Architectural boundary decisions (use clean-architecture)
- Testing strategy configuration (use testable-design)
- **NEVER apply patterns preemptively for imagined future needs**

## Pattern Selection Workflow

### Step 1: Identify the Design Problem

**Output:** Concrete problem statement without pattern jargon.

Ask:

- What conditional logic or coupling exists?
- What needs to vary independently?
- What external system needs integration?

**Example:**

```text
Problem: Payment processing logic uses if/else chains to select processors.
Adding new processors requires editing existing code (OCP violation).
```

### Step 2: Evaluate Pattern Fit

**Output:** Pattern choice with explicit win condition.

Ask before choosing a pattern:

- Does this pattern solve the concrete problem?
- What complexity does it remove vs. add?
- Can the team maintain it?

**Example:**

```text
Pattern: Strategy (not Factory).
Win condition: Eliminates if/else chains for payment processor selection.
Cost: One interface + implementations. Benefit: Easy to add new processors.
```

### Step 3: Document Pattern Application

**Output:** Pattern applied with before/after comparison.

Template:

```text
Pattern: Strategy
Problem: if/else chain for payment processor selection
Solution: PaymentProcessor interface + implementations (Stripe, PayPal, etc.)
Validation: New processors added without editing existing code
```

## Common Patterns

### Strategy Pattern

**Use when:** Conditional logic selects between algorithms.

**Example:**

```typescript
// Before: if/else chain
if (type === 'stripe') {
  // Stripe logic
} else if (type === 'paypal') {
  // PayPal logic
}

// After: Strategy pattern
interface PaymentProcessor {
  process(amount: number): Promise<Receipt>
}

class StripeProcessor implements PaymentProcessor { /* ... */ }
class PayPalProcessor implements PaymentProcessor { /* ... */ }

// Select strategy
const processor = processors[type]
await processor.process(amount)
```

### Factory Pattern

**Use when:** Object creation logic is complex or conditional.

**Example:**

```typescript
// Before: constructor with complex logic
class Report {
  constructor(type: string) {
    if (type === 'pdf') {
      // PDF setup
    } else if (type === 'csv') {
      // CSV setup
    }
  }
}

// After: Factory pattern
interface Report { generate(): string }

class ReportFactory {
  create(type: string): Report {
    switch (type) {
      case 'pdf': return new PdfReport()
      case 'csv': return new CsvReport()
      default: throw new Error('Unknown type')
    }
  }
}
```

### Adapter Pattern

**Use when:** Integrating external APIs or legacy systems with incompatible interfaces.

**Example:**

```typescript
// External API (incompatible interface)
class LegacyEmailService {
  sendMail(to: string, subject: string, body: string): void { /* ... */ }
}

// Domain interface
interface IEmailService {
  send(email: Email): Promise<void>
}

// Adapter
class EmailServiceAdapter implements IEmailService {
  constructor(private legacy: LegacyEmailService) {}
  
  async send(email: Email): Promise<void> {
    this.legacy.sendMail(email.to, email.subject, email.body)
  }
}
```

### Observer Pattern

**Use when:** Multiple components need to react to state changes.

**Example:**

```typescript
interface Observer {
  update(event: Event): void
}

class Subject {
  private observers: Observer[] = []
  
  attach(observer: Observer): void {
    this.observers.push(observer)
  }
  
  notify(event: Event): void {
    this.observers.forEach(o => o.update(event))
  }
}
```

### Anti-Corruption Layer

**Use when:** Integrating with external systems that should not pollute your domain.

**Example:**

```typescript
// External API with poor design
interface ExternalUserAPI {
  getUserData(id: number): { usr_nm: string, eml: string }
}

// Anti-Corruption Layer
class UserGateway {
  constructor(private api: ExternalUserAPI) {}
  
  async getUser(id: string): Promise<User> {
    const data = this.api.getUserData(Number(id))
    return new User({ name: data.usr_nm, email: data.eml })
  }
}
```

### Humble Object Pattern

**Use when:** Separating testable logic from hard-to-test infrastructure.

**Example:**

```typescript
// Humble object (hard to test, minimal logic)
class HttpController {
  constructor(private useCase: CreateOrderUseCase) {}
  
  async handle(req: Request, res: Response): Promise<void> {
    const input = this.mapRequest(req)
    const output = await this.useCase.execute(input)
    res.json(output)
  }
}

// Testable logic (easy to test, no infrastructure)
class CreateOrderUseCase {
  async execute(input: CreateOrderInput): Promise<CreateOrderOutput> {
    // Business logic here
  }
}
```

## Anti-Patterns

### NEVER apply patterns preemptively

**BAD:** Add Factory pattern "in case" we need multiple implementations.  
**GOOD:** Wait for concrete need (second implementation) before extracting pattern.

### NEVER use patterns to hide complexity

**BAD:** Wrap simple logic in Strategy/Factory/Builder for "future flexibility."  
**GOOD:** Start simple; refactor to pattern when complexity justifies it.

### NEVER cargo-cult patterns from other codebases

**BAD:** Copy design patterns because they "look professional."  
**GOOD:** Apply patterns to solve concrete problems in your codebase.

### NEVER optimize before measurement

**BAD:** Add cache or pool because function "might be slow."  
**GOOD:** Measure baseline, optimize when threshold is exceeded.

## Quick Commands

```bash
# Find Strategy/Factory candidates (conditional logic)
rg -n "if.*type.*==|switch.*\(type\)|instanceof" src
```

```bash
# Find Adapter candidates (external API usage)
rg -n "import.*from.*external|import.*sdk" src
```

```bash
# Find Observer candidates (event handling)
rg -n "addEventListener|on\(|emit\(|dispatch\(" src
```

## Quick Reference

### Pattern Categories

**Creational:** Control object creation (Factory, Builder, Singleton)  
**Structural:** Compose objects and classes (Adapter, Decorator, Facade)  
**Behavioral:** Manage algorithms and responsibilities (Strategy, Observer, Command)

### Pattern Reference Files

- Anti-corruption layer: [references/adapt-anti-corruption-layer.md](references/adapt-anti-corruption-layer.md)
- Anti-patterns: [references/anti-patterns-and-frameworks.md](references/anti-patterns-and-frameworks.md)
- Humble object: [references/bound-humble-object.md](references/bound-humble-object.md)
- Partial boundaries: [references/bound-partial-boundaries.md](references/bound-partial-boundaries.md)
- Interface ownership: [references/dep-interface-ownership.md](references/dep-interface-ownership.md)
- Stable abstractions: [references/dep-stable-abstractions.md](references/dep-stable-abstractions.md)
- Detailed examples: [references/detailed-examples.md](references/detailed-examples.md)
- Logging abstraction: [references/frame-logging-abstraction.md](references/frame-logging-abstraction.md)
- Main component: [references/bound-main-component.md](references/bound-main-component.md)
- Common closure: [references/comp-common-closure.md](references/comp-common-closure.md)
- Reuse-release equivalence: [references/comp-reuse-release-equivalence.md](references/comp-reuse-release-equivalence.md)

## References

- [Design Patterns (Gang of Four)](https://en.wikipedia.org/wiki/Design_Patterns)
- [Refactoring Guru Design Patterns](https://refactoring.guru/design-patterns)
- [Martin Fowler Catalog](https://martinfowler.com/eaaCatalog/)
