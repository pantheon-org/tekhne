# Bun Runtime API Notes for Nx Projects

Use this reference when implementing runtime details after selecting the Nx Bun integration workflow.

## Bun.serve in Nx Applications

- Prefer explicit host and port configuration for deterministic local and CI behavior.
- Keep request handling logic in project libs when possible to preserve testability.

Example:

```ts
Bun.serve({
  port: Number(process.env.PORT ?? 3000),
  fetch() {
    return new Response("ok");
  },
});
```

## Bun.file and Bun.write

- Use `Bun.file()` for read operations and avoid unnecessary Node fs wrappers.
- Use `Bun.write()` for generated artifacts when writing from scripts or codegen tasks.

```ts
const text = await Bun.file("./config.json").text();
await Bun.write("./dist/status.txt", "ready\n");
```

## SQLite Integration Notes

- Use prepared statements for user-provided values.
- Wrap multi-statement write flows in explicit transaction blocks.

```ts
const stmt = db.prepare("SELECT * FROM users WHERE id = ?");
const user = stmt.get(userId);
```

## WebSocket Considerations

- Validate payloads before state mutation.
- Keep room/session lifecycle logic separate from protocol adapters.
- Ensure disconnect handling is deterministic in long-running Bun servers.

## Runtime Caveats

- Container memory limits can expose different behavior vs local hot reload.
- Verify production startup path without watch mode before release.
