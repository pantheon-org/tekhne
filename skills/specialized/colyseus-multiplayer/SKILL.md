---
name: colyseus-multiplayer
description: >-
  Build authoritative real-time multiplayer servers with Colyseus 0.17+. Use when implementing rooms, schema state sync, client message validation, matchmaking, authentication, reconnection handling, or server-side anti-cheat constraints. Keywords: colyseus, room lifecycle, schema, multiplayer, websocket, matchmaking, onJoin, onLeave, onDrop, allowReconnection.
allowed-tools: read, write, edit, bash
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
4. **Checkpoint:** Verify schema sync with a test client before proceeding — confirm field changes propagate and no plain properties are used for game-critical state.
5. Wire matchmaking and metadata filters.
6. Add reconnection policy and timeout behavior.
7. **Checkpoint:** Simulate a transient disconnect and confirm `allowReconnection` restores session — verify the player is not ejected prematurely.
8. Verify full room behavior with local multi-client runs.

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

## Room Implementation Example

The following shows lifecycle hooks, Schema state, and reconnection handling together:

```typescript
import { Room, Client } from "@colyseus/core";
import { Schema, type, MapSchema } from "@colyseus/schema";

class Player extends Schema {
  @type("string") sessionId: string = "";
  @type("number") x: number = 0;
  @type("number") score: number = 0;
}

class GameState extends Schema {
  @type({ map: Player }) players = new MapSchema<Player>();
}

export class GameRoom extends Room<GameState> {
  maxClients = 4;

  onCreate(options: any) {
    this.setState(new GameState());

    this.onMessage("move", (client, payload: { dx: number; dy: number }) => {
      const player = this.state.players.get(client.sessionId);
      if (!player) return;

      // Server-side validation: cap movement delta
      const dx = Math.max(-5, Math.min(5, payload.dx));
      const dy = Math.max(-5, Math.min(5, payload.dy));
      player.x += dx;
    });
  }

  onJoin(client: Client, options: any) {
    const player = new Player();
    player.sessionId = client.sessionId;
    this.state.players.set(client.sessionId, player);
  }

  async onLeave(client: Client, consented: boolean) {
    if (!consented) {
      // Hold the slot for up to 20 seconds on transient disconnect
      const reconnection = await this.allowReconnection(client, 20);
      if (!reconnection) {
        this.state.players.delete(client.sessionId);
      }
    } else {
      this.state.players.delete(client.sessionId);
    }
  }

  onDispose() {
    console.log("Room disposed");
  }
}
```

## Anti-Patterns

### NEVER trust client position or score updates directly

**WHY:** Clients are untrusted and can be modified for cheating.

**BAD:** Apply `payload.x` and `payload.score` without server checks.

**GOOD:** Validate movement delta and compute score on server.

```typescript
// BAD
this.onMessage("move", (client, payload) => {
  player.x = payload.x;
  player.score = payload.score;
});

// GOOD
this.onMessage("move", (client, payload: { dx: number }) => {
  const player = this.state.players.get(client.sessionId);
  if (!player) return;
  const dx = Math.max(-5, Math.min(5, payload.dx)); // clamp server-side
  player.x += dx;
  // score computed exclusively by server logic, never from client
});
```

**Consequence:** Competitive integrity is lost and leaderboard data is corrupted.

### NEVER mutate non-Schema fields expecting automatic sync

**WHY:** Only Schema-decorated fields are synchronized to clients.

**BAD:** Store gameplay-critical values in plain class properties.

**GOOD:** Keep synchronized values in Schema fields and collections.

```typescript
// BAD
class Player extends Schema {
  hp: number = 100; // plain property — not synced
}

// GOOD
class Player extends Schema {
  @type("number") hp: number = 100; // decorated — synced automatically
}
```

**Consequence:** Clients desync and render stale or inconsistent state.

### NEVER skip reconnection handling for transient disconnects

**WHY:** Mobile and unstable networks frequently drop short-lived connections.

**BAD:** Remove player immediately in `onLeave` for all disconnects.

**GOOD:** Use `allowReconnection` with a bounded timeout, falling back to removal only on expiry.

```typescript
// BAD
async onLeave(client: Client, consented: boolean) {
  this.state.players.delete(client.sessionId);
}

// GOOD
async onLeave(client: Client, consented: boolean) {
  if (!consented) {
    const reconnection = await this.allowReconnection(client, 20);
    if (!reconnection) {
      this.state.players.delete(client.sessionId);
    }
  } else {
    this.state.players.delete(client.sessionId);
  }
}
```

**Consequence:** Players are ejected from active matches unnecessarily.

### NEVER expose privileged room messages to all clients

**WHY:** Admin or host-only actions must be authorization-gated.

**BAD:** Let any client trigger `startMatch` or `kickPlayer`.

**GOOD:** Verify role/ownership before privileged actions.

```typescript
// BAD
this.onMessage("startMatch", (client) => {
  this.startMatch();
});

// GOOD
this.onMessage("startMatch", (client) => {
  if (client.sessionId !== this.hostSessionId) return; // guard
  this.startMatch();
});
```

**Consequence:** Match flow can be hijacked by unauthorized clients.

## References

- `references/room-lifecycle-and-state.md`
- `references/message-validation-and-security.md`
- `references/matchmaking-and-reconnection.md`
