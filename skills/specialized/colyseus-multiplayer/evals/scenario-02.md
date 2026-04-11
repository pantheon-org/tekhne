# Scenario 02: Competitive Runner: Movement and Score Handlers

## User Prompt

A competitive runner game has had reports of cheaters teleporting across the map and submitting impossible scores. The lead developer suspects the current server blindly applies whatever coordinates and score the client sends in the move message. The team wants message handlers that are robust against manipulation.

Implement the `move` and `collectCoin` message handlers for the game room. Players send movement as a delta (change in x/y, not an absolute position). Collecting a coin should increment the player's score. The game runs at 30fps so move messages are very frequent.

## Output Specification

Produce a TypeScript file `handlers.ts` containing:
- Handler registration code using `this.onMessage(...)` for both `"move"` and `"collectCoin"` messages
- A `Player` Schema class (minimal: x, y, score fields)
- Any validation logic, rate-limiting logic, and type definitions needed

Include imports. Assume the room class body is already set up — just provide the handler implementations and supporting code.

## Expected Behavior

1. Validate the incoming move payload with Zod or equivalent schema validation before using any fields
2. Return early or skip mutation if payload validation fails
3. Accept dx/dy deltas in the move handler and add them to the current position — do not assign client-provided absolute coordinates directly
4. Clamp the movement delta on the server with a max bound (e.g. `Math.max`/`Math.min`)
5. Increment score using server-side logic in the `collectCoin` handler — do not read a score value from the client payload
6. Look up the player by `client.sessionId` in both handlers and return early if not found
7. Include rate-limiting logic in the move handler (e.g. timestamp check or cooldown) to reject messages sent too frequently
8. Decorate Player x, y, and score fields with `@type`

## Success Criteria

- **Payload schema validation**: Move handler validates the incoming payload with Zod (or equivalent) before using any fields
- **Early return on invalid**: Handler returns early (or skips mutation) if payload validation fails
- **Delta movement not absolute**: Move handler accepts dx/dy deltas and adds them to current position — does not assign client-provided absolute x/y directly
- **Server-side clamping**: Movement delta is clamped with a max bound on the server (e.g. `Math.max`/`Math.min` or equivalent)
- **Server-computed score**: `collectCoin` handler increments score using server-side logic — does not read a score value from the client payload
- **Player lookup guard**: Both handlers look up the player by `client.sessionId` and return early if not found
- **Rate limiting**: Move handler includes rate-limiting logic (e.g. timestamp check, cooldown) to reject messages sent too frequently
- **Schema fields decorated**: Player x, y, and score fields use `@type` decorator

## Failure Conditions

- Move handler uses the client-provided payload without schema validation
- Handler does not return early when payload validation fails
- Move handler assigns absolute x/y coordinates directly from the client payload instead of applying deltas
- No server-side clamping is applied to movement deltas
- `collectCoin` handler reads and applies a score value from the client message instead of computing server-side
- Either handler does not guard against missing player by session ID
- Move handler has no rate-limiting mechanism
- Any Player field (x, y, score) is defined without `@type` decorator
