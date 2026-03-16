---
name: solid-principles
description: Apply SOLID principles (SRP, OCP, LSP, ISP, DIP) for tactical class design, method extraction, and interface definition. Use when reviewing class design, refactoring code for maintainability, or designing interfaces and abstractions.
---

# SOLID Principles

Tactical design principles for class-level decisions, interface design, and dependency management.

## When to Use

- Reviewing class design for single responsibility violations
- Evaluating interface design and client dependencies
- Refactoring code with tight coupling or poor testability
- Designing new classes, methods, or abstractions
- Assessing whether to extend or modify existing code

## When Not to Use

- Architectural decisions about service boundaries or module structure (use clean-architecture)
- Choosing structural design patterns (use design-patterns)
- Configuring testing strategies (use testable-design)

## SOLID Checklist

Apply during code review or design:

| Principle | Check | Refactor Signal |
| --- | --- | --- |
| **SRP** | Does this class have one reason to change? | Multiple concerns → split into focused classes |
| **OCP** | Can you extend behavior without modifying existing code? | Adding features requires editing stable code → use abstraction |
| **LSP** | Do subtypes preserve base behavior contracts? | Subtype breaks parent assumptions → redesign hierarchy |
| **ISP** | Do clients depend only on methods they use? | Clients ignore many interface methods → split interface |
| **DIP** | Do you depend on abstractions, not concrete details? | Direct concrete coupling → introduce interface/port |

## Workflow

### Step 1: Identify SRP Violations

**Output:** List of classes with multiple reasons to change.

Ask for each class:

- Does it handle more than one concern (e.g., business logic + persistence + notifications)?
- Would changes to different features require editing this class?

**Example:**

```text
SRP violation: UserService handles auth, persistence, and notifications.
Refactor: Extract NotificationService and PersistenceGateway.
Validation: Each class now has one reason to change.
```

### Step 2: Check Open-Closed Compliance

**Output:** List of areas where new features require modifying stable code.

Ask:

- Do new features require editing existing code?
- Could abstraction/polymorphism eliminate the modification?

**Example:**

```text
OCP violation: Adding payment processor requires editing PaymentService.
Refactor: Extract PaymentProcessor interface + implementations.
Validation: New processors added without touching existing code.
```

### Step 3: Validate Liskov Substitution

**Output:** Subtypes that break base type contracts.

Check:

- Do subtypes strengthen preconditions or weaken postconditions?
- Do clients need to check types before using them?

**Example:**

```text
LSP violation: CachedRepository.save() throws exception when cache is full.
Refactor: Base Repository contract promises no exceptions; use Result type instead.
```

### Step 4: Segregate Interfaces

**Output:** Interfaces split by client needs.

Check:

- Do clients depend on methods they never call?
- Are there multiple unrelated method groups in one interface?

**Example:**

```text
ISP violation: IUserRepository has save(), find(), and sendNotification().
Refactor: Split into IUserRepository and INotificationService.
```

### Step 5: Invert Dependencies

**Output:** Abstractions introduced at module boundaries.

Check:

- Do high-level modules depend on low-level modules?
- Are concrete types instantiated in business logic?

**Example:**

```text
DIP violation: OrderService instantiates PostgresRepository.
Refactor: OrderService depends on IOrderRepository interface.
Injection: Container provides PostgresRepository implementation.
```

## Quick Commands

```bash
# Find SRP violations (multiple concerns)
rg -n "class.*Service.*Repository|class.*Manager.*Handler" src
```

```bash
# Find OCP violations (type checks)
rg -n "instanceof|typeof.*==|switch.*type" src
```

```bash
# Find DIP violations (concrete instantiation)
rg -n "new [A-Z].*Repository\(|new [A-Z].*Service\(" src
```

## Anti-Patterns

### NEVER apply SOLID as a mechanical checklist to every class

- **WHY**: SOLID principles are design guidelines, not mandates; applying them without judgment creates unnecessary abstraction layers and over-engineered code in simple domains where change pressure does not exist.
- **BAD**: Extract an interface for every class because "you should always program to interfaces" (misapplied DIP), resulting in one-implementation interfaces that add indirection with no benefit.
- **GOOD**: Apply SOLID principles where the design is causing pain — tight coupling, frequent change in one direction, difficulty testing — not as a universal rule regardless of context.

### NEVER create God Classes to avoid "too many files"

- **WHY**: Condensing multiple responsibilities into one class saves file count but creates high coupling, low cohesion, and fragile code where unrelated changes break each other.
- **BAD**: A `UserManager` class that handles authentication, profile editing, email notifications, and audit logging — violating SRP by having four distinct reasons to change.
- **GOOD**: Split into `AuthService`, `UserProfileService`, `NotificationService`, and `AuditLog` with clear single responsibilities and independent change trajectories.

### NEVER confuse ISP with "one method per interface"

- **WHY**: The Interface Segregation Principle says clients should not depend on methods they do not use; it does not mean every interface must have exactly one method, which leads to over-granularization and interface explosion.
- **BAD**: Creating dozens of single-method interfaces for every operation (`ICanSave`, `ICanFind`, `ICanDelete`) that are always implemented and injected together.
- **GOOD**: Group cohesive method sets into role-specific interfaces that represent a single role or behavior contract used by a specific type of client.

### NEVER use deep inheritance to satisfy OCP when composition achieves the same result

- **WHY**: Deep inheritance hierarchies are fragile; subclass changes ripple up the chain, LSP violations are common, and the Open/Closed Principle is better achieved through composition and dependency injection.
- **BAD**: `class PremiumUser extends AuthenticatedUser extends User extends BaseUser` to add behavior, creating a four-level hierarchy where changing any layer risks breaking all subclasses.
- **GOOD**: Inject behavior through composition: `new User(new PremiumFeatureSet())` — open for extension by adding new feature sets, closed for modification of the `User` class itself.

## References

SOLID principles are tactical class-level design rules. For architectural decisions (boundaries, modules, dependencies), use the clean-architecture skill.

For detailed SOLID guidance, see the software-design-principles hub (references/detailed-examples.md, references/anti-patterns-and-frameworks.md).
- [Martin Fowler on SOLID](https://martinfowler.com/bliki/BeckDesignRules.html)
- [Uncle Bob SOLID Papers](https://fi.ort.edu.uy/innovaportal/file/2032/1/design_principles.pdf)
