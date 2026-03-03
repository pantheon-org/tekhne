# Migration from ESLint and Prettier

Use this reference to move to Biome without rule or formatter conflicts.

## Sequence

1. Remove ESLint/Prettier CI jobs for files Biome will own.
2. Initialize and customize `biome.json`.
3. Run Biome `check` and address high-signal findings.
4. Run Biome `format --write` once across the target scope.
5. Update CI to enforce Biome only for that scope.

## Example Command Sequence

```bash
bunx @biomejs/biome init
```

```bash
bunx @biomejs/biome check .
```

```bash
bunx @biomejs/biome check . --write
```

```bash
bunx @biomejs/biome format . --write
```

## Guardrails

- Migrate by scope (folder or package) if repository risk is high.
- Keep suppressions local and documented.
- Avoid parallel Biome + Prettier formatting on the same files.
