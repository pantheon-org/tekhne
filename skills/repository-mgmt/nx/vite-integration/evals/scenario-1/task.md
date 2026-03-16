# Task: Create a vite.config.ts for a React application in an Nx monorepo

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
