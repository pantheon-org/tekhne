---
name: colyseus-multiplayer
description: |-
  Build authoritative multiplayer game servers with Colyseus 0.17+. Covers room lifecycle, state synchronization with Schema, message handling, matchmaking, authentication, and reconnection. Use for: "create colyseus server", "multiplayer room", "state sync", "colyseus schema", "websocket game server".
  
  Examples:
  - user: "Create multiplayer game server" → scaffold Colyseus server with rooms and state
  - user: "Add player state sync" → implement Schema-based state synchronization
  - user: "Handle player reconnection" → implement onDrop/allowReconnection pattern
  - user: "Validate player input" → add Zod validation to message handlers
---

# Colyseus Multiplayer Framework

Build authoritative multiplayer game servers with real-time state synchronization using Colyseus 0.17+.

## Core Concepts

### Rooms

**Rooms** are isolated game sessions. Each room instance:

- Contains its own **state** (synchronized with clients)
- Has its own **clients** (players)
- Runs its own **game loop** (simulation interval)
- Can be **dynamically created and destroyed**

**Example: Multiple room instances from one definition**

```typescript
// Server defines ONE room type: "game_room"
defineServer({
  rooms: {
    game_room: defineRoom(GameRoom),
  },
});

// Clients create MULTIPLE room instances:
// - Room A: 4 players in one match
// - Room B: 2 players in another match
// - Room C: 1 player waiting for others
```

### State Synchronization

Colyseus uses **Schema** to define synchronized state:

```typescript
import { Schema, MapSchema, type } from "@colyseus/schema";

export class Player extends Schema {
  @type("number") x: number = 0;
  @type("number") y: number = 0;
  @type("string") name: string = "";
}

export class GameState extends Schema {
  @type({ map: Player }) players = new MapSchema<Player>();
  @type("number") currentRound: number = 0;
}
```

**How it works:**

1. Server mutates `room.state` (e.g., `player.x = 100`)
2. Colyseus tracks changes automatically
3. At `patchRate` interval (default 50ms), **delta patches** are sent to clients
4. Clients receive **only the changed properties** (not full state)

### Clients

Each client connection has:

- **`sessionId`**: Unique identifier (use as player ID)
- **`auth`**: Authentication data (from `onAuth`)
- **`userData`**: Custom data storage
- **`reconnectionToken`**: Token for reconnecting

## Server Setup

### Installation

```bash
# Create new Colyseus project (Node.js)
npm create colyseus-app@latest ./server

# OR with Bun (faster)
bun create colyseus-app@latest ./server
cd server
bun add @colyseus/bun-websockets
bun run src/index.ts
```

### Server Configuration

```typescript
// src/app.config.ts
import { defineServer, defineRoom } from "colyseus";
import { GameRoom } from "./rooms/GameRoom";

const server = defineServer({
  rooms: {
    game_room: defineRoom(GameRoom, {
      // Default options (can be overridden by client)
      maxPlayers: 4,
      mode: "race",
    }),
  },
  options: {
    port: 2567,
    graceShutdownTime: 10000, // 10 seconds
  },
});

export default server;
```

## Room Lifecycle

### Room Class Structure

```typescript
import { Room, Client } from "colyseus";
import { GameState, Player } from "./GameState";

export class GameRoom extends Room<GameState> {
  maxClients = 4;
  state = new GameState();

  // Message handlers
  messages = {
    move: (client, payload) => {
      const player = this.state.players.get(client.sessionId);
      player.x = payload.x;
      player.y = payload.y;
    },
  };

  // Lifecycle hooks
  onCreate(options: any) {}
  onAuth(client: Client, options: any, context: any) {}
  onJoin(client: Client, options: any, auth: any) {}
  onDrop(client: Client, code?: number) {}
  onReconnect(client: Client) {}
  onLeave(client: Client, code?: number) {}
  onDispose() {}
}
```

### Lifecycle Methods

| Method                             | When Called                      | Purpose                                  |
| ---------------------------------- | -------------------------------- | ---------------------------------------- |
| `onCreate(options)`                | Room created by matchmaker       | Initialize room state, start game logic  |
| `onAuth(client, options, context)` | Before client joins              | Validate authentication token            |
| `onJoin(client, options, auth)`    | Client successfully joins        | Add player to state                      |
| `onDrop(client, code)`             | Client disconnects unexpectedly  | Allow reconnection, mark player inactive |
| `onReconnect(client)`              | Client reconnects successfully   | Restore player state                     |
| `onLeave(client, code)`            | Client leaves permanently        | Remove player from state                 |
| `onDispose()`                      | Room destroyed (no clients left) | Cleanup, persist data to database        |

### onCreate Example

```typescript
onCreate(options: { maxPlayers: number, mode: string }) {
  console.log("Room created with options:", options);

  // Set room metadata (for matchmaking)
  this.setMetadata({ mode: options.mode, difficulty: "normal" });

  // Start game loop
  this.setSimulationInterval((deltaTime) => this.update(deltaTime));

  // Set max clients
  this.maxClients = options.maxPlayers;
}

update(deltaTime: number) {
  // Game logic runs here at ~60 FPS (default 16.6ms interval)
  // For turn-based games, you may not need this
}
```

### onJoin Example

```typescript
onJoin(client: Client, options: any, auth: any) {
  console.log(client.sessionId, "joined with options:", options);

  // Add player to state
  const player = new Player();
  player.name = auth.username; // From onAuth
  player.x = 0;
  player.y = 0;
  this.state.players.set(client.sessionId, player);

  // Notify other players
  this.broadcast("player-joined", {
    sessionId: client.sessionId,
    name: player.name
  }, { except: client });
}
```

### onLeave Example

```typescript
onLeave(client: Client, code: number) {
  console.log(client.sessionId, "left with code:", code);

  // Remove player from state
  this.state.players.delete(client.sessionId);

  // Notify other players
  this.broadcast("player-left", { sessionId: client.sessionId });
}
```

## State Schema

### Schema Definition

```typescript
import { Schema, MapSchema, ArraySchema, type } from "@colyseus/schema";

export class Player extends Schema {
  @type("string") id: string;
  @type("string") name: string;
  @type("number") x: number = 0;
  @type("number") y: number = 0;
  @type("number") score: number = 0;
  @type("boolean") ready: boolean = false;
}

export class GameState extends Schema {
  @type({ map: Player }) players = new MapSchema<Player>();
  @type(["string"]) chatMessages = new ArraySchema<string>();
  @type("number") roundStartTime: number = 0;
  @type("string") gameStatus: string = "waiting"; // "waiting", "playing", "finished"
}
```

### Supported Schema Types

| Type                    | Description   | Example                                             |
| ----------------------- | ------------- | --------------------------------------------------- |
| `"string"`              | UTF-8 string  | `@type("string") name: string`                      |
| `"number"`              | Float64       | `@type("number") score: number`                     |
| `"boolean"`             | Boolean       | `@type("boolean") ready: boolean`                   |
| `"int8"` ... `"uint64"` | Integer types | `@type("uint8") health: number`                     |
| `[type]`                | Array         | `@type(["string"]) items: ArraySchema<string>`      |
| `{ map: Type }`         | Map (dict)    | `@type({ map: Player }) players: MapSchema<Player>` |
| `{ set: Type }`         | Set           | `@type({ set: "string" }) tags: SetSchema<string>`  |
| `Type`                  | Nested Schema | `@type(Player) player: Player`                      |

### Mutating State (Server)

```typescript
// Add player
const player = new Player();
player.name = "Alice";
this.state.players.set(client.sessionId, player);

// Update player
const player = this.state.players.get(client.sessionId);
player.x += 5;
player.score = 100;

// Remove player
this.state.players.delete(client.sessionId);

// Array operations
this.state.chatMessages.push("Hello world!");

// State is automatically synchronized!
```

### Listening to State Changes (Client)

```typescript
import { Client, Callbacks } from "@colyseus/sdk";

const client = new Client("http://localhost:2567");
const room = await client.joinOrCreate("game_room");
const callbacks = Callbacks.get(room);

// Listen to player additions
callbacks.onAdd("players", (player, sessionId) => {
  console.log("Player joined:", player.name);

  // Listen to player property changes
  callbacks.listen(player, "x", (x, prevX) => {
    console.log(`Player moved from ${prevX} to ${x}`);
  });

  callbacks.listen(player, "score", (score) => {
    console.log(`Player score: ${score}`);
  });
});

// Listen to player removals
callbacks.onRemove("players", (player, sessionId) => {
  console.log("Player left:", player.name);
});

// Listen to array changes
callbacks.onChange("chatMessages", (messages) => {
  console.log("New message:", messages[messages.length - 1]);
});
```

## Message Handling

### Server: Handling Client Messages

```typescript
export class GameRoom extends Room<GameState> {
  messages = {
    // Simple message handler
    move: (client, payload: { x: number; y: number }) => {
      const player = this.state.players.get(client.sessionId);
      player.x = payload.x;
      player.y = payload.y;
    },

    // Message with validation (using Zod)
    chat: validate(
      z.object({
        message: z.string().min(1).max(200),
      }),
      (client, payload) => {
        const player = this.state.players.get(client.sessionId);
        const msg = `${player.name}: ${payload.message}`;
        this.state.chatMessages.push(msg);
      },
    ),

    // Fallback for unhandled messages
    "*": (client, type, payload) => {
      console.log("Unhandled message:", type, payload);
    },
  };
}
```

### Client: Sending Messages

```typescript
// Send message to server
room.send("move", { x: 100, y: 200 });
room.send("chat", { message: "Hello!" });

// Receive messages from server
room.onMessage("game-started", (payload) => {
  console.log("Game started!", payload);
});

room.onMessage("*", (type, payload) => {
  console.log("Received message:", type, payload);
});
```

### Server: Sending Messages to Clients

```typescript
// Broadcast to all clients
this.broadcast("game-started", { startTime: Date.now() });

// Broadcast except sender
this.broadcast("player-action", { action: "jump" }, { except: client });

// Send to specific client
client.send("welcome", { message: "Welcome to the game!" });

// Broadcast after next state patch
this.broadcast("round-ended", {}, { afterNextPatch: true });
```

## Matchmaking

### Client: Join or Create Room

```typescript
import { Client } from "@colyseus/sdk";

const client = new Client("http://localhost:2567");

// Join or create room (recommended)
const room = await client.joinOrCreate("game_room", {
  maxPlayers: 4,
  mode: "race",
});

// Create new room only
const room = await client.create("game_room", { mode: "race" });

// Join existing room by ID
const room = await client.joinById("room-id-here");

// Join existing room by criteria
const room = await client.join("game_room", { mode: "race" });
```

### Server: Room Metadata

```typescript
onCreate(options) {
  // Set metadata for matchmaking queries
  this.setMetadata({
    mode: options.mode,
    difficulty: "normal",
    map: "maze_1"
  });
}

// Update metadata during game
this.setMetadata({ difficulty: "hard" });

// Set private (hide from matchmaking)
this.setPrivate(true);

// Lock room (no new joins)
this.lock();

// Unlock room
this.unlock();
```

### Client: Query Available Rooms

```typescript
const rooms = await client.getAvailableRooms("game_room");

rooms.forEach((room) => {
  console.log(room.roomId);
  console.log(room.clients); // Number of clients
  console.log(room.maxClients);
  console.log(room.metadata); // Custom metadata
});

// Filter rooms by metadata
const hardRooms = rooms.filter((r) => r.metadata.difficulty === "hard");
```

## Authentication

### Server: onAuth Hook

```typescript
import { Client, AuthContext } from "colyseus";
import jwt from "jsonwebtoken";

export class GameRoom extends Room {
  async onAuth(client: Client, options: any, context: AuthContext) {
    // Validate JWT token
    const token = context.token; // From Authorization header

    try {
      const decoded = jwt.verify(token, process.env.JWT_SECRET);

      // Return auth data (available in onJoin)
      return {
        userId: decoded.userId,
        username: decoded.username,
      };
    } catch (error) {
      throw new Error("Invalid token");
    }
  }

  onJoin(client: Client, options: any, auth: any) {
    console.log("Authenticated user:", auth.username);

    // Access auth data
    const player = new Player();
    player.name = auth.username;
    this.state.players.set(client.sessionId, player);
  }
}
```

### Client: Sending Auth Token

```typescript
const client = new Client("http://localhost:2567", {
  headers: {
    Authorization: `Bearer ${jwtToken}`,
  },
});

const room = await client.joinOrCreate("game_room");
```

## Reconnection Handling

### Server: Allow Reconnection

```typescript
export class GameRoom extends Room {
  async onDrop(client: Client, code?: number) {
    console.log(client.sessionId, "dropped unexpectedly");

    // Mark player as inactive
    const player = this.state.players.get(client.sessionId);
    if (player) {
      player.ready = false;
    }

    // Allow 20 seconds to reconnect
    try {
      await this.allowReconnection(client, 20);
      console.log(client.sessionId, "reconnected!");
    } catch (e) {
      console.log(client.sessionId, "failed to reconnect");
      // onLeave will be called automatically
    }
  }

  onReconnect(client: Client) {
    console.log(client.sessionId, "successfully reconnected");

    // Re-activate player
    const player = this.state.players.get(client.sessionId);
    if (player) {
      player.ready = true;
    }
  }

  onLeave(client: Client, code?: number) {
    console.log(client.sessionId, "left permanently");

    // Remove player from state
    this.state.players.delete(client.sessionId);
  }
}
```

### Client: Automatic Reconnection

```typescript
const client = new Client("http://localhost:2567", {
  enableReconnect: true, // Enable automatic reconnection
  reconnectAttempts: 5, // Max 5 attempts
  reconnectDelay: 2000, // 2 seconds between attempts
});

const room = await client.joinOrCreate("game_room");

// Automatic reconnection will happen in the background
```

## Best Practices

### Performance

1. **Minimize State Size**: Only synchronize what clients need to see

   ```typescript
   // BAD: Synchronizing server-only data
   @type("string") private secretData: string;

   // GOOD: Don't add @type decorator for server-only data
   private secretData: string; // Not synchronized
   ```

2. **Use Appropriate Data Types**: Use smaller integer types when possible

   ```typescript
   @type("uint8") health: number = 100; // 0-255 (1 byte)
   @type("int16") x: number = 0;        // -32768 to 32767 (2 bytes)
   @type("number") score: number = 0;   // Float64 (8 bytes)
   ```

3. **Batch State Changes**: Multiple mutations are sent as one patch

   ```typescript
   // All changes sent in one patch
   player.x = 100;
   player.y = 200;
   player.score += 10;
   ```

4. **Adjust Patch Rate**: Default 50ms (20fps), increase for slower games

   ```typescript
   export class GameRoom extends Room {
     patchRate = 100; // 10 patches per second (turn-based games)
   }
   ```

### Security

1. **Validate All Client Input**: Never trust client data

   ```typescript
   messages = {
     move: validate(
       z.object({
         x: z.number().min(0).max(MAP_WIDTH),
         y: z.number().min(0).max(MAP_HEIGHT),
       }),
       (client, payload) => {
         // payload is validated and typed
       },
     ),
   };
   ```

2. **Rate Limit Messages**: Prevent spam

   ```typescript
   export class GameRoom extends Room {
     maxMessagesPerSecond = 30; // Disconnect clients sending >30 msg/s
   }
   ```

3. **Use Authoritative Server**: Server is source of truth

   ```typescript
   // BAD: Trust client position
   messages = {
     move: (client, payload) => {
       player.x = payload.x; // Client can cheat!
     },
   };

   // GOOD: Server validates movement
   messages = {
     move: (client, payload) => {
       const player = this.state.players.get(client.sessionId);
       const newX = player.x + payload.dx;

       // Server checks if move is valid
       if (this.isValidPosition(newX, player.y)) {
         player.x = newX;
       }
     },
   };
   ```

### Code Organization

1. **Separate State from Logic**: Use command pattern

   ```typescript
   // state/Player.ts
   export class Player extends Schema {
     @type("number") x: number = 0;
     @type("number") y: number = 0;
   }

   // commands/MoveCommand.ts
   export const movePlayer = (player: Player, dx: number, dy: number) => {
     player.x += dx;
     player.y += dy;
   };

   // rooms/GameRoom.ts
   messages = {
     move: (client, payload) => {
       const player = this.state.players.get(client.sessionId);
       movePlayer(player, payload.dx, payload.dy);
     },
   };
   ```

2. **Use TypeScript**: Type safety prevents bugs

   ```typescript
   // Define message types
   type Messages = {
     move: { x: number; y: number };
     chat: { message: string };
   };

   export class GameRoom extends Room<{ messages: Messages }> {
     // Messages are now type-checked!
   }
   ```

3. **Share Types Between Client/Server**: Use monorepo

   ```
   libs/
     shared/
       src/
         state/
           Player.ts
           GameState.ts
   apps/
     client/
     server/
   ```

## Resources

- **Colyseus Docs**: [https://docs.colyseus.io](https://docs.colyseus.io)
- **GitHub**: [https://github.com/colyseus/colyseus](https://github.com/colyseus/colyseus)
- **Discord**: [https://discord.gg/RY8rRS7](https://discord.gg/RY8rRS7)
- **Phaser + Colyseus Tutorial**: [https://docs.colyseus.io/tutorial/phaser](https://docs.colyseus.io/tutorial/phaser)
- **Colyseus Examples**: [https://github.com/colyseus/colyseus-examples](https://github.com/colyseus/colyseus-examples)
