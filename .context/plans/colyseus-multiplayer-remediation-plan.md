---
plan_date: 2026-02-23
skill_name: colyseus-multiplayer
source_audit: .context/audits/colyseus-multiplayer-audit-2026-02-22.md
---

# Remediation Plan: colyseus-multiplayer

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 81/120 | 96/120 |
| **Grade** | D | B |
| **Priority** | Critical | - |
| **Effort** | Large (L) | - |

**Focus Areas**: Progressive disclosure (D5), Knowledge delta (D1), Mindset + Procedures (D2)

## Critical Issues to Address

| Issue | Severity | Dimension | Impact |
| --- | --- | --- | --- |
| Very large SKILL.md (701 lines, 0 refs) | Critical | D5 (5/15) | Unmaintainable structure |
| Generic tutorial content | High | D1 (14/20) | Lacks repository specificity |
| Unclear workflow sequence | High | D2 (10/15) | Execution ambiguity |
| Weak pattern recognition | Medium | D7 (6/10) | Skill activation issues |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - Priority: Critical

**Target**: Increase from 5/15 to 12/15 (+7 points)

#### Step 1.1: Create comprehensive reference structure

**File**: `skills/colyseus-multiplayer/`

```
skills/colyseus-multiplayer/
├── SKILL.md (hub, ~100 lines)
├── references/
│   ├── room-lifecycle.md
│   ├── state-synchronization.md
│   ├── message-handling.md
│   ├── matchmaking.md
│   ├── authentication.md
│   └── deployment.md
├── templates/
│   └── room-template.ts.yaml
```

#### Step 1.2: Extract Room Lifecycle content

**File**: `skills/colyseus-multiplayer/references/room-lifecycle.md`

Move detailed room creation, lifecycle hooks, and cleanup logic:

````markdown
# Room Lifecycle

## Creation Hooks

### onCreate(options)

Called when room is created. Initialize state here.

```ts
async onCreate(options: any) {
  this.setState(new GameState());
  this.roomId = generateRoomId();
  this.maxClients = options.maxClients || 4;
}
```

### onJoin(client, options)

Called when client joins. Add player to state.

### onLeave(client, consented)

Called when client leaves. Handle reconnection.

### onDispose()

Called when room is destroyed. Cleanup resources.

[Continue with detailed content from SKILL.md...]
````

#### Step 1.3: Extract State Synchronization content

**File**: `skills/colyseus-multiplayer/references/state-synchronization.md`

Move Schema definitions, state patterns, and synchronization rules.

#### Step 1.4: Extract Message Handling content

**File**: `skills/colyseus-multiplayer/references/message-handling.md`

Move message types, validation, and handler patterns.

#### Step 1.5: Extract remaining deep content

Continue with:
- `matchmaking.md` - Matchmaking patterns and queue logic
- `authentication.md` - Auth integration patterns
- `deployment.md` - Production deployment considerations

#### Step 1.6: Rewrite SKILL.md as navigation hub

**File**: `skills/colyseus-multiplayer/SKILL.md`

````markdown
---
name: colyseus-multiplayer
description: "[from Phase 3]"
---

# Colyseus Multiplayer Game Servers

Authoritative multiplayer server development with Colyseus 0.17+.

## Use When

- "Create multiplayer game server"
- "Set up Colyseus room"
- "State synchronization with Schema"
- "Handle player reconnection"
- "Colyseus matchmaking"

## Quick Start

```bash
npm install colyseus
npx colyseus-scripts init
```

For full setup, see [Room Lifecycle](references/room-lifecycle.md).

## Core Concepts

### Rooms
Server-side game rooms with lifecycle management.
→ [Room Lifecycle](references/room-lifecycle.md)

### State
Schema-based state synchronization.
→ [State Synchronization](references/state-synchronization.md)

### Messages
Client-server message handling.
→ [Message Handling](references/message-handling.md)

## Anti-Patterns

### NEVER: Trust client state

WHY: Clients can be modified. Server is authoritative.

BAD: Client sends position updates directly to other clients.
GOOD: Server validates and broadcasts all state changes.

[Additional anti-patterns...]

## Quick Commands

```bash
# Development
npm run dev

# Test room
npx colyseus-scripts test:room MyRoom

# Production
npm run build && npm start
```

## Related

- [Matchmaking](references/matchmaking.md)
- [Authentication](references/authentication.md)
- [Deployment](references/deployment.md)
````

---

### Phase 2: Knowledge Delta (D1) - Priority: High

**Target**: Increase from 14/20 to 17/20 (+3 points)

#### Step 2.1: Add repository-specific guidance

**File**: `skills/colyseus-multiplayer/SKILL.md`

Add project context:

````markdown
## Repository Conventions

This repository uses:

- TypeScript strict mode for all rooms
- Zod for message validation
- `./src/rooms/` for room definitions
- `./src/state/` for Schema classes
- Integration tests with `bun test`

### Project-specific patterns

```ts
// Room file structure
src/rooms/
  ├── LobbyRoom.ts
  ├── GameRoom.ts
  └── __tests__/
      └── GameRoom.test.ts
```
````

#### Step 2.2: Add production caveats

Document real-world gotchas specific to multiplayer development:

```markdown
## Production Caveats

- Room state size affects bandwidth; keep under 64KB per room
- Reconnection timeout defaults to 20 seconds; adjust for mobile
- Use `allowReconnection` for graceful disconnect handling
- Rate limit message handlers to prevent DoS
```

---

### Phase 3: Mindset + Procedures (D2) - Priority: High

**Target**: Increase from 10/15 to 13/15 (+3 points)

#### Step 3.1: Create deterministic workflow

**File**: `skills/colyseus-multiplayer/SKILL.md`

Add explicit workflow:

````markdown
## Development Workflow

### Step 1: Define State

Create Schema class in `src/state/`:

```bash
# Check existing state patterns
ls src/state/
# Create new state file
touch src/state/GameState.ts
```

Exit criteria: Schema compiles, type definitions complete.

### Step 2: Create Room

```bash
touch src/rooms/MyRoom.ts
```

Exit criteria: Room extends `Room<State>`, lifecycle hooks defined.

### Step 3: Add Message Handlers

```bash
# Define message types
touch src/messages/MyRoomMessages.ts
```

Exit criteria: Zod schemas defined, handlers registered.

### Step 4: Write Tests

```bash
bun test src/rooms/__tests__/MyRoom.test.ts
```

Exit criteria: All tests pass, coverage > 80%.

### Step 5: Integration Test

```bash
npm run dev
# Connect with test client
```

Exit criteria: Room accessible, state syncs correctly.
````

---

### Phase 4: Pattern Recognition (D7) - Priority: Medium

**Target**: Increase from 6/10 to 9/10 (+3 points)

#### Step 4.1: Expand frontmatter description

```yaml
---
name: colyseus-multiplayer
description: |
  Build authoritative multiplayer game servers with Colyseus 0.17+.
  Use when: creating multiplayer rooms, implementing state sync with Schema,
  handling player connections, matchmaking, authentication integration,
  or deploying Colyseus servers to production.
  
  Keywords: Colyseus, multiplayer, game server, room, Schema, state sync,
  WebSocket, matchmaking, reconnection, authoritative server, real-time
---
```

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh colyseus-multiplayer --json
bunx markdownlint-cli2 "skills/colyseus-multiplayer/**/*.md"
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D5 Progressive Disclosure | Score >= 12/15 |
| D1 Knowledge Delta | Score >= 17/20 |
| D2 Mindset + Procedures | Score >= 13/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| SKILL.md line count | <= 150 lines |
| References created | >= 5 files |
| Overall Score | >= 96/120 (B) |

## Estimated Effort

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Disclosure | L | 3 hours |
| Phase 2: Knowledge Delta | M | 1 hour |
| Phase 3: Procedures | M | 1 hour |
| Phase 4: Triggers | S | 20 min |
| **Total** | **L** | **5+ hours** |

## Dependencies

- None (self-contained skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/colyseus-multiplayer/
```

## Notes

This skill requires the most significant rework. Consider breaking into phases:

1. **Phase 1 (Critical)**: Extract references, reduce SKILL.md to hub
2. **Phase 2-4 (Follow-up)**: Enhance content quality
