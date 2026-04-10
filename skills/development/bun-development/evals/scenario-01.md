# Scenario 01: Migrate Node.js File I/O to Bun-Native APIs

## User Prompt

The following TypeScript module was written for Node.js and is being migrated to a Bun project. Replace all Node.js `fs` module calls with their Bun-native equivalents. The project uses `bun` as its runtime (confirmed in `package.json`).

```typescript
import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const CONFIG_DIR = "./config";
const CONFIG_PATH = join(CONFIG_DIR, "settings.json");

export function loadConfig(): Record<string, unknown> {
  if (!existsSync(CONFIG_PATH)) {
    return {};
  }
  const raw = readFileSync(CONFIG_PATH, "utf8");
  return JSON.parse(raw);
}

export async function saveConfig(config: Record<string, unknown>): Promise<void> {
  if (!existsSync(CONFIG_DIR)) {
    mkdirSync(CONFIG_DIR, { recursive: true });
  }
  writeFileSync(CONFIG_PATH, JSON.stringify(config, null, 2), "utf8");
}

export async function appendLog(message: string): Promise<void> {
  const logPath = "./logs/app.log";
  let existing = "";
  if (existsSync(logPath)) {
    existing = readFileSync(logPath, "utf8");
  }
  writeFileSync(logPath, existing + message + "\n", "utf8");
}
```

Produce the migrated TypeScript module using Bun-native APIs. Explain each substitution made and why Bun's API is preferred.

## Expected Behavior

1. Remove all `import` statements from `node:fs` — no `fs` module references remain in the output
2. Replace all `readFileSync` calls with `await Bun.file(path).text()` or equivalent Bun file API
3. Replace all `writeFileSync` calls with `await Bun.write(path, content)`
4. Handle `existsSync` replacement using `try/catch` around `Bun.file().text()`, `Bun.file(path).exists()`, or an appropriate Bun-native alternative, with reasoning explained
5. Explain each API substitution and state why the Bun-native API is preferred (lazy loading, no Node.js shims, performance)
6. Update `loadConfig` to be async since it now uses `await`; update all exported function signatures to reflect async/await changes

## Success Criteria

- **No node:fs imports in output**: The produced module contains no `import` from `'node:fs'` or `require('fs')`; all file operations use Bun APIs
- **readFileSync replaced with Bun.file().text()**: All `readFileSync` calls are replaced with `await Bun.file(path).text()` or equivalent Bun.file() API; async/await is used correctly
- **writeFileSync replaced with Bun.write()**: All `writeFileSync` calls are replaced with `await Bun.write(path, content)`
- **existsSync handled correctly**: `existsSync` is either replaced with a try/catch around `Bun.file().text()`, or the agent uses `Bun.file(path).exists()` or an appropriate Bun-native alternative; reasoning is explained
- **Each substitution explained**: Agent explains each API substitution and states why the Bun-native API is preferred
- **Async signatures updated where needed**: `loadConfig` is updated to be async since it now uses `await`; all exported function signatures reflect async/await changes correctly

## Failure Conditions

- Agent retains any `import` from `node:fs` in the output module
- Agent uses `readFileSync` or a synchronous equivalent instead of `await Bun.file().text()`
- Agent uses `writeFileSync` instead of `await Bun.write()`
- Agent does not handle the `existsSync` replacement and leaves a non-Bun call in place
- Agent produces the migrated module without explaining each substitution
- Agent leaves `loadConfig` as a synchronous function when it now requires `await`
