# Project Boundaries

Use this reference to enforce dependency constraints and ownership boundaries.

## Tagging Strategy

Define a small stable vocabulary, for example:

- `type:app`, `type:feature`, `type:ui`, `type:data-access`, `type:util`
- `scope:web`, `scope:api`, `scope:shared`

## Boundary Enforcement

Use `@nx/enforce-module-boundaries` with `depConstraints` to encode rules.

Principles:

- apps depend downward on feature/data/ui/util layers.
- util libraries remain dependency sinks.
- scopes import within scope unless explicitly shared.

## Rule Hygiene

- Keep constraints understandable; avoid overfitting edge cases.
- Add tests or intentional-violation checks during rule changes.
- Update tag docs whenever the taxonomy changes.
