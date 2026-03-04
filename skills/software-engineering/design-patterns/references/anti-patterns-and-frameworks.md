# Anti-Patterns and Self-Assessment Frameworks

This file contains systematic NEVER lists and self-assessment frameworks for software design principles.

## Table of Contents
- [Dependency Direction Anti-Patterns](#dependency-direction-anti-patterns)
- [Entity Design Anti-Patterns](#entity-design-anti-patterns)
- [SOLID Principles Anti-Patterns](#solid-principles-anti-patterns)
- [Self-Assessment Frameworks](#self-assessment-frameworks)

## Dependency Direction Anti-Patterns

### NEVER DO - Dependency Direction

1. **NEVER import framework code in domain entities** because it couples business rules to volatile implementation details and makes testing impossible
2. **NEVER import infrastructure in application layer** because it violates the dependency inversion principle and makes the app untestable
3. **NEVER create circular dependencies between modules** because it creates tight coupling and deployment issues
4. **NEVER depend on concrete implementations from higher layers** because it violates dependency inversion and reduces flexibility
5. **NEVER import UI components in business logic** because it couples domain rules to presentation concerns
6. **NEVER import database schemas in domain entities** because it couples business logic to data storage details
7. **NEVER import HTTP clients in domain services** because it couples business logic to communication protocols
8. **NEVER import configuration in domain entities** because it makes business rules dependent on deployment details

## Entity Design Anti-Patterns

### NEVER DO - Entity Design

1. **NEVER create anemic entities with only getters/setters** because it scatters business logic across services and violates encapsulation
2. **NEVER expose entity internals through public fields** because it breaks encapsulation and allows invalid state changes
3. **NEVER put infrastructure concerns in entities** because it violates single responsibility and makes testing hard
4. **NEVER create entities without invariant validation** because it allows invalid business states to persist
5. **NEVER use primitive types for domain concepts** because it allows type mismatches and loses business meaning
6. **NEVER create entities that know about persistence** because it couples business logic to data storage mechanisms
7. **NEVER mix entity validation with UI validation** because business rules become dependent on presentation layer
8. **NEVER create god entities with multiple responsibilities** because it violates SRP and becomes unmaintainable

## SOLID Principles Anti-Patterns

### Single Responsibility Principle (SRP)

#### NEVER DO - Single Responsibility

1. **NEVER create classes that have multiple reasons to change** because changes become risky and affect unrelated functionality
2. **NEVER mix business logic with data access** because it couples domain rules to storage mechanisms  
3. **NEVER combine validation with persistence** because rule changes affect database operations unnecessarily
4. **NEVER put UI logic in business classes** because presentation changes break business functionality
5. **NEVER mix error handling with business logic** because error formatting becomes coupled to domain rules
6. **NEVER combine configuration with business operations** because deployment changes affect business behavior
7. **NEVER put logging in domain entities** because infrastructure concerns pollute business logic
8. **NEVER create service classes that do everything** because they become unmaintainable god objects

### Open/Closed Principle (OCP)

#### NEVER DO - Open/Closed Principle

1. **NEVER use switch statements for extensible behavior** because adding new cases requires modifying existing code
2. **NEVER modify existing classes to add new features** because it risks breaking existing functionality
3. **NEVER hard-code type checks for polymorphic behavior** because new types require code changes everywhere
4. **NEVER create if-else chains for strategy selection** because new strategies require modifying selection logic
5. **NEVER put feature flags directly in business logic** because new features modify core business code
6. **NEVER create monolithic classes that handle multiple scenarios** because new scenarios break existing code

### Interface Segregation Principle (ISP)

#### NEVER DO - Interface Segregation

1. **NEVER create fat interfaces with many methods** because clients depend on methods they don't use
2. **NEVER force clients to implement unused methods** because it violates interface segregation
3. **NEVER create one-size-fits-all interfaces** because different clients have different needs
4. **NEVER put optional methods in required interfaces** because not all clients need all functionality

### Dependency Inversion Principle (DIP)

#### NEVER DO - Dependency Inversion

1. **NEVER depend on concrete classes in high-level modules** because it creates tight coupling to implementation details
2. **NEVER instantiate dependencies directly** because it makes testing and flexibility impossible
3. **NEVER import low-level modules in high-level policies** because it violates the dependency rule
4. **NEVER use global variables for dependency access** because it creates hidden dependencies and testing issues

## Self-Assessment Frameworks

### Dependency Direction Self-Assessment

**Ask yourself these questions when reviewing dependency relationships:**

1. **Does this import violate the dependency rule?** (Are dependencies pointing inward toward the domain?)
2. **Would this code still work if I swapped the implementation?** (Am I depending on abstractions?)
3. **Can I test this code without the infrastructure?** (Are my dependencies invertible?)
4. **Does this create a circular dependency?** (Can I build these modules independently?)
5. **Would changing the database/framework break this code?** (Am I coupled to implementation details?)
6. **Can I deploy these modules separately?** (Are my boundaries clean?)
7. **Does this make the build order fragile?** (Are my compilation dependencies stable?)

### Entity Design Self-Assessment

**Ask yourself these questions when designing or reviewing entities:**

1. **Does this entity contain the business logic that operates on its data?** (Rich vs Anemic)
2. **Can this entity get into an invalid state?** (Are invariants enforced?)
3. **Does this entity depend on infrastructure concerns?** (Is it pure domain logic?)
4. **Would a business expert recognize these methods and properties?** (Is it expressed in domain language?)
5. **Can I create this entity in different valid states?** (Are there proper construction methods?)
6. **Does changing business rules require changing only this entity?** (Is the logic encapsulated?)

### Single Responsibility Self-Assessment

**Ask yourself these questions when evaluating class responsibility:**

1. **How many different reasons would cause me to change this class?** (Should be only one)
2. **Can I describe what this class does in a single, simple sentence?** (Without using "and" or "or")  
3. **Do the methods in this class all operate on related data?** (Cohesion check)
4. **Would different stakeholders want to change different parts of this class?** (Separation of concerns)
5. **Can I extract any behavior into a separate class without losing meaning?** (Is it doing too much?)
6. **Do I need to understand multiple domains to understand this class?** (Cross-cutting concerns check)

### Open/Closed Self-Assessment

**Ask yourself these questions about extensibility:**

1. **If I need to add a new feature, do I have to modify existing code?** (Open for extension, closed for modification)
2. **Can I add new behavior through composition or inheritance?** (Extension mechanisms available?)
3. **Are there switch statements or if-else chains that grow with new requirements?** (Strategy pattern needed?)
4. **Do I have abstractions that allow different implementations?** (Plugin points exist?)
5. **Would adding new functionality risk breaking existing features?** (Stability of existing code)

### Interface Segregation Self-Assessment

**Ask yourself these questions about interface design:**

1. **Do all clients use all methods in this interface?** (No forced dependencies on unused methods)
2. **Would different types of clients benefit from different interfaces?** (Client-specific interfaces)
3. **Are there methods that only some implementations can provide?** (Interface bloat check)
4. **Can I group related methods into smaller, focused interfaces?** (Cohesive interfaces)

### Dependency Inversion Self-Assessment

**Ask yourself these questions about dependencies:**

1. **Do my high-level modules depend on low-level modules?** (Should depend on abstractions)
2. **Can I easily swap implementations for testing?** (Dependency injection working?)
3. **Do I have circular dependencies between layers?** (Clean architecture boundaries)
4. **Are my dependencies pointing in the same direction as my architecture?** (Consistent dependency flow)

These frameworks help guide decision-making and catch violations before they become problematic.