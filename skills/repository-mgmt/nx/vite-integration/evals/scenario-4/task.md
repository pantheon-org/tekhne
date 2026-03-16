# Task: Fix anti-patterns in a broken vite.config.ts for an Nx project

You have been given a `vite.config.ts` that was written without following Nx Vite integration best practices. It contains several anti-patterns from the skill's prohibited list.

Identify all anti-patterns and produce a corrected version of the file.

## Broken vite.config.ts

```ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  build: {
    outDir: './dist',
    lib: {
      entry: 'src/index.ts',
      name: 'MyLib',
      formats: ['es'],
    },
  },
  test: {
    cache: { dir: './node_modules/.vitest' },
    coverage: { reportsDirectory: './coverage' },
  },
});
```

## Context

- This config is for a library project named `shared-lib` at `libs/shared-lib` in an Nx monorepo.
- The library has React as a peer dependency that should NOT be bundled.
- The workspace root is two directory levels above the project root (`../../`).

## Requirements

Fix ALL of the following problems:

1. `nxViteTsPaths()` is missing from plugins.
2. `root: __dirname` is missing.
3. `cacheDir` is missing (should be workspace-root-relative).
4. `build.outDir` is project-relative `./dist` instead of workspace-root-relative.
5. Vitest `test.cache.dir` is project-relative instead of workspace-root-relative.
6. Vitest `test.coverage.reportsDirectory` is project-relative instead of workspace-root-relative.
7. Library mode has no `rollupOptions.external` for React.

## Output

Produce the corrected file `libs/shared-lib/vite.config.ts`.
