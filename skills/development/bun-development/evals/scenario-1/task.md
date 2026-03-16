# Task: Migrate Node.js File I/O to Bun-Native APIs

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
