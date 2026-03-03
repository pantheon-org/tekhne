# Nx Vite Plugin Reference

Core Nx-specific plugins for Vite projects.

## `nxViteTsPaths`

Import:

```ts
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';
```

Purpose:

- Resolves workspace TypeScript path aliases in monorepos.

Use:

```ts
plugins: [react(), nxViteTsPaths()]
```

When required:

- Any Nx workspace with cross-project imports (`@scope/lib`).

## `nxCopyAssetsPlugin`

Import:

```ts
import { nxCopyAssetsPlugin } from '@nx/vite/plugins/nx-copy-assets.plugin';
```

Purpose:

- Copies non-public assets into output during build.

Use:

```ts
plugins: [nxCopyAssetsPlugin(['*.md', 'LICENSE'])]
```

Notes:

- `public/` files are already handled by Vite without this plugin.
- Use for files outside standard public asset flow.

## `replaceFiles` (Rollup Replace Files Plugin)

Import:

```ts
import { replaceFiles } from '@nx/vite/plugins/rollup-replace-files.plugin';
```

Purpose:

- Replaces files for environment-specific builds.

Use:

```ts
plugins: [
  replaceFiles([
    {
      replace: 'apps/web/src/environments/environment.ts',
      with: 'apps/web/src/environments/environment.prod.ts',
    },
  ]),
]
```

## Plugin Ordering Guidance

Suggested order:

1. Framework plugin (`react`, `vue`, `svelte`)
2. `nxViteTsPaths()`
3. Asset/environment plugins
4. DTS plugin (for libraries)

Reason:

- Framework and path resolution should stabilize module graph before output-time plugins.
