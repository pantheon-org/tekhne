# Scenario 04: Fix Anti-Patterns in a Broken vite.config.ts for an Nx Project

## User Prompt

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

## Expected Behavior

1. Import `nxViteTsPaths` from `@nx/vite/plugins/nx-tsconfig-paths.plugin` and add `nxViteTsPaths()` to the plugins array alongside `react()`
2. Add `root: __dirname` and `cacheDir: '../../node_modules/.vite/libs/shared-lib'` at the top level
3. Change `build.outDir` to `'../../dist/libs/shared-lib'` (workspace-root-relative)
4. Change `test.cache.dir` to `'../../node_modules/.vitest/libs/shared-lib'` (workspace-root-relative)
5. Change `test.coverage.reportsDirectory` to `'../../coverage/libs/shared-lib'` (workspace-root-relative)
6. Add `build.rollupOptions.external` containing at minimum `'react'` and `'react-dom'`

## Success Criteria

- **nxViteTsPaths added to plugins**: `nxViteTsPaths` is imported and called in the plugins array alongside `react()`.
- **root: __dirname and cacheDir added**: `root` is set to `__dirname` and `cacheDir` is set to a workspace-root-relative path (`../../node_modules/.vite/libs/shared-lib`).
- **build.outDir corrected to workspace-relative path**: `build.outDir` is `../../dist/libs/shared-lib` (not `./dist`).
- **test.cache.dir corrected to workspace-relative path**: `test.cache.dir` is `../../node_modules/.vitest/libs/shared-lib` (not `./node_modules/.vitest`).
- **test.coverage.reportsDirectory corrected to workspace-relative path**: `test.coverage.reportsDirectory` is `../../coverage/libs/shared-lib` (not `./coverage`).
- **rollupOptions.external added for React**: `build.rollupOptions.external` includes `'react'` and `'react-dom'` at minimum.

## Failure Conditions

- `nxViteTsPaths` is still missing, leaving TypeScript path aliases broken in the Nx workspace
- `root: __dirname` or `cacheDir` is still absent
- `build.outDir` still uses `./dist` instead of the workspace-root-relative path
- `test.cache.dir` still uses `./node_modules/.vitest`, causing cache collisions across projects
- `test.coverage.reportsDirectory` still uses `./coverage`, causing coverage reports in the wrong location
- `rollupOptions.external` is absent, causing React to be bundled inside the library output
