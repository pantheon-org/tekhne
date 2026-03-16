# Competitive Runner: Movement and Score Handlers

## Problem Description

A competitive runner game has had reports of cheaters teleporting across the map and submitting impossible scores. The lead developer suspects the current server blindly applies whatever coordinates and score the client sends in the move message. The team wants message handlers that are robust against manipulation.

Implement the `move` and `collectCoin` message handlers for the game room. Players send movement as a delta (change in x/y, not an absolute position). Collecting a coin should increment the player's score. The game runs at 30fps so move messages are very frequent.

## Output Specification

Produce a TypeScript file `handlers.ts` containing:
- Handler registration code using `this.onMessage(...)` for both `"move"` and `"collectCoin"` messages
- A `Player` Schema class (minimal: x, y, score fields)
- Any validation logic, rate-limiting logic, and type definitions needed

Include imports. Assume the room class body is already set up — just provide the handler implementations and supporting code.
