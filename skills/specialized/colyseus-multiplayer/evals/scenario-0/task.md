# Battle Arena: Player State Design

## Problem Description

A small studio is building a 4-player battle arena game with Colyseus. Each player needs several tracked attributes: health points, x/y position, an active status flag, and a score. The team's previous prototype had a bug where spectators saw stale health values and score updates were invisible to opponents — a clear state sync issue they need to avoid this time.

Design the Schema classes for the room. The room should support up to 4 players joining by session. When a player joins, they should be added to the room state with default values. When a player leaves, they should be removed.

## Output Specification

Produce a single TypeScript file `state.ts` containing:
- The `Player` Schema class with all required fields
- The `BattleState` Schema class containing a collection of players
- A `BattleRoom` class with `onCreate`, `onJoin`, `onLeave`, and `onDispose` lifecycle hooks implemented

Do not worry about running the code — just write it correctly. Include necessary imports from `@colyseus/core` and `@colyseus/schema`.
