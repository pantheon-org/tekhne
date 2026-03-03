# Room Lifecycle and State

Use this reference for room hook sequencing and synchronized state design.

## Lifecycle Sequence

1. `onCreate(options)` initializes state and room metadata.
2. `onAuth(client, options, context)` validates join eligibility.
3. `onJoin(client, options, auth)` adds player state.
4. `onDrop(client)` handles transient disconnect.
5. `onLeave(client, code)` removes player permanently.
6. `onDispose()` performs cleanup/persistence.

## Schema Guidance

- Use Schema classes for every client-visible gameplay value.
- Prefer `MapSchema<Player>` keyed by `sessionId` for per-player state.
- Keep non-synced operational details outside gameplay state.

## Verification Checklist

- Joining a room creates exactly one player entry.
- Leaving removes that entry.
- Client receives updates only for Schema mutations.
