# Scenario 01: Battle Arena: Player State Design

## User Prompt

A small studio is building a 4-player battle arena game with Colyseus. Each player needs several tracked attributes: health points, x/y position, an active status flag, and a score. The team's previous prototype had a bug where spectators saw stale health values and score updates were invisible to opponents — a clear state sync issue they need to avoid this time.

Design the Schema classes for the room. The room should support up to 4 players joining by session. When a player joins, they should be added to the room state with default values. When a player leaves, they should be removed.

## Output Specification

Produce a single TypeScript file `state.ts` containing:
- The `Player` Schema class with all required fields
- The `BattleState` Schema class containing a collection of players
- A `BattleRoom` class with `onCreate`, `onJoin`, `onLeave`, and `onDispose` lifecycle hooks implemented

Do not worry about running the code — just write it correctly. Include necessary imports from `@colyseus/core` and `@colyseus/schema`.

## Expected Behavior

1. Decorate the Player health/hp field with `@type` (not a plain class property)
2. Decorate the Player x and y position fields with `@type`
3. Decorate the Player score field with `@type`
4. Decorate the Player active/isActive status field with `@type`
5. Use `MapSchema<Player>` (not an array or plain object) for the players collection in `BattleState`
6. Store players in the map using `client.sessionId` as the key
7. Call `this.setState()` with a new `BattleState` instance in `onCreate`
8. Create a `Player` and add it to `state.players` with `sessionId` as key in `onJoin`
9. Remove the player from `state.players` using `client.sessionId` in `onLeave`
10. Implement all four hooks: `onCreate`, `onJoin`, `onLeave`, and `onDispose`

## Success Criteria

- **HP field decorated**: Player health/hp field uses `@type` decorator (not a plain class property)
- **Position fields decorated**: Player x and y fields use `@type` decorator
- **Score field decorated**: Player score field uses `@type` decorator
- **Status field decorated**: Player active/isActive field uses `@type` decorator
- **MapSchema for players**: `BattleState` uses `MapSchema<Player>` (not an array or plain object) for the players collection
- **sessionId as map key**: Players are stored in the map using `client.sessionId` as the key
- **onCreate sets state**: `onCreate` calls `this.setState()` with a new `BattleState` instance
- **onJoin adds player**: `onJoin` creates a `Player` and adds it to `state.players` with `sessionId` as key
- **onLeave removes player**: `onLeave` removes the player from `state.players` using `client.sessionId`
- **All four hooks present**: File contains `onCreate`, `onJoin`, `onLeave`, and `onDispose` methods

## Failure Conditions

- Any Player gameplay field (hp, x, y, score, active) is defined as a plain class property without `@type`
- `BattleState` uses a plain array or object instead of `MapSchema<Player>` for the players collection
- Players are not keyed by `client.sessionId` in the map
- `onCreate` does not call `this.setState()` with a `BattleState` instance
- `onJoin` does not add the new player to `state.players`
- `onLeave` does not remove the player from `state.players`
- Any of the four required lifecycle hooks (`onCreate`, `onJoin`, `onLeave`, `onDispose`) is missing
