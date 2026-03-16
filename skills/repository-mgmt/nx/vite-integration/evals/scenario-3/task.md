# Task: Configure Vitest in a vite.config.ts for an Nx project

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
