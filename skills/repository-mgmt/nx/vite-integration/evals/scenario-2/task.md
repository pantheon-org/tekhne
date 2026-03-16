# Task: Create a library-mode vite.config.ts with DTS and peer dependency externalization

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
