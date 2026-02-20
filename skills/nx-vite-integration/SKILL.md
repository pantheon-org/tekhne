---
name: nx-vite-integration
description: |-
  Configure and integrate Vite build tool in Nx monorepo workspaces for applications and libraries. Covers TypeScript path resolution, framework plugins, asset handling, vitest configuration, library mode, and file replacements. Use proactively when setting up Vite in Nx projects or migrating from other build tools.
  
  Examples:
  - user: "Add Vite to my Nx React app" → configure vite.config.ts with nxViteTsPaths, React plugin, proper paths
  - user: "Set up Vite for my library" → configure library mode with dts plugin, external dependencies
  - user: "Configure vitest in Nx" → add test configuration with proper cache dirs and reporters
  - user: "Vite build not resolving TypeScript paths" → add nxViteTsPaths plugin
  - user: "Copy assets during Vite build" → use nxCopyAssetsPlugin for non-public assets
---

# Nx Vite Integration

Configure Vite as the build tool for Nx monorepo applications and libraries.

## Prerequisites

Install the Nx Vite plugin:

```bash
npm install -D @nx/vite
# or
bun add -d @nx/vite
```

## Quick Setup (Recommended)

Use the Nx generator to automatically configure Vite:

```bash
nx g @nx/vite:configuration <project-name>
```

This handles all configuration automatically. Use manual setup below only when needed.

## Manual Configuration

### For Applications

Create `vite.config.ts` in your app root (e.g., `apps/my-app/vite.config.ts`):

```typescript
/// <reference types='vitest' />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';

export default defineConfig({
  root: __dirname,
  build: {
    outDir: '../../dist/apps/my-app',
    reportCompressedSize: true,
    commonjsOptions: {
      transformMixedEsModules: true,
    },
  },
  cacheDir: '../../node_modules/.vite/apps/my-app',
  server: {
    port: 4200,
    host: 'localhost',
  },
  preview: {
    port: 4300,
    host: 'localhost',
  },
  plugins: [react(), nxViteTsPaths()],
  test: {
    reporters: ['default'],
    coverage: {
      reportsDirectory: '../../coverage/apps/my-app',
      provider: 'v8',
    },
    globals: true,
    cache: {
      dir: '../../node_modules/.vitest/apps/my-app',
    },
    environment: 'jsdom',
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
  },
});
```

### For Libraries

Create `vite.config.ts` in your library root (e.g., `libs/my-lib/vite.config.ts`):

```typescript
/// <reference types='vitest' />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import dts from 'vite-plugin-dts';
import * as path from 'path';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';

export default defineConfig({
  root: __dirname,
  cacheDir: '../../node_modules/.vite/my-lib',
  plugins: [
    react(),
    nxViteTsPaths(),
    dts({
      entryRoot: 'src',
      tsConfigFilePath: path.join(__dirname, 'tsconfig.lib.json'),
      skipDiagnostics: true,
    }),
  ],
  build: {
    outDir: '../../dist/libs/my-lib',
    reportCompressedSize: true,
    commonjsOptions: {
      transformMixedEsModules: true,
    },
    lib: {
      entry: 'src/index.ts',
      name: 'my-lib',
      fileName: 'index',
      formats: ['es'],
    },
    rollupOptions: {
      external: ['react', 'react-dom', 'react/jsx-runtime'],
    },
  },
  test: {
    globals: true,
    cache: {
      dir: '../../node_modules/.vitest/libs/my-lib',
    },
    environment: 'jsdom',
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    reporters: ['default'],
    coverage: {
      reportsDirectory: '../../coverage/libs/my-lib',
      provider: 'v8',
    },
  },
});
```

## Critical Configuration Requirements

### 1. TypeScript Path Resolution

**MUST use `nxViteTsPaths()` plugin** to resolve monorepo paths correctly:

```typescript
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';

export default defineConfig({
  plugins: [nxViteTsPaths()],
});
```

Without this, imports from other workspace packages will fail.

### 2. Root Path

**MUST set `root: __dirname`** for correct path resolution:

```typescript
export default defineConfig({
  root: __dirname,
});
```

### 3. Output Directory

**MUST set `outDir` relative to workspace root**:

- For `apps/my-app`: `outDir: '../../dist/apps/my-app'`
- For `libs/my-lib`: `outDir: '../../dist/libs/my-lib'`
- For `packages/my-pkg`: `outDir: '../dist/packages/my-pkg'`

Pattern: Navigate up to workspace root, then into `dist/` with same structure.

## Framework Plugins

### React

```bash
npm install -D @vitejs/plugin-react
```

```typescript
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
});
```

### Vue

```bash
npm install -D @vitejs/plugin-vue
```

```typescript
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  plugins: [vue()],
});
```

### Other Frameworks

Check [Vite plugin directory](https://vite.dev/plugins/) for Svelte, Solid, Preact, etc.

## Library-Specific Configuration

### TypeScript Declaration Files (DTS)

Install the plugin:

```bash
npm install -D vite-plugin-dts
```

#### Fast Builds (Skip Type Checking)

```typescript
import dts from 'vite-plugin-dts';
import { join } from 'path';

export default defineConfig({
  plugins: [
    dts({
      entryRoot: 'src',
      tsConfigFilePath: join(__dirname, 'tsconfig.lib.json'),
      skipDiagnostics: true,
    }),
  ],
});
```

Use when:
- Build speed is critical
- Type checking happens elsewhere (CI, IDE)
- Rapid iteration during development

#### Full Type Checking

```typescript
import dts from 'vite-plugin-dts';

export default defineConfig({
  plugins: [
    dts({
      root: '../../',
      entryRoot: 'libs/my-lib/src',
      tsConfigFilePath: 'libs/my-lib/tsconfig.lib.json',
      include: ['libs/my-lib/src/**/*.ts'],
      outputDir: 'dist/libs/my-lib',
      skipDiagnostics: false,
    }),
  ],
});
```

Use when:
- Type errors must block builds
- Publishing to npm
- Strict type guarantees required

### Library Mode Configuration

```typescript
export default defineConfig({
  build: {
    lib: {
      entry: 'src/index.ts',
      name: 'MyLib',
      fileName: 'index',
      formats: ['es'], // or ['es', 'cjs', 'umd']
    },
    rollupOptions: {
      external: ['react', 'react-dom', 'react/jsx-runtime'],
    },
  },
});
```

**Key points:**

- `entry`: Main export file (usually `src/index.ts`)
- `name`: Global variable name for UMD builds
- `fileName`: Output file name pattern
- `formats`: Output formats - `'es'` (ESM), `'cjs'` (CommonJS), `'umd'` (UMD)
- `external`: Dependencies NOT to bundle (peer dependencies)

## Asset Handling

### Public Directory

Files in `publicDir` (default: `public/`) are copied automatically.

### Non-Public Assets

Use `nxCopyAssetsPlugin` for assets outside `publicDir`:

```typescript
import { nxCopyAssetsPlugin } from '@nx/vite/plugins/nx-copy-assets.plugin';

export default defineConfig({
  plugins: [
    nxCopyAssetsPlugin(['*.md', 'LICENSE']),
  ],
});
```

Accepts glob patterns for flexible asset selection.

## Environment-Specific File Replacement

Replace files for different environments (production, staging, etc.):

```typescript
import { replaceFiles } from '@nx/vite/plugins/rollup-replace-files.plugin';

export default defineConfig({
  plugins: [
    replaceFiles([
      {
        replace: 'apps/my-app/src/environments/environment.ts',
        with: 'apps/my-app/src/environments/environment.prod.ts',
      },
    ]),
  ],
});
```

Common use case: Environment configurations in `src/environments/`.

## Vitest Configuration

```typescript
export default defineConfig({
  test: {
    globals: true,
    cache: {
      dir: '../../node_modules/.vitest/<project-root>',
    },
    environment: 'jsdom', // or 'node', 'happy-dom'
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    reporters: ['default'],
    coverage: {
      reportsDirectory: '../../coverage/<project-root>',
      provider: 'v8', // or 'istanbul'
    },
  },
});
```

**Required settings:**

- `reporters`: Nx needs explicit reporter configuration
- `environment`: Specify test environment (jsdom for DOM APIs)
- `cache.dir`: Relative to workspace root for Nx caching
- `coverage.reportsDirectory`: Relative to workspace root

## Common Patterns

### Multi-Format Library

```typescript
build: {
  lib: {
    entry: 'src/index.ts',
    name: 'MyLib',
    fileName: (format) => `index.${format}.js`,
    formats: ['es', 'cjs'],
  },
}
```

### Externalize All Dependencies

```typescript
build: {
  rollupOptions: {
    external: (id) => !id.startsWith('.') && !id.startsWith('/'),
  },
}
```

### Custom Dev Server Proxy

```typescript
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

### Optimize Dependencies

```typescript
optimizeDeps: {
  include: ['react', 'react-dom'],
  exclude: ['@my-org/my-lib'],
}
```

## Troubleshooting

### TypeScript paths not resolving

**Solution:** Add `nxViteTsPaths()` plugin.

### Build outputs to wrong directory

**Solution:** Check `outDir` is relative to workspace root, not project root.

### Library consumers can't find types

**Solution:** Ensure `vite-plugin-dts` is configured and `package.json` has `types` field.

### Tests not running

**Solution:** Verify `test.reporters` and `test.environment` are set.

### Assets not copied

**Solution:** Use `nxCopyAssetsPlugin` for non-public assets.

### Slow library builds

**Solution:** Enable `skipDiagnostics: true` in `vite-plugin-dts`.

## Path Depth Reference

Calculate `outDir` and `cacheDir` based on project depth:

| Project Location | Workspace Root Relative | Cache Dir | Out Dir |
|-----------------|-------------------------|-----------|---------|
| `apps/my-app` | `../../` | `../../node_modules/.vite/apps/my-app` | `../../dist/apps/my-app` |
| `libs/my-lib` | `../../` | `../../node_modules/.vite/libs/my-lib` | `../../dist/libs/my-lib` |
| `packages/pkg` | `../` | `../node_modules/.vite/packages/pkg` | `../dist/packages/pkg` |
| `my-app` (root) | `../` | `../node_modules/.vite/my-app` | `../dist/my-app` |

Count `../` segments from project root to workspace root.

## Complete Examples

### Nx + Vite + React App

```typescript
/// <reference types='vitest' />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';
import { replaceFiles } from '@nx/vite/plugins/rollup-replace-files.plugin';

export default defineConfig({
  root: __dirname,
  build: {
    outDir: '../../dist/apps/web-app',
    reportCompressedSize: true,
    commonjsOptions: { transformMixedEsModules: true },
  },
  cacheDir: '../../node_modules/.vite/apps/web-app',
  server: {
    port: 4200,
    host: 'localhost',
  },
  preview: {
    port: 4300,
    host: 'localhost',
  },
  plugins: [
    react(),
    nxViteTsPaths(),
    replaceFiles([
      {
        replace: 'apps/web-app/src/environments/environment.ts',
        with: 'apps/web-app/src/environments/environment.prod.ts',
      },
    ]),
  ],
  test: {
    reporters: ['default'],
    coverage: {
      reportsDirectory: '../../coverage/apps/web-app',
      provider: 'v8',
    },
    globals: true,
    cache: { dir: '../../node_modules/.vitest/apps/web-app' },
    environment: 'jsdom',
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
  },
});
```

### Nx + Vite + React Library (Published)

```typescript
/// <reference types='vitest' />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import dts from 'vite-plugin-dts';
import * as path from 'path';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';
import { nxCopyAssetsPlugin } from '@nx/vite/plugins/nx-copy-assets.plugin';

export default defineConfig({
  root: __dirname,
  cacheDir: '../../node_modules/.vite/libs/ui-components',
  plugins: [
    react(),
    nxViteTsPaths(),
    nxCopyAssetsPlugin(['*.md']),
    dts({
      entryRoot: 'src',
      tsConfigFilePath: path.join(__dirname, 'tsconfig.lib.json'),
      skipDiagnostics: false,
    }),
  ],
  build: {
    outDir: '../../dist/libs/ui-components',
    reportCompressedSize: true,
    commonjsOptions: { transformMixedEsModules: true },
    lib: {
      entry: 'src/index.ts',
      name: 'UiComponents',
      fileName: (format) => `index.${format}.js`,
      formats: ['es', 'cjs'],
    },
    rollupOptions: {
      external: ['react', 'react-dom', 'react/jsx-runtime'],
    },
  },
  test: {
    globals: true,
    cache: { dir: '../../node_modules/.vitest/libs/ui-components' },
    environment: 'jsdom',
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    reporters: ['default'],
    coverage: {
      reportsDirectory: '../../coverage/libs/ui-components',
      provider: 'v8',
    },
  },
});
```

## Related Resources

- [Vite Documentation](https://vite.dev/config/)
- [Nx Vite Plugin Reference](https://nx.dev/nx-api/vite)
- [vite-plugin-dts Documentation](https://www.npmjs.com/package/vite-plugin-dts)
- [Vitest Configuration](https://vitest.dev/config/)
