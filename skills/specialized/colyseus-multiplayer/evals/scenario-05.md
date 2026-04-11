# Scenario 05: Ranked Card Duel: Full Room Setup

## User Prompt

A competitive card game team is building a ranked 1v1 duel mode. Matches are region-specific to minimize latency, and the ranking system needs to know the game mode for each room so the matchmaker can pair players correctly. The game is turn-based: only the active player should be allowed to play a card, and the turn switches after each play. The team wants a well-structured room that can be discovered via matchmaking.

Implement the full `CardDuelRoom` in TypeScript. Each player has a hand size (number of cards) and a score. The room tracks whose turn it is. Include a handler for the `"playCard"` message that enforces turn order.

## Output Specification

Produce a TypeScript file `card-duel-room.ts` with:
- `DuelPlayer` and `DuelState` Schema classes
- `CardDuelRoom` implementing all lifecycle hooks
- Room metadata configuration for matchmaking
- The `"playCard"` message handler

The room creator passes `{ mode: "ranked", region: string }` as options to `onCreate`. Include imports.

## Expected Behavior

1. Call `this.setMetadata()` (or equivalent) in `onCreate` with `mode` and `region` values from the `options` parameter
2. Derive metadata values from the `onCreate` options object — not hardcoded constants
3. Decorate `DuelPlayer` fields (score, hand size, etc.) with `@type` — no plain class properties for gameplay-critical values
4. Use `MapSchema<DuelPlayer>` for the players collection in `DuelState`, keyed by `sessionId`
5. Track whose turn it is — either via a `currentTurn` field in state or an internal server variable
6. Have the `playCard` handler check whether it is the sending client's turn and return early / ignore the message if not
7. If reconnection is implemented, set the timeout in the 10–15 second range (appropriate for ranked mode)
8. Implement all four lifecycle hooks: `onCreate`, `onJoin`, `onLeave`, and `onDispose`

## Success Criteria

- **Metadata set in onCreate**: `onCreate` calls `this.setMetadata()` (or equivalent) with `mode` and `region` values from the `options` parameter
- **Metadata from options**: Metadata values are derived from the `onCreate` options object — not hardcoded constants
- **Schema fields decorated**: `DuelPlayer` fields (score, hand size, etc.) use `@type` decorator — no plain class properties for gameplay-critical values
- **MapSchema for players**: `DuelState` uses `MapSchema<DuelPlayer>` for the players collection, keyed by `sessionId`
- **Turn order tracked**: Room tracks whose turn it is — either via a `currentTurn` field in state or an internal server variable
- **Turn order enforced**: `playCard` handler checks whether it is the sending client's turn and returns early / ignores the message if not
- **Ranked reconnection timeout**: If reconnection is implemented, timeout is in the 10–15 second range (appropriate for ranked mode)
- **All hooks present**: Room implements `onCreate`, `onJoin`, `onLeave`, and `onDispose`

## Failure Conditions

- `onCreate` does not call `setMetadata()` with `mode` and `region`
- Metadata values are hardcoded instead of derived from the `options` parameter
- Any `DuelPlayer` gameplay field is defined as a plain class property without `@type`
- `DuelState` uses a plain array or object instead of `MapSchema<DuelPlayer>`
- Room does not track whose turn it is
- `playCard` handler does not check turn order before executing
- Reconnection timeout (if present) is outside the 10–15 second range for ranked mode
- Any of the four required lifecycle hooks (`onCreate`, `onJoin`, `onLeave`, `onDispose`) is missing
