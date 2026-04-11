# Scenario 03: Mobile Puzzle Co-op: Disconnect and Rejoin Handling

## User Prompt

A casual co-op puzzle game targets mobile players. The QA team has logged a recurring complaint: when a player briefly loses signal (underground, elevator), they are immediately ejected from the match and their partner is left alone in a broken room. Players who tap "Leave" voluntarily should be removed immediately, but involuntary disconnects should get a grace window.

Implement the `onLeave` and `onDispose` methods for the `PuzzleRoom`. The room supports exactly 2 players working through a shared puzzle. When a player disconnects unexpectedly, their slot should stay reserved for a reasonable time so they can return without disrupting their partner.

## Output Specification

Produce a TypeScript file `puzzle-room.ts` with:
- A minimal `Player` Schema class (just a name or id field is enough)
- A `PuzzleState` Schema class
- The `PuzzleRoom` class implementing `onCreate`, `onJoin`, `onLeave`, and `onDispose`

Focus the implementation on correct disconnect handling. Include any necessary imports.

## Expected Behavior

1. Inspect the `consented` parameter in `onLeave` to branch between voluntary and involuntary disconnect
2. Call `this.allowReconnection(client, timeout)` on the non-consented path — do not remove the player immediately
3. Set the `allowReconnection` timeout between 20 and 30 seconds (appropriate for a casual match)
4. Delete the player from state only after `allowReconnection` resolves to falsy (timeout expired) — not before
5. Remove the player from state immediately when the disconnect is voluntary (`consented === true`) without calling `allowReconnection`
6. Declare `onLeave` as `async` (required for `await this.allowReconnection`)
7. Implement all four hooks: `onCreate`, `onJoin`, `onLeave`, and `onDispose`

## Success Criteria

- **Checks consented flag**: `onLeave` inspects the `consented` parameter to branch between voluntary and involuntary disconnect
- **allowReconnection called**: Non-consented path calls `this.allowReconnection(client, timeout)` — not `this.state.players.delete()` immediately
- **Timeout in 20-30s range**: The timeout passed to `allowReconnection` is between 20 and 30 seconds (appropriate for a casual match)
- **Conditional removal**: Player is deleted from state only after `allowReconnection` resolves to falsy (timeout expired) — not before
- **Consented remove immediately**: Voluntary disconnect (`consented === true`) removes the player from state without calling `allowReconnection`
- **onLeave is async**: `onLeave` is declared as an async function (required for `await this.allowReconnection`)
- **All hooks present**: Room implements `onCreate`, `onJoin`, `onLeave`, and `onDispose`

## Failure Conditions

- `onLeave` does not check the `consented` parameter to distinguish voluntary from involuntary disconnect
- Non-consented path removes the player immediately instead of calling `allowReconnection`
- `allowReconnection` timeout is outside the 20–30 second range
- Player is deleted from state before `allowReconnection` resolves
- Voluntary disconnect path calls `allowReconnection` instead of removing the player immediately
- `onLeave` is not declared as `async`
- Any of the four required lifecycle hooks (`onCreate`, `onJoin`, `onLeave`, `onDispose`) is missing
