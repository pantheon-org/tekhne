# Mobile Puzzle Co-op: Disconnect and Rejoin Handling

## Problem Description

A casual co-op puzzle game targets mobile players. The QA team has logged a recurring complaint: when a player briefly loses signal (underground, elevator), they are immediately ejected from the match and their partner is left alone in a broken room. Players who tap "Leave" voluntarily should be removed immediately, but involuntary disconnects should get a grace window.

Implement the `onLeave` and `onDispose` methods for the `PuzzleRoom`. The room supports exactly 2 players working through a shared puzzle. When a player disconnects unexpectedly, their slot should stay reserved for a reasonable time so they can return without disrupting their partner.

## Output Specification

Produce a TypeScript file `puzzle-room.ts` with:
- A minimal `Player` Schema class (just a name or id field is enough)
- A `PuzzleState` Schema class
- The `PuzzleRoom` class implementing `onCreate`, `onJoin`, `onLeave`, and `onDispose`

Focus the implementation on correct disconnect handling. Include any necessary imports.
