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

## Mindset

Nx and Vite each have their own opinions about project structure. The integration works by making Vite defer to Nx for path resolution and task orchestration while Nx defers to Vite for bundling. The critical bridge is `nxViteTsPaths()` — without it, workspace path aliases silently resolve to nothing and builds fail with cryptic module-not-found errors.

Think in two layers: **workspace-level** (Nx executor targets, cache dirs, project boundaries) and **build-level** (Vite plugins, rollup options, output formats). Workspace-level decisions live in `project.json`; build-level decisions live in `vite.config.ts`. Keep them separate and each layer becomes predictable.

Library mode differs meaningfully from app mode: apps bundle everything together, libraries externalize peer deps to avoid duplication in consumers. The distinction determines whether `rollupOptions.external` and `vite-plugin-dts` are needed.

## When to Use

| Scenario | Use This Skill |
|---|---|
| Adding Vite to an existing Nx app or lib | Yes |
| Migrating from Webpack or CRA to Vite inside Nx | Yes |
| Setting up Vitest alongside an existing Vite config | Yes |
| Fixing `@org/lib` path aliases failing in Vite builds | Yes |
| Configuring library mode (`build.lib`) with DTS output | Yes |
| Setting up a standalone Vite project (no Nx) | No — use Vite docs directly |
| Adding a non-Vite bundler (esbuild standalone, Rollup) | No — this skill is Nx + Vite specific |

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

## References

- [Vite Config Patterns](references/vite-config-patterns.md) — vite.config.ts options, plugin setup, and resolver configuration for Nx projects
- [Nx Vite Plugins](references/nx-vite-plugin-reference.md) — `@nx/vite` executor options, inferred task configuration, and caching setup
- [Library Mode](references/library-mode-guide.md) — `build.lib` configuration, entry point patterns, and DTS generation
- [Vitest Integration](references/vitest-integration.md) — Vitest configuration, coverage providers, and Nx test executor setup
- [Troubleshooting](references/troubleshooting.md) — common Vite + Nx errors, path resolution failures, and DTS issues
