# Scenario 02: Create a Library-Mode vite.config.ts with DTS and Peer Dependency Externalization

## User Prompt

You are setting up a publishable TypeScript React component library named `ui-components` at `libs/ui-components` in an Nx monorepo. This library needs to be built in library mode, generate TypeScript declarations, and externalize React and all `@my-org/*` packages.

Produce the file `libs/ui-components/vite.config.ts`.

## Requirements

- Import `defineConfig` from `vite`, React plugin from `@vitejs/plugin-react`, `nxViteTsPaths` from `@nx/vite/plugins/nx-tsconfig-paths.plugin`, and `dts` from `vite-plugin-dts`.
- Set `root: __dirname`.
- Set `cacheDir` to `../../node_modules/.vite/libs/ui-components`.
- Include `react()`, `nxViteTsPaths()`, and the DTS plugin in plugins.
- The DTS plugin must use:
  - `entryRoot: 'src'`
  - `tsConfigFilePath` pointing to `tsconfig.lib.json` (use `join(__dirname, 'tsconfig.lib.json')`)
  - `skipDiagnostics: true`
- Configure `build`:
  - `outDir: '../../dist/libs/ui-components'`
  - `lib.entry: 'src/index.ts'`
  - `lib.name: 'UiComponents'`
  - `lib.fileName: 'index'`
  - `lib.formats: ['es']`
  - `rollupOptions.external`: externalize `'react'`, `'react-dom'`, `'react/jsx-runtime'`, and all packages matching `/^@my-org\//`

## Output

Produce the file `libs/ui-components/vite.config.ts`.

## Expected Behavior

1. Include `nxViteTsPaths()` in the plugins array
2. Import `dts` from `vite-plugin-dts` and configure it with `entryRoot`, `tsConfigFilePath`, and `skipDiagnostics`
3. Set `build.lib.entry` to `'src/index.ts'`, `name` to `'UiComponents'`, and `formats` to `['es']`
4. Externalize `'react'`, `'react-dom'`, and `'react/jsx-runtime'` in `rollupOptions.external`
5. Externalize all `@my-org/` scoped packages using a regex or pattern in `rollupOptions.external`
6. Use workspace-root-relative paths: `cacheDir` as `../../node_modules/.vite/libs/ui-components` and `build.outDir` as `../../dist/libs/ui-components`

## Success Criteria

- **nxViteTsPaths included in plugins**: `nxViteTsPaths()` is present in the plugins array.
- **DTS plugin configured correctly**: `vite-plugin-dts` is imported and configured with `entryRoot`, `tsConfigFilePath`, and `skipDiagnostics`.
- **build.lib configured with correct entry, name, and formats**: `build.lib.entry` is `'src/index.ts'`, `name` is `'UiComponents'`, `formats` is `['es']`.
- **React packages externalized**: `rollupOptions.external` includes `'react'`, `'react-dom'`, and `'react/jsx-runtime'`.
- **@my-org scoped packages externalized**: `rollupOptions.external` includes a regex or pattern matching `/^@my-org\//`.
- **Workspace-relative cacheDir and outDir**: `cacheDir` is `../../node_modules/.vite/libs/ui-components` and `build.outDir` is `../../dist/libs/ui-components`.

## Failure Conditions

- `nxViteTsPaths()` is absent from plugins, breaking TypeScript path aliases
- DTS plugin is missing or misconfigured (no `entryRoot`, `tsConfigFilePath`, or `skipDiagnostics`)
- `build.lib` is misconfigured: wrong entry, name, or formats
- React packages (`react`, `react-dom`, `react/jsx-runtime`) are not externalized, causing them to be bundled
- `@my-org/` scoped packages are not externalized, causing dependencies to be bundled
- `cacheDir` or `build.outDir` uses project-relative paths instead of workspace-root-relative paths
