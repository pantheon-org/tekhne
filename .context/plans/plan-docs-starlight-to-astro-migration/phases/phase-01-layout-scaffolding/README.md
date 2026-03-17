# Phase 01 — Layout Scaffolding

## Goal

Build all new layouts and components while Starlight is still running, so the
cutover in Phase 2 can be a single atomic swap with no half-built UI surfaces.

## Gate

- [ ] `cd docs && bun run build` exits 0 (Starlight build still passes)
- [ ] `docs/src/styles/tokens.css` exists and defines both `[data-theme="dark"]` and `[data-theme="light"]` blocks with `--tk-*` variables
- [ ] `docs/src/styles/base.css` exists
- [ ] `docs/src/layouts/BaseLayout.astro`, `DocsLayout.astro`, `SkillLayout.astro` exist
- [ ] `docs/src/components/LeftNav.astro`, `ThemeToggle.astro`, `NavToggle.astro` exist
- [ ] Slug prototype script exits 0 and prints no collisions across all 537 IDs

## Dependencies

- Existing `docs/astro.config.mjs` (read `buildSidebarNode` / `buildDomainSidebar` logic to port)
- Existing `docs/src/content/content.config.ts` (read collection schema)
- No changes to any existing Starlight routes or components

## Tasks

### P01T01 — Add `@astrojs/mdx` dependency

Add `@astrojs/mdx` explicitly to `docs/package.json` so it survives Starlight removal.

[Full detail](tasks/task-P01T01-add-mdx-dependency.md)

### P01T02 — Create `tokens.css`

Define `--tk-*` CSS custom properties for both dark and light themes (zinc-950/green palette).

[Full detail](tasks/task-P01T02-create-tokens-css.md)

### P01T03 — Create `base.css`

CSS reset, base typography, and body background rules.

[Full detail](tasks/task-P01T03-create-base-css.md)

### P01T04 — Create `BaseLayout.astro`

HTML shell with `<head>` meta, font import, inline FOUT-prevention script, and named slots.

[Full detail](tasks/task-P01T04-create-base-layout.md)

### P01T05 — Create `DocsLayout.astro`

Two-column layout: `LeftNav` + content area for non-skill pages (index, tiles, reference).

[Full detail](tasks/task-P01T05-create-docs-layout.md)

### P01T06 — Create `SkillLayout.astro`

Three-column layout: `LeftNav` + content + right sidebar for skill pages and reference sub-pages.

[Full detail](tasks/task-P01T06-create-skill-layout.md)

### P01T07 — Create `LeftNav.astro`

Server-side domain-grouped skill tree with `localStorage` collapsed/expanded state, active-link highlight, and force-expanded ancestor.

[Full detail](tasks/task-P01T07-create-left-nav.md)

### P01T08 — Create `ThemeToggle.astro`

Sun/moon button that writes `data-theme` on `<html>` and persists to `localStorage`.

[Full detail](tasks/task-P01T08-create-theme-toggle.md)

### P01T09 — Create `NavToggle.astro`

Hamburger button that toggles the `LeftNav` overlay on mobile viewports.

[Full detail](tasks/task-P01T09-create-nav-toggle.md)

### P01T10 — Prototype slug mapping

Dry-run script that reads all `docs` collection entry IDs and prints computed `[...slug]` values; diff against 537 entries to confirm no collisions.

[Full detail](tasks/task-P01T10-prototype-slug-mapping.md)
