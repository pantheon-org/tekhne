# Troubleshooting

Common Nx + Vite issues and direct fixes.

## TypeScript Paths Not Resolving

Symptoms:

- Imports like `@my-org/lib` fail during dev/build.

Fix:

- Add `nxViteTsPaths()` to `plugins`.
- Confirm project `tsconfig` path aliases are correct.

## Build Output in Wrong Directory

Symptoms:

- Artifacts appear inside project folder or unexpected dist path.

Fix:

- Set `root: __dirname`.
- Set `build.outDir` relative to workspace root (not project root).

## Library Types Missing for Consumers

Symptoms:

- Published package compiles but downstream TS projects fail type lookup.

Fix:

- Add `vite-plugin-dts` and verify output.
- Ensure package metadata points to types (`types`/`exports`).

## Assets Not Copied

Symptoms:

- Markdown/license/static files missing from output.

Fix:

- Use `nxCopyAssetsPlugin([...])` for non-public files.
- Keep `public/` assets in Vite public flow.

## Tests Not Running or Cached Correctly

Symptoms:

- Vitest behaves inconsistently between local and CI.

Fix:

- Set `test.reporters`, `test.environment`, and deterministic cache/coverage dirs.
- Validate with `nx run <project>:test`.

## Slow Library Builds

Symptoms:

- Build times grow significantly after adding DTS generation.

Fix:

- Set `skipDiagnostics: true` in local development.
- Keep strict diagnostics in CI or release checks.

## Migration Regressions from Webpack

Symptoms:

- Feature parity gaps after moving to Vite.

Fix:

- Identify Webpack-only behavior first.
- Replace loader/plugin behavior with Vite equivalents incrementally.
- Stop migration for unsupported critical features.
