# Scenario 01: Create a vite.config.ts for a React Application in an Nx Monorepo

## User Prompt

You are adding Vite to a React application named `web-app` located at `apps/web-app` in an Nx monorepo. The workspace root is two directory levels above the project root.

Produce the file `apps/web-app/vite.config.ts`.

## Requirements

- Import `defineConfig` from `vite`.
- Import the React plugin from `@vitejs/plugin-react`.
- Import `nxViteTsPaths` from `@nx/vite/plugins/nx-tsconfig-paths.plugin`.
- Set `root: __dirname`.
- Set `cacheDir` to the workspace-root-relative path: `../../node_modules/.vite/apps/web-app`.
- Include both `react()` and `nxViteTsPaths()` in the `plugins` array.
- Configure `build.outDir` to `../../dist/apps/web-app`.
- Configure `build.reportCompressedSize: true`.
- Configure `server` with port `4200` and `host: 'localhost'`.
- Configure `preview` with port `4300` and `host: 'localhost'`.

## Output

Produce the file `apps/web-app/vite.config.ts`.

## Expected Behavior

1. Import `nxViteTsPaths` from `@nx/vite/plugins/nx-tsconfig-paths.plugin` and include `nxViteTsPaths()` in the plugins array
2. Import `react` from `@vitejs/plugin-react` and include `react()` in the plugins array
3. Set `root: __dirname` at the top level of `defineConfig`
4. Set `cacheDir` to `"../../node_modules/.vite/apps/web-app"` (workspace-root-relative, not project-relative)
5. Set `build.outDir` to `"../../dist/apps/web-app"` (workspace-root-relative)
6. Set `server.port` to `4200` and `preview.port` to `4300`

## Success Criteria

- **nxViteTsPaths imported and included in plugins**: The config imports `nxViteTsPaths` from `@nx/vite/plugins/nx-tsconfig-paths.plugin` and calls `nxViteTsPaths()` in the plugins array.
- **react() plugin included**: The config imports `react` from `@vitejs/plugin-react` and calls `react()` in the plugins array.
- **root: __dirname set**: The config sets `root: __dirname` at the top-level of `defineConfig`.
- **cacheDir uses workspace-relative path**: `cacheDir` is set to `"../../node_modules/.vite/apps/web-app"` (not a project-relative path).
- **build.outDir uses workspace-relative path**: `build.outDir` is set to `"../../dist/apps/web-app"`.
- **server and preview ports configured**: `server.port` is `4200` and `preview.port` is `4300`.

## Failure Conditions

- `nxViteTsPaths` is missing from imports or not called in the plugins array, breaking TypeScript path aliases
- `react()` plugin is missing from the plugins array
- `root: __dirname` is absent, causing Vite to resolve paths incorrectly
- `cacheDir` uses a project-relative path (e.g. `./node_modules/.vite`) instead of the workspace-root-relative path
- `build.outDir` uses a project-relative path instead of the workspace-root-relative path
- Server or preview ports are incorrect or missing
