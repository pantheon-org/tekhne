# Scenario 04: Party Game Lobby: Host Controls

## User Prompt

A party game needs a lobby room where the first player to join becomes the host. The host should have exclusive abilities: starting the match when enough players have joined, and removing a disruptive player from the lobby. Other players should not be able to trigger either of these actions, even if they try to send the same messages.

Implement the lobby room with host tracking and the two privileged message handlers: one to start the match and one to remove a specific player by their session ID.

## Output Specification

Produce a TypeScript file `lobby-room.ts` containing:
- A minimal `LobbyPlayer` Schema class (id and ready status are enough)
- A `LobbyState` Schema class
- A `LobbyRoom` class with `onCreate`, `onJoin`, `onLeave`, `onDispose`, and handlers for `"startMatch"` and `"kickPlayer"` messages

Include imports. The `kickPlayer` message payload contains a `targetSessionId` string.

## Expected Behavior

1. Determine and store the host session on the server (e.g. first joiner's `sessionId` in `onJoin` or `onCreate`) — not from a client-provided payload field
2. Assign the host when the first player joins (`onCreate` options or first `onJoin` call)
3. Check `client.sessionId` against the stored host ID in `startMatch` and return early if not the host
4. Check `client.sessionId` against the stored host ID in `kickPlayer` and return early if not the host
5. Use `payload.targetSessionId` to identify the player to remove in `kickPlayer` — not a client-supplied authority flag
6. Have both privileged handlers return (do nothing) when the caller is not the host — they must not throw or allow partial execution
7. Implement all four lifecycle hooks: `onCreate`, `onJoin`, `onLeave`, and `onDispose`

## Success Criteria

- **Host assigned server-side**: The host session is determined and stored by the server (e.g. first joiner's `sessionId` in `onJoin` or `onCreate`) — not provided by the client as a payload field
- **Host set on first join**: The host is assigned when the first player joins (`onCreate` options or first `onJoin` call)
- **startMatch guard**: `startMatch` handler checks `client.sessionId` against the stored host ID and returns early if not the host
- **kickPlayer guard**: `kickPlayer` handler checks `client.sessionId` against the stored host ID and returns early if not the host
- **Target from payload**: `kickPlayer` uses `payload.targetSessionId` to identify the player to remove — not a client-supplied authority flag
- **Guard returns early**: Both privileged handlers return (do nothing) when the caller is not the host — they do not throw or allow partial execution
- **All hooks present**: Room implements `onCreate`, `onJoin`, `onLeave`, and `onDispose`

## Failure Conditions

- The host identity is accepted from a client payload field instead of being assigned server-side
- The host is not assigned on the first join or room creation
- `startMatch` handler does not verify `client.sessionId` against the stored host before executing
- `kickPlayer` handler does not verify `client.sessionId` against the stored host before executing
- `kickPlayer` does not use `payload.targetSessionId` to identify the target player
- Non-host callers can trigger partial execution or cause side effects in either privileged handler
- Any of the four required lifecycle hooks (`onCreate`, `onJoin`, `onLeave`, `onDispose`) is missing
