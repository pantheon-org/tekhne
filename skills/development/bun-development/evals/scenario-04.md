# Scenario 04: Write a bun:test Suite for a Config Loader Module

## User Prompt

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

## Expected Behavior

1. Import `describe`, `it`, `expect`, and `mock` (or `spyOn`) exclusively from `'bun:test'`; no Jest, Vitest, or Mocha imports
2. Write at least one happy-path test that asserts `port`, `logLevel`, and `databaseUrl` values on the returned `AppConfig`
3. Write a test that verifies `loadConfig` throws an error matching `'Config file not found'` when the file does not exist
4. Write separate tests for all three validation failure cases: invalid port type, invalid logLevel value, and missing/empty databaseUrl
5. Use `mock()` or `spyOn()` to intercept `Bun.file` calls and return controlled test doubles — no actual files read from disk
6. Organize tests with `describe` blocks and `it` cases with descriptive names

## Success Criteria

- **bun:test imports used exclusively**: The test file imports `describe`, `it`, `expect`, and `mock` (or `spyOn`) from `'bun:test'`; no Jest, Vitest, or Mocha imports
- **Happy path test present**: At least one test verifies that `loadConfig` returns a correctly typed `AppConfig` when given a valid config; asserts `port`, `logLevel`, and `databaseUrl` values
- **File not found error tested**: A test verifies that `loadConfig` throws an error matching `'Config file not found'` when the file does not exist
- **All three validation failure cases tested**: Separate tests cover: invalid port type, invalid logLevel value, and missing/empty databaseUrl; each asserts the correct error message
- **Bun.file mocked to avoid real filesystem access**: The test file uses `mock()` or `spyOn()` to intercept `Bun.file` calls and return controlled test doubles
- **describe/it structure used**: Tests are organized with `describe` blocks and `it` test cases; test names are descriptive

## Failure Conditions

- Agent imports from `jest`, `vitest`, or `mocha` instead of `bun:test`
- Agent omits the happy path test or does not assert `port`, `logLevel`, and `databaseUrl`
- Agent does not test the file-not-found error case
- Agent omits one or more of the three validation failure cases
- Agent reads actual files from disk instead of mocking `Bun.file`
- Agent uses a flat list of `test()` calls without `describe`/`it` organization
