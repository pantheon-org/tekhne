# Scenario 03: Configure Vitest in a vite.config.ts for an Nx Project

## User Prompt

You are adding Vitest configuration to an existing React app `my-app` at `apps/my-app` in an Nx monorepo. The app already has a working Vite setup but lacks a test configuration.

Produce the complete updated `apps/my-app/vite.config.ts` with a full Vitest `test` section added.

## Requirements

The existing config (shown below) must be preserved. Add a `test` section with:

- `globals: true`
- `environment: 'jsdom'`
- `cache.dir` set to `../../node_modules/.vitest/apps/my-app` (workspace-root-relative, NOT `./node_modules/.vitest`)
- `coverage.reportsDirectory` set to `../../coverage/apps/my-app` (workspace-root-relative, NOT `./coverage`)
- `coverage.provider: 'v8'`
- `reporters: ['default']`

Add the Vitest triple-slash reference comment at the top of the file: `/// <reference types='vitest' />`

## Existing vite.config.ts

```ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';

export default defineConfig({
  root: __dirname,
  cacheDir: '../../node_modules/.vite/apps/my-app',
  plugins: [react(), nxViteTsPaths()],
  build: {
    outDir: '../../dist/apps/my-app',
    reportCompressedSize: true,
  },
  server: { port: 4200, host: 'localhost' },
  preview: { port: 4300, host: 'localhost' },
});
```

## Output

Produce the updated file `apps/my-app/vite.config.ts`.

## Expected Behavior

1. Add `/// <reference types='vitest' />` at the very top of the file
2. Preserve `root`, `cacheDir`, `plugins`, `build`, `server`, and `preview` exactly as in the starting file
3. Set `test.environment` to `'jsdom'`
4. Set `test.cache.dir` to `'../../node_modules/.vitest/apps/my-app'` (workspace-root-relative)
5. Set `test.coverage.reportsDirectory` to `'../../coverage/apps/my-app'` (workspace-root-relative)
6. Set `test.globals` to `true` and include `'default'` in `test.reporters`

## Success Criteria

- **Vitest triple-slash reference added**: The file begins with `/// <reference types='vitest' />`.
- **Existing config sections preserved**: `root`, `cacheDir`, `plugins`, `build`, `server`, and `preview` are identical to the starting file.
- **test.environment set to jsdom**: `test.environment` is `'jsdom'`.
- **test.cache.dir uses workspace-relative path**: `test.cache.dir` is `'../../node_modules/.vitest/apps/my-app'`, not `'./node_modules/.vitest'` or similar project-relative path.
- **test.coverage.reportsDirectory uses workspace-relative path**: `test.coverage.reportsDirectory` is `'../../coverage/apps/my-app'`, not `'./coverage'`.
- **test.globals and reporters present**: `test.globals` is `true` and `test.reporters` includes `'default'`.

## Failure Conditions

- The Vitest triple-slash reference comment is missing from the top of the file
- Any existing config section (`root`, `cacheDir`, `plugins`, `build`, `server`, `preview`) is modified or removed
- `test.environment` is missing or set to a value other than `'jsdom'`
- `test.cache.dir` uses a project-relative path instead of `../../node_modules/.vitest/apps/my-app`
- `test.coverage.reportsDirectory` uses a project-relative path instead of `../../coverage/apps/my-app`
- `test.globals` is absent or `test.reporters` does not include `'default'`
