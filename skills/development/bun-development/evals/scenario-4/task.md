# Task: Write a bun test Suite for a Config Loader Module

The following module loads and validates application configuration from a JSON file using Bun-native file I/O:

```typescript
// config-loader.ts
export interface AppConfig {
  port: number;
  logLevel: "debug" | "info" | "warn" | "error";
  databaseUrl: string;
}

export async function loadConfig(path: string): Promise<AppConfig> {
  const file = Bun.file(path);
  if (!(await file.exists())) {
    throw new Error(`Config file not found: ${path}`);
  }
  const raw = await file.text();
  const parsed = JSON.parse(raw);

  if (typeof parsed.port !== "number") {
    throw new Error("Config validation failed: port must be a number");
  }
  if (!["debug", "info", "warn", "error"].includes(parsed.logLevel)) {
    throw new Error("Config validation failed: invalid logLevel");
  }
  if (typeof parsed.databaseUrl !== "string" || parsed.databaseUrl === "") {
    throw new Error("Config validation failed: databaseUrl is required");
  }

  return parsed as AppConfig;
}
```

Write a `config-loader.test.ts` test file using `bun:test` that:
- Tests the happy path (valid config file)
- Tests error when file does not exist
- Tests each validation failure case
- Mocks `Bun.file` to avoid actual file system access
- Uses `describe`, `it`, `expect`, and `mock` from `bun:test`

Produce the complete test file.
