# Vitest Integration (Nx)

Use deterministic Vitest config that works with Nx caching and coverage outputs.

## Baseline Test Config

```ts
import { defineConfig } from 'vite';

export default defineConfig({
  test: {
    reporters: ['default'],
    globals: true,
    environment: 'jsdom',
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    cache: {
      dir: '../../node_modules/.vitest/apps/my-app',
    },
    coverage: {
      provider: 'v8',
      reportsDirectory: '../../coverage/apps/my-app',
    },
  },
});
```

## Environment Selection

- `jsdom`: UI components, browser APIs
- `node`: backend logic, utility libraries
- `happy-dom`: alternative DOM runtime

## Required Nx-Friendly Settings

- `reporters`: keep explicit (`['default']`)
- `cache.dir`: workspace-relative path
- `coverage.reportsDirectory`: workspace-relative path

## Separate Config Option

If test concerns become large, use a dedicated `vitest.config.ts` and keep `vite.config.ts` focused on build/serve.

## Validation Commands

```bash
bunx nx run <project>:test
bunx nx run <project>:test --coverage
```
