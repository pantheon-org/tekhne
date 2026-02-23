# Message Validation and Security

Use this reference when defining room message handlers.

## Validation Pattern

1. Parse payload with schema validation (for example Zod).
2. Enforce role or ownership constraints.
3. Enforce gameplay constraints (cooldowns, ranges, turn order).
4. Apply validated mutation to server state.

## Handler Skeleton

```typescript
room.onMessage("move", (client, payload) => {
  const parsed = moveSchema.safeParse(payload);
  if (!parsed.success) return;

  const player = room.state.players.get(client.sessionId);
  if (!player) return;

  if (!isValidDelta(player, parsed.data)) return;
  player.x = parsed.data.x;
  player.y = parsed.data.y;
});
```

## Security Rules

- Never trust client-derived score, inventory, or authority flags.
- Keep anti-cheat checks on server.
- Rate-limit spam-prone message types.
