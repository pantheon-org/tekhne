# Scenario 7: Review an executor entrypoint

## User Prompt

Review the executor below and rewrite it so it follows the skill's guidance. The plugin team wants executors to stay thin, the build task to be cache-friendly, and parallel runs not to be slowed down.

```ts
import * as fs from 'fs';

export default async function runExecutor(options: { src: string; out: string }) {
  const raw = fs.readFileSync(options.src, 'utf8');
  // 180 lines of parsing, validation and transformation follow here
  const result = raw.split('\n').map((l) => l.trim().toUpperCase()).join('\n');
  fs.writeFileSync(options.out, result);
  return true;
}
```

The target in `project.json` currently reads `"executor": "../../tools/executors:transform"` and declares no `outputs`.

## Repo state

- Plugin package name is `@acme/tools`.
- Executor is registered in `executors.json` as `transform`.
