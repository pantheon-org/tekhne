---
name: nx-vite-integration
description: |
  Configure and integrate Vite in Nx monorepos for applications and libraries.
  Covers vite.config.ts setup, framework plugins, TypeScript path resolution,
  asset copying, library mode builds, and Vitest integration.

  Use when: adding Vite to an Nx project, migrating from Webpack,
  configuring Vitest, fixing tsconfig path resolution, or setting up library mode.

  Triggers: "add vite", "nx vite", "vite setup", "vite.config.ts",
  "vitest config", "library mode", "nxViteTsPaths", "copy assets",
  "vite path aliases", "migrate webpack to vite"

  Examples:
  - user: "Add Vite to this Nx app" -> install plugin and configure vite.config.ts
  - user: "Vitest is failing in Nx" -> fix test config and cache/coverage paths
  - user: "Path aliases break in Vite" -> add nxViteTsPaths plugin
  - user: "Set up Vite for my Nx library" -> configure lib mode + dts + externals
---

# Nx Vite Integration

Configure Vite as the build tool in Nx workspaces with predictable, cache-friendly defaults.

## Scope

In scope:

- Nx projects using Vite for app and library builds
- Monorepo TypeScript path resolution
- Vitest setup in Nx projects
- Library packaging patterns with `vite-plugin-dts`

Out of scope:

- Webpack-only features (complex module federation, loader chains)
- Legacy IE11 browser support
- Non-Nx monorepo tooling decisions

## Use When

- Setting up Vite for a new Nx app or library
- Migrating an existing Nx project from Webpack to Vite
- Configuring Vitest with Nx-compatible cache/coverage paths
- Fixing path alias resolution across workspace packages
- Setting up library mode with externalized dependencies

## When NOT to Use

- Project depends on Webpack-specific loaders/plugins that have no Vite equivalent
- You require advanced module federation patterns that are currently Webpack-first
- Browser support requirements include IE11

## Quick Reference

| Topic | Reference |
| --- | --- |
| Vite Config Patterns | [references/vite-config-patterns.md](references/vite-config-patterns.md) |
| Nx Vite Plugins | [references/nx-vite-plugin-reference.md](references/nx-vite-plugin-reference.md) |
| Library Mode | [references/library-mode-guide.md](references/library-mode-guide.md) |
| Vitest Integration | [references/vitest-integration.md](references/vitest-integration.md) |
| Troubleshooting | [references/troubleshooting.md](references/troubleshooting.md) |

## Mindset

- Prefer generator-first setup, then manual edits for edge cases.
- Configure workspace-relative paths first; path mistakes cause most Nx + Vite failures.
- Treat library externalization as mandatory for publishable packages.
- Keep build and test concerns explicit and easy to verify.

## Workflow

### Step 1: Install Dependencies

Preconditions:

- Nx workspace exists
- Node.js 18+ and package manager available

Commands:

```bash
bunx nx add @nx/vite
bun add -d vite
```

Framework plugin (choose one):

```bash
bun add -d @vitejs/plugin-react
# or
bun add -d @vitejs/plugin-vue
# or
bun add -d @sveltejs/vite-plugin-svelte
```

Expected result:

- `@nx/vite` and framework plugin installed

### Step 2: Generate or Configure Vite

Preconditions:

- Dependencies installed
- Target project exists in Nx

Recommended command:

```bash
bunx nx g @nx/vite:configuration <project-name>
```

Manual baseline (`vite.config.ts`):

```ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';

export default defineConfig({
  root: __dirname,
  plugins: [react(), nxViteTsPaths()],
});
```

Expected result:

- `vite.config.ts` exists and resolves workspace path aliases

### Step 3: Configure Nx Targets

Preconditions:

- Vite config exists

Ensure target config supports:

- `build`
- `serve`
- `test` (Vitest)

Expected result:

- `nx run <project>:build` and `nx run <project>:test` are valid targets

### Step 4: Add Project-Specific Features

For apps:

- Set `outDir` and `cacheDir` relative to workspace root
- Add `nxCopyAssetsPlugin` if non-public files must be copied
- Add environment replacement plugin when needed

For libraries:

- Configure `build.lib`
- Externalize peer deps in `rollupOptions.external`
- Add `vite-plugin-dts` for declaration output

Expected result:

- Build outputs to `dist/` with correct dependency boundaries

### Step 5: Verify Setup

Commands:

```bash
bunx nx run <project>:build
bunx nx run <project>:test
```

Expected result:

- Build succeeds
- Tests run with configured environment and cache directories

## Anti-Patterns

### NEVER: Skip `nxViteTsPaths()` in Nx Monorepos

Why:

- Workspace path aliases (`@org/lib`) fail without Nx path plugin integration.

Bad:

```ts
export default defineConfig({
  plugins: [react()],
});
```

Good:

```ts
export default defineConfig({
  plugins: [react(), nxViteTsPaths()],
});
```

### NEVER: Bundle All Dependencies in Library Mode

Why:

- Bloats bundles and breaks consumer-side tree shaking.

Bad:

```ts
build: {
  lib: { entry: 'src/index.ts' },
}
```

Good:

```ts
build: {
  lib: { entry: 'src/index.ts' },
  rollupOptions: {
    external: ['react', 'react-dom', /^@my-org\//],
  },
}
```

### NEVER: Use Project-Relative Cache/Coverage Paths that Ignore Workspace Root

Why:

- Nx caching and output tracking become inconsistent across projects.

Bad:

```ts
test: {
  cache: { dir: './node_modules/.vitest' },
  coverage: { reportsDirectory: './coverage' },
}
```

Good:

```ts
test: {
  cache: { dir: '../../node_modules/.vitest/apps/my-app' },
  coverage: { reportsDirectory: '../../coverage/apps/my-app' },
}
```

### NEVER: Mix Unrelated Build and Test Concerns Without Intention

Why:

- Large mixed configs become hard to debug and maintain.

Bad:

- One growing config with ad-hoc overrides and no sectioning.

Good:

- Keep sections intentional (`build`, `server`, `test`, plugins) and validate each step.

## Constraint Guidelines

### Hard Constraints

- Always include `nxViteTsPaths()` in Nx monorepos
- Always set `root: __dirname`
- Always configure `outDir` and `cacheDir` relative to workspace root
- Always externalize peer dependencies for published libraries
- Always define `test.reporters`, `test.environment`, and deterministic cache/coverage paths

### Flexible Choices

- Framework plugin (React, Vue, Svelte, Solid, etc.)
- Output formats (`es`, `cjs`, `umd`) based on consumers
- CSS tooling (PostCSS, Tailwind, vanilla)
- DTS strictness (`skipDiagnostics: true/false`) based on CI strategy

## Verification Commands

```bash
bunx nx run <project>:build
bunx nx run <project>:test
bunx @biomejs/biome check skills/nx-vite-integration/
bunx markdownlint-cli2 "skills/nx-vite-integration/**/*.md"
```
