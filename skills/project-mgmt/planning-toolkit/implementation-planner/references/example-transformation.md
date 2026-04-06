# Example Transformation

## Before (Flat Structure)

```
docs/refactoring/phases/
├── phase-1-analysis.md (400 lines)
├── phase-2-implementation.md (1029 lines)
└── phase-3-schema.md (300 lines)
```

**Problems:**
- Large files are hard to navigate
- No clear ownership boundaries
- Difficult to track progress on individual items
- Contributors must scroll through unrelated content

## After (Hierarchical Structure)

```
docs/refactoring/phases/
├── phase-1-analysis/
│   ├── README.md
│   └── activities/
│       ├── README.md
│       └── activity-1-analysis-and-design/
│           ├── README.md
│           ├── activity-1.1-current-codebase-analysis.md
│           └── activity-1.2-architecture-design.md
├── phase-2-implementation/
│   ├── README.md
│   └── steps/
│       ├── README.md
│       ├── step-1-extract-and-refactor/
│       │   ├── README.md
│       │   ├── step-1.1-extract-movement-logic.md
│       │   ├── step-1.2-network-protocol.md
│       │   ├── step-1.3-refactor-colyseus-rooms.md
│       │   └── step-1.4-refactor-client.md
│       └── step-2-create-structure/
│           ├── README.md
│           ├── step-2.1-create-libs-directories.md
│           └── step-2.2-configure-nx-boundaries.md
└── phase-3-schema/
    ├── README.md
    └── steps/
        ├── README.md
        └── step-1-define-schemas/
            ├── README.md
            ├── step-1.1-player-state.md
            └── step-1.2-room-state.md
```

**Benefits:**
- Each item has a stable URL
- Progress tracking per leaf file
- Clear ownership and dependencies
- Related items grouped together
- Maximum 4 levels deep (phase → activities/steps → group → leaf)

## README Content Example

**Phase README (phase-2-implementation/README.md):**
```markdown
# Phase 2: Implementation

Refactor the monolithic game code into the libs/ architecture defined in Phase 1.

## Steps

- [Step 1: Extract and Refactor](./steps/step-1-extract-and-refactor/)
- [Step 2: Create Structure](./steps/step-2-create-structure/)

## Dependencies

- Phase 1 analysis complete
- Architecture design approved

## Status

In Progress
```

**Step README (step-1-extract-and-refactor/README.md):**
```markdown
# Step 1: Extract and Refactor

Extract game logic from apps/ into libs/ packages.

## Items

- [1.1 Extract Movement Logic](./step-1.1-extract-movement-logic.md)
- [1.2 Network Protocol](./step-1.2-network-protocol.md)
- [1.3 Refactor Colyseus Rooms](./step-1.3-refactor-colyseus-rooms.md)
- [1.4 Refactor Client](./step-1.4-refactor-client.md)

## Acceptance Criteria

- [ ] All movement logic in libs/game-simulation/
- [ ] Network protocol in libs/network-protocol/
- [ ] Server uses shared simulation
- [ ] Client uses shared types
```

**Leaf File (step-1.1-extract-movement-logic.md):**
```markdown
# Step 1.1: Extract Movement Logic

Move player movement calculations from apps/server to libs/game-simulation.

## Description

Currently movement logic is scattered across:
- apps/server/src/rooms/GameRoom.ts
- apps/server/src/utils/movement.ts
- apps/web-client/src/player/PlayerController.ts

Extract into a single, testable module.

## Checklist

- [ ] Create libs/game-simulation/src/movement.ts
- [ ] Extract calculatePosition function
- [ ] Extract validateMovement function
- [ ] Add unit tests
- [ ] Update server imports
- [ ] Update client imports (for prediction)

## Acceptance Criteria

- [ ] Movement logic in single file
- [ ] 100% test coverage
- [ ] Server and client both compile
- [ ] No behavior changes in game

## Status

Not Started
```
