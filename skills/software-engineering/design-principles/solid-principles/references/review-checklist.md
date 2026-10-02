# SOLID Review Checklist

Use this reference when running a SOLID review step by step. Each step lists the questions to ask, the signal that a refactor is needed, and the output to record.

## Output format

Record each finding in three lines:

```text
<Principle> violation: <what is wrong and where>.
Refactor: <the change>.
Validation: <how to confirm the problem is gone>.
```

## SRP: one reason to change

- Does the class handle more than one concern, such as business logic, persistence and notifications?
- Would changes to different features require editing this class?
- Signal: multiple concerns in one class. Split into focused classes.

## OCP: extend without modifying

- Do new features require editing existing code?
- Could abstraction or polymorphism remove the edit?
- Signal: adding a feature edits stable code. Introduce an abstraction and add implementations.
- Prefer composition and injection over deep inheritance to achieve extension.

## LSP: substitutable subtypes

- Do subtypes strengthen preconditions or weaken postconditions?
- Do clients need to check types before using a value?
- Signal: a subtype breaks parent assumptions. Redesign the hierarchy, or return a Result type where the base contract promises no exceptions.

## ISP: client-shaped interfaces

- Do clients depend on methods they never call?
- Does one interface hold several unrelated method groups?
- Signal: clients ignore many methods. Split by client role, but keep cohesive method sets together rather than one method per interface.

## DIP: depend on abstractions

- Do high-level modules depend on low-level modules?
- Are concrete types instantiated inside business logic?
- Signal: direct concrete coupling. Introduce an interface or port and inject the implementation.

## Candidate searches

These searches find candidates only. Inspect each hit before calling it a violation.

```bash
rg -n "class.*Service.*Repository|class.*Manager.*Handler" src
rg -n "instanceof|typeof.*==|switch.*type" src
rg -n "new [A-Z].*Repository\(|new [A-Z].*Service\(" src
```

## Judgement

Apply SOLID where the design causes pain: tight coupling, frequent change in one direction, or difficulty testing. Do not extract an interface for every class.
