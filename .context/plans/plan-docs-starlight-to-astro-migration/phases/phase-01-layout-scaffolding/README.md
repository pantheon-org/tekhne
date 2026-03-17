# Phase 01 — Layout Scaffolding

## Goal

Build all new layouts and components without touching Starlight routes. Starlight
remains fully active; new files are created but unused by any route. A prototype
slug-mapping script validates all 537 collection IDs before Phase 2 cutover.

## Gate

- [ ] `bun run build` (inside `docs/`) exits 0 — Starlight still active, no regressions
- [ ] `docs/src/styles/tokens.css` exists with `[data-theme="dark"]` and `[data-theme="light"]` blocks containing `--tk-*` variables
- [ ] `docs/src/layouts/BaseLayout.astro`, `DocsLayout.astro`, and `SkillLayout.astro` all exist
- [ ] `docs/src/components/LeftNav.astro`, `ThemeToggle.astro`, and `NavToggle.astro` all exist
- [ ] Slug prototype script exits 0 and prints 537 IDs with no collisions

## Dependencies

None — Phase 1 is the entry point.

## Tasks

### P01T01 — Add `@astrojs/mdx` dependency

Add `@astrojs/mdx` explicitly to `docs/package.json`. Currently only available
transitively through Starlight; will break on Starlight removal.

[Full detail](tasks/task-P01T01-add-mdx-dependency.md)

### P01T02 — Create `src/styles/tokens.css`

Define `[data-theme="dark"]` and `[data-theme="light"]` `--tk-*` token sets.
Mirror current zinc-950/green palette for dark; define a clean light counterpart.

[Full detail](tasks/task-P01T02-create-tokens-css.md)

### P01T03 — Create `src/styles/base.css`

CSS reset, base typography, and body background. No component-specific rules.

[Full detail](tasks/task-P01T03-create-base-css.md)

### P01T04 — Create `src/layouts/BaseLayout.astro`

HTML shell: `<head>` meta, font import, token + base CSS. Inline theme-init
`<script>` before first paint (prevents flash-of-wrong-theme). Slots: `header`,
`default`. All asset hrefs via Astro `import` + Vite (respects `base: "/tekhne"`).

[Full detail](tasks/task-P01T04-create-base-layout.md)

### P01T05 — Create `src/layouts/DocsLayout.astro`

Two-column layout: `LeftNav` + content area. Used for non-skill pages, index,
and tiles.

[Full detail](tasks/task-P01T05-create-docs-layout.md)

### P01T06 — Create `src/layouts/SkillLayout.astro`

Three-column layout: `LeftNav` + content + right sidebar. Used for skill pages
and reference sub-pages.

[Full detail](tasks/task-P01T06-create-skill-layout.md)

### P01T07 — Create `src/components/LeftNav.astro`

Server-side: queries `docs` collection, builds domain-grouped skill tree (port
`buildSidebarNode`/`buildDomainSidebar` from `astro.config.mjs`). Active link
highlighted via `Astro.url`. Ancestor domain always force-expanded. Group state
persisted in `localStorage` per domain slug. Client `<script>` handles toggle
clicks.

[Full detail](tasks/task-P01T07-create-left-nav.md)

### P01T08 — Create `src/components/ThemeToggle.astro`

Sun/moon icon button; updates `data-theme` on `<html>` and writes to `localStorage`.

[Full detail](tasks/task-P01T08-create-theme-toggle.md)

### P01T09 — Create `src/components/NavToggle.astro`

Hamburger button; toggles `LeftNav` overlay on mobile.

[Full detail](tasks/task-P01T09-create-nav-toggle.md)

### P01T10 — Prototype slug mapping

Write a script (or `astro check`-safe dry-run) that reads all `docs` collection
entry IDs and prints computed `[...slug]` values. Diff against 537 entries to
confirm no collisions or missing pages before Phase 2 cutover.

[Full detail](tasks/task-P01T10-prototype-slug-mapping.md)
