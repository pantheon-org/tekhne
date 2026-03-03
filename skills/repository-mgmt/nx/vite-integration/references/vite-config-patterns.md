# Vite Config Patterns (Nx)

Use these patterns when creating `vite.config.ts` for Nx apps and libraries.

## App Baseline (React)

```ts
/// <reference types='vitest' />
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
    commonjsOptions: { transformMixedEsModules: true },
  },
  server: { port: 4200, host: 'localhost' },
  preview: { port: 4300, host: 'localhost' },
});
```

## App Baseline (Vue)

```ts
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';

export default defineConfig({
  root: __dirname,
  plugins: [vue(), nxViteTsPaths()],
  cacheDir: '../../node_modules/.vite/apps/my-app',
  build: { outDir: '../../dist/apps/my-app' },
});
```

## App Baseline (Svelte)

```ts
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';

export default defineConfig({
  root: __dirname,
  plugins: [svelte(), nxViteTsPaths()],
  cacheDir: '../../node_modules/.vite/apps/my-app',
  build: { outDir: '../../dist/apps/my-app' },
});
```

## Proxy Pattern

```ts
server: {
  proxy: {
    '/api': {
      target: 'http://localhost:3000',
      changeOrigin: true,
      rewrite: (path) => path.replace(/^\/api/, ''),
    },
  },
}
```

## Environment Replacement Pattern

```ts
import { replaceFiles } from '@nx/vite/plugins/rollup-replace-files.plugin';

plugins: [
  replaceFiles([
    {
      replace: 'apps/my-app/src/environments/environment.ts',
      with: 'apps/my-app/src/environments/environment.prod.ts',
    },
  ]),
]
```

## Path Depth Rule

Always compute paths from project root to workspace root:

- `apps/my-app` and `libs/my-lib` usually use `../../`
- `packages/pkg` may use `../`

Example mapping:

| Project | cacheDir | outDir |
| --- | --- | --- |
| `apps/my-app` | `../../node_modules/.vite/apps/my-app` | `../../dist/apps/my-app` |
| `libs/my-lib` | `../../node_modules/.vite/libs/my-lib` | `../../dist/libs/my-lib` |
| `packages/pkg` | `../node_modules/.vite/packages/pkg` | `../dist/packages/pkg` |
