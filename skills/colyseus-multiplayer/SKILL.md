---
name: colyseus-multiplayer
description: Build authoritative real-time multiplayer servers with Colyseus 0.17+. Use when implementing rooms, schema state sync, client message validation, matchmaking, authentication, reconnection handling, or server-side anti-cheat constraints. Keywords: colyseus, room lifecycle, schema, multiplayer, websocket, matchmaking, onJoin, onLeave, onDrop, allowReconnection.
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
---

# Colyseus Multiplayer

## When to Use

Use this skill for server-authoritative multiplayer game backends using Colyseus.

## When Not to Use

Do not use this skill for peer-to-peer networking or single-player game architecture.

## Core Principles

1. Server is authoritative for all game state.
2. Room state changes must be validated before applying.
3. Reconnection and drop handling must be explicit.
4. Matchmaking metadata must reflect real room capability.

## Deterministic Workflow

1. Define state models in Schema.
2. Implement room lifecycle hooks (`onCreate`, `onJoin`, `onLeave`, `onDrop`).
3. Register message handlers with payload validation.
4. Wire matchmaking and metadata filters.
5. Add reconnection policy and timeout behavior.
6. Verify room behavior with local multi-client runs.

## Quick Commands

### Scaffold a Colyseus app

```bash
npm create colyseus-app@latest server
```

Expected result: runnable Colyseus project in `server/`.

### Install and run with Bun

```bash
cd server && bun install && bun run src/index.ts
```

Expected result: server listening on configured port.

### Run with Node scripts

```bash
cd server && npm install && npm run start
```

Expected result: room handlers and matchmaker initialized.

### Evaluate this skill quality

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh colyseus-multiplayer --json
```

Expected result: updated dimension score breakdown.

### Lint this skill docs

```bash
bunx markdownlint-cli2 "skills/colyseus-multiplayer/**/*.md"
```

Expected result: no markdownlint violations.

## Anti-Patterns

### NEVER trust client position or score updates directly

**WHY:** Clients are untrusted and can be modified for cheating.

**BAD:** Apply `payload.x` and `payload.score` without server checks.
**GOOD:** Validate movement delta and compute score on server.

**Consequence:** Competitive integrity is lost and leaderboard data is corrupted.

### NEVER mutate non-Schema fields expecting automatic sync

**WHY:** Only Schema-decorated fields are synchronized to clients.

**BAD:** Store gameplay-critical values in plain class properties.
**GOOD:** Keep synchronized values in Schema fields and collections.

**Consequence:** Clients desync and render stale or inconsistent state.

### NEVER skip reconnection handling for transient disconnects

**WHY:** Mobile and unstable networks frequently drop short-lived connections.

**BAD:** Remove player immediately in `onLeave` for all disconnects.
**GOOD:** Use `onDrop` + `allowReconnection` with bounded timeout.

**Consequence:** Players are ejected from active matches unnecessarily.

### NEVER expose privileged room messages to all clients

**WHY:** Admin or host-only actions must be authorization-gated.

**BAD:** Let any client trigger `startMatch` or `kickPlayer`.
**GOOD:** Verify role/ownership before privileged actions.

**Consequence:** Match flow can be hijacked by unauthorized clients.

## References

- `references/room-lifecycle-and-state.md`
- `references/message-validation-and-security.md`
- `references/matchmaking-and-reconnection.md`
