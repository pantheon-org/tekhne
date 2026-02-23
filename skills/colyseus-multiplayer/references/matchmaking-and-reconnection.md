# Matchmaking and Reconnection

Use this reference for room discovery, filtering, and reconnect workflows.

## Matchmaking Rules

- Set room metadata in `onCreate` for mode/region/skill filters.
- Use metadata and seat availability in matchmaking selection.
- Keep metadata consistent with room runtime state.

## Reconnection Pattern

1. In `onDrop`, mark player as disconnected.
2. Allow reconnection for a bounded timeout.
3. Restore session if token is valid.
4. Expire and remove player if timeout passes.

## Example Timeout Policy

- Casual match: 20-30 seconds grace.
- Ranked match: 10-15 seconds grace.
- Tournament: strict policy with admin override.

## Operational Checks

- Reconnected clients rebind to prior player state.
- Duplicate sessions for same player are rejected.
- Expired reconnection tokens are invalidated.
