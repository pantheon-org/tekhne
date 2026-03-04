# DEPRECATED: software-design-principles

**Status:** DEPRECATED (2026-03)  
**Reason:** Monolithic hub violated single-responsibility principle  
**Replacement:** 4 focused skills under `pantheon-ai/design-principles` tile

## Migration Guide

The monolithic `software-design-principles` skill (254 lines, 44 references) has been split into 4 focused, single-responsibility skills:

### 1. solid-principles
**Path:** `skills/software-engineering/solid-principles/SKILL.md`  
**Focus:** Tactical class design (SRP, OCP, LSP, ISP, DIP)  
**Use for:** Class-level refactoring, interface design, dependency management

### 2. clean-architecture
**Path:** `skills/software-engineering/clean-architecture/SKILL.md`  
**Focus:** Strategic boundary design, dependency direction, layered architecture  
**Use for:** Service boundaries, module structure, architectural decisions

### 3. design-patterns
**Path:** `skills/software-engineering/design-patterns/SKILL.md`  
**Focus:** Structural patterns (Strategy, Factory, Adapter, Observer, Humble Object, etc.)  
**Use for:** Eliminating conditional logic, integrating external systems, separating concerns

### 4. testable-design
**Path:** `skills/software-engineering/testable-design/SKILL.md`  
**Focus:** Dependency injection, layer isolation, boundary testing  
**Use for:** Test strategy, improving testability, refactoring untestable code

## Tessl Registry

**Old:** `pantheon-ai/software-design-principles`  
**New:** `pantheon-ai/design-principles` (unified tile containing 4 skills)

## Installation

```bash
# Install all 4 skills via unified tile
tessl install pantheon-ai/design-principles

# Or install specific skills (if published individually)
tessl install pantheon-ai/solid-principles
tessl install pantheon-ai/clean-architecture
tessl install pantheon-ai/design-patterns
tessl install pantheon-ai/testable-design
```

## Reference Migration

All 44 reference documents were redistributed:

- **solid-principles:** 0 references (new skill with fresh documentation)
- **clean-architecture:** 29 references (dep-*, comp-*, bound-*, frame-*, entity-*, usecase-*, adapt-*)
- **design-patterns:** 11 references (anti-patterns, patterns, abstractions, component principles)
- **testable-design:** 4 references (test-*)

## Breaking Changes

- `software-design-principles/` directory will be deleted
- Tessl registry entry `pantheon-ai/software-design-principles` deprecated
- All references updated in AGENTS.md, README.md, skill-taxonomy.md

## Timeline

- **2026-03-04:** Deprecation notice added
- **2026-03-04:** New skills published to Tessl registry
- **2026-03-11:** Original skill directory deleted (7-day grace period)
