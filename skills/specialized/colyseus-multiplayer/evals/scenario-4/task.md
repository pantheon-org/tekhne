# Ranked Card Duel: Full Room Setup

## Problem Description

A competitive card game team is building a ranked 1v1 duel mode. Matches are region-specific to minimize latency, and the ranking system needs to know the game mode for each room so the matchmaker can pair players correctly. The game is turn-based: only the active player should be allowed to play a card, and the turn switches after each play. The team wants a well-structured room that can be discovered via matchmaking.

Implement the full `CardDuelRoom` in TypeScript. Each player has a hand size (number of cards) and a score. The room tracks whose turn it is. Include a handler for the `"playCard"` message that enforces turn order.

## Output Specification

Produce a TypeScript file `card-duel-room.ts` with:
- `DuelPlayer` and `DuelState` Schema classes
- `CardDuelRoom` implementing all lifecycle hooks
- Room metadata configuration for matchmaking
- The `"playCard"` message handler

The room creator passes `{ mode: "ranked", region: string }` as options to `onCreate`. Include imports.
