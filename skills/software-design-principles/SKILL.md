---
name: software-design-principles
description: Comprehensive software design guidance covering strategic architecture decisions, tactical implementation principles (SOLID, structural patterns), and code quality fundamentals. Use when reviewing code for SOLID violations, refactoring legacy code, designing clean architecture, assessing technical debt, debugging architectural issues, preventing code rot, or making design decisions for GraphQL resolvers, React components, enterprise applications, microservices, and domain models. Essential for code reviews, architecture planning, API design, system design, technical debt management, and creating maintainable, testable, and flexible software systems.
---

# Software Design Principles

Comprehensive guidance for software design covering strategic architecture decisions, tactical implementation principles, and code quality fundamentals. This skill provides systematic approaches to creating maintainable, testable, and flexible software systems.

## Skill Organization

This skill is organized into three progressive layers:
1. **Strategic Design**: System-level architecture and boundaries (CRITICAL)
2. **Tactical Design**: Component-level principles and patterns (HIGH)  
3. **Code Quality**: Implementation-level practices (MEDIUM)

## Rule Classification System

| Category | Rules | Impact | Prefix | Description |
|----------|-------|--------|--------|-------------|
| **Dependency Rules** | 6 rules | CRITICAL | `dep-` | Dependency direction, acyclic dependencies, stable abstractions |
| **Entity Design** | 5 rules | CRITICAL | `entity-` | Pure business rules, rich entities, invariant encapsulation |
| **Use Case Isolation** | 5 rules | HIGH | `usecase-` | Single responsibility, explicit dependencies, transaction boundaries |
| **SOLID Principles** | 5 rules | HIGH | `solid-` | SRP, OCP, LSP, ISP, DIP implementation |
| **Component Cohesion** | 5 rules | MEDIUM | `comp-` | Screaming architecture, common closure principle |
| **Boundary Definition** | 6 rules | MEDIUM | `bound-` | Humble objects, anti-corruption layers, partial boundaries |
| **Framework Isolation** | 5 rules | MEDIUM | `frame-` | Domain purity, ORM placement, plugin architecture |
| **Interface Adapters** | 5 rules | MEDIUM | `adapt-` | Controller thinness, presenter patterns, gateway abstractions |
| **Structural Design** | 4 rules | MEDIUM | `struct-` | Composition over inheritance, Law of Demeter, Tell Don't Ask |
| **Core Principles** | 3 rules | LOW | `core-` | DRY, KISS, YAGNI applications |
| **Testing Architecture** | 4 rules | LOW | `test-` | Boundary verification, test doubles, integration testing |

## Reference Loading Guidelines

### ⚠️ MANDATORY Loading Triggers

**CRITICAL Violations - Always Load:**
- Before applying `dep-inward-only` → READ `references/dep-inward-only.md`
- Before applying `entity-pure-business-rules` → READ `references/entity-pure-business-rules.md`
- Before applying `dep-acyclic-dependencies` → READ `references/dep-acyclic-dependencies.md`
- Before applying `entity-rich-not-anemic` → READ `references/entity-rich-not-anemic.md`

**HIGH Priority Violations - Load for Complex Cases:**
- Before applying `solid-srp` → READ `references/solid-srp.md`
- Before applying `usecase-single-responsibility` → READ `references/usecase-single-responsibility.md`

### ❌ Do NOT Load Guidelines

**Context Bloat Prevention Rules:**
- Load only one reference file at a time
- Confirm violation exists before loading detailed reference
- Skip loading if you're familiar with the rule
- For MEDIUM/LOW priority rules, apply from memory first

**When to Skip Loading:**
- Routine violations (basic SRP, obvious coupling)
- Simple refactoring tasks (renaming, extracting methods)
- Code reviews for basic violations
- Time-constrained analysis

## Output Format Specifications

### Standard Format
```
file:line - [rule-id] Description

Example:
src/domain/user.ts:15 - [dep-inward-only] Domain entity imports infrastructure Database class
src/services/user-service.ts:42 - [solid-srp] UserService handles validation, persistence, and email sending
```

### Priority-Based Format
```
🔴 CRITICAL: file:line - [rule-id] Description
🟠 HIGH: file:line - [rule-id] Description  
🟡 MEDIUM: file:line - [rule-id] Description
```

### Multi-Line Violations
```
src/components/user-dashboard.tsx:25-67 - [solid-srp] 
Component violates SRP:
- Data fetching (lines 25-32)
- Business logic (lines 33-45)  
- Analytics tracking (lines 46-52)
- Error handling (lines 53-67)
```

---

## Decision Trees for Complex Scenarios

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

---

## Part 1: Strategic Design (System Architecture)

Strategic design focuses on high-level system boundaries and dependency management.

### 🔴 Dependency Direction [CRITICAL]

**Rule IDs**: `dep-inward-only`, `dep-acyclic-dependencies`, `dep-stable-abstractions`, `dep-interface-ownership`, `dep-no-framework-imports`

**⚠️ BEFORE APPLYING** - Read references for specific violations:
- Before applying `dep-inward-only` → READ `references/dep-inward-only.md`
- Before applying `dep-acyclic-dependencies` → READ `references/dep-acyclic-dependencies.md`

**Core Principle**: Dependencies must point inward toward the domain. Infrastructure depends on domain, never the reverse.

**Quick Check**: Can you test your business logic without the database, web framework, or external services?

**📖 For detailed examples**: See `references/detailed-examples.md#dependency-direction-examples`
**🚫 For anti-patterns**: See `references/anti-patterns-and-frameworks.md#dependency-direction-anti-patterns`

### 🔴 Entity Design [CRITICAL]

**Rule IDs**: `entity-pure-business-rules`, `entity-rich-not-anemic`, `entity-encapsulate-invariants`, `entity-value-objects`, `entity-domain-services`

**⚠️ BEFORE APPLYING** - Read references for specific violations:
- Before applying `entity-pure-business-rules` → READ `references/entity-pure-business-rules.md`
- Before applying `entity-rich-not-anemic` → READ `references/entity-rich-not-anemic.md`

**Core Principle**: Entities contain business logic and enforce business invariants. No anemic data containers.

**Quick Check**: Does your entity contain the business rules that operate on its data?

**📖 For detailed examples**: See `references/detailed-examples.md#entity-design-examples`
**🚫 For anti-patterns**: See `references/anti-patterns-and-frameworks.md#entity-design-anti-patterns`

### 🟠 Use Case Isolation [HIGH]

**Rule IDs**: `usecase-single-responsibility`, `usecase-explicit-dependencies`, `usecase-orchestrates-not-implements`

**Core Principle**: Use cases orchestrate business operations without implementing business rules.

**Quick Check**: Does each use case have a single, clear business purpose?

---

## Part 2: Tactical Design (Component-Level Principles)

Tactical design applies to individual components, classes, and modules.

### SOLID Principles [HIGH]

#### 🟠 Single Responsibility Principle (SRP)

**Rule ID**: `solid-srp`

**⚠️ BEFORE APPLYING** - For complex violations:
- Before applying `solid-srp` → READ `references/solid-srp.md`

**Principle**: A class should have only one reason to change.

**Quick Check**: Can you describe what this class does in one sentence without using "and"?

**📖 For detailed examples**: See `references/detailed-examples.md#solid-principles-examples`
**🚫 For anti-patterns**: See `references/anti-patterns-and-frameworks.md#solid-principles-anti-patterns`

#### 🟠 Open/Closed Principle (OCP)

**Rule ID**: `solid-ocp`

**Principle**: Open for extension, closed for modification.

**Quick Check**: Can you add new features without modifying existing code?

#### 🟠 Interface Segregation Principle (ISP)

**Rule ID**: `solid-isp`

**Principle**: Clients should not depend on interfaces they don't use.

**Quick Check**: Do all clients use all methods in your interfaces?

#### 🟠 Dependency Inversion Principle (DIP)

**Rule ID**: `solid-dip`

**Principle**: Depend on abstractions, not concretions.

**Quick Check**: Can you easily swap implementations for testing?

### Structural Design Principles [MEDIUM]

#### 🟡 Composition Over Inheritance

**Rule ID**: `struct-composition-over-inheritance`

**Principle**: Prefer object composition over class inheritance.

#### 🟠 Law of Demeter

**Rule ID**: `struct-law-of-demeter`

**Principle**: Only talk to your immediate friends.

#### 🟠 Tell, Don't Ask

**Rule ID**: `struct-tell-dont-ask`

**Principle**: Tell objects what to do, don't ask for their state.

---

## Part 3: Code Quality Fundamentals

Implementation-level practices for clean, maintainable code.

### Core Principles [LOW]

- **DRY** (`core-dry`): Don't Repeat Yourself
- **KISS** (`core-kiss`): Keep It Simple, Stupid  
- **YAGNI** (`core-yagni`): You Aren't Gonna Need It

### Design Patterns [LOW]

Essential patterns for common design problems:
- **Factory** (`pattern-factory`): Object creation
- **Strategy** (`pattern-strategy`): Algorithm selection
- **Repository** (`pattern-repository`): Data access abstraction
- **Observer** (`pattern-observer`): Event handling

---

## Error Handling and Fallback Guidance

### Common Failure Scenarios

#### Reference Loading Failures
**Problem**: Cannot load required reference file or context limit reached
**Fallback**: Apply rule ID from memory, use decision trees, focus on CRITICAL rules only

#### Conflicting Principles
**Problem**: Two principles contradict in specific context
**Resolution Priority**: CRITICAL > HIGH > MEDIUM > LOW, Entity Design > Tactical Design

#### Legacy System Complications
**Problem**: Modern principles seem impossible to apply
**Pragmatic Fallback**: Grandfather existing violations, focus on preventing new ones, use anti-corruption layers

### Recovery Strategies

- **When stuck**: Use decision trees, step back to higher level, apply 80/20 rule
- **Too complex**: Prioritize by impact, group related violations, provide implementation order
- **Insufficient context**: State assumptions clearly, request specific information, provide conditional guidance

---

## Integration with Other Skills

### With code-reviewer
- Use rule IDs for consistent violation reporting
- Apply decision trees for review prioritization
- Focus on CRITICAL violations first

### With architecture-design  
- Strategic design principles guide system boundaries
- Use dependency direction for clean architecture
- Entity design informs domain modeling

### With test-driven-development
- Design principles improve testability
- Dependency inversion enables test doubles
- Entity design supports isolated unit tests

---

## Agent Strategy Documentation

### 1. Architecture Review Strategy
**Purpose**: Evaluate system architecture for principle violations
**Approach**: Start with dependency direction, then entity design, finally tactical principles
**Output**: Prioritized violation list with remediation steps

### 2. Code Quality Audit Strategy  
**Purpose**: Systematic codebase assessment
**Approach**: Apply decision trees, focus on high-impact violations
**Output**: Technical debt assessment with priority rankings

### 3. Refactoring Guidance Strategy
**Purpose**: Guide incremental improvements
**Approach**: Use crisis triage for urgent fixes, progressive disclosure for planning
**Output**: Step-by-step refactoring plan with safety measures

### 4. Test-Driven Design Strategy
**Purpose**: Design components for testability
**Approach**: Apply dependency inversion and entity design principles
**Output**: Testable design with clear boundaries

### 5. Real-Time Code Review Strategy
**Purpose**: Live development feedback
**Approach**: Quick violation detection using rule IDs
**Output**: Immediate feedback with specific improvement suggestions

---

## Further Resources

### Essential Books
- "Clean Architecture" by Robert C. Martin
- "Domain-Driven Design" by Eric Evans
- "Refactoring" by Martin Fowler
- "Design Patterns" by Gang of Four
- "Clean Code" by Robert C. Martin

### Online Resources
- Martin Fowler's Architecture Guide: https://martinfowler.com/architecture/
- Uncle Bob's Clean Code Blog: https://blog.cleancoder.com/
- Domain-Driven Design Community: https://dddcommunity.org/
- Refactoring Guru: https://refactoring.guru/

### Reference Files (Detailed Content)
- `references/detailed-examples.md` - Comprehensive code examples
- `references/anti-patterns-and-frameworks.md` - NEVER lists and self-assessment frameworks
- `references/[rule-id].md` - Specific rule documentation (42 files)

---

*This skill provides systematic guidance for creating maintainable, testable, and flexible software systems through progressive application of strategic, tactical, and code quality principles.*