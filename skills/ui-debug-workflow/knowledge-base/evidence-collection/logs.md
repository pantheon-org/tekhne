# Logs

## Types of Logs

### Build Logs

Capture complete build output:

```bash
npm run build 2>&1 | tee build-output.log
# or
bun run build 2>&1 | tee build-output.log
```

**What to look for**:
- Build errors or warnings
- Bundle sizes
- Build time
- Optimization notes

### Server Logs

Capture server startup and runtime logs:

```bash
npm start 2>&1 | tee server-output.log
```

**What to look for**:
- Server port
- Initialization errors
- Runtime warnings
- Request logs

### Test Execution Logs

```bash
./test-script.sh 2>&1 | tee test-execution.log
```

### Browser Console Logs

#### Via Playwright

```typescript
import { test } from "@playwright/test";
import fs from "fs";

test("capture console logs", async ({ page }) => {
  // Capture all console messages
  page.on("console", (msg) => {
    fs.appendFileSync("console.log", `[${msg.type()}] ${msg.text()}\n`);
  });

  // Capture errors
  page.on("pageerror", (error) => {
    fs.appendFileSync("console.log", `[ERROR] ${error.message}\n${error.stack}\n`);
  });

  await page.goto("http://localhost:3000");
  // ... run tests
});
```

#### Via agent-browser

```bash
# Check for console errors
agent-browser --session test eval "
  console.log('Test checkpoint 1');
  // Your test code
  'Checkpoint reached';
"
```

## Git Context

Always include git information in reports:

```bash
# Current branch and commit
git branch --show-current > git-branch.txt
git log -1 --oneline > git-commit.txt

# Changes made
git diff main...HEAD > changes.diff

# File list
git diff --name-status main...HEAD > changed-files.txt
```

## Log Organization

Suggested directory structure:

```
./docs/change-fix/
└── 2026-02-05-feature-name/
    ├── baseline/
    │   ├── build-output.log
    │   ├── server-output.log
    │   └── console.log
    ├── changed/
    │   ├── build-output.log
    │   ├── server-output.log
    │   └── console.log
    └── git/
        ├── changes.diff
        ├── changed-files.txt
        └── commit-info.txt
```

## Related Documentation

- [Screenshots](./screenshots.md) - Visual evidence
- [DOM Snapshots](./dom-snapshots.md) - Page structure
- [Git Context](./git-context.md) - Version control info
