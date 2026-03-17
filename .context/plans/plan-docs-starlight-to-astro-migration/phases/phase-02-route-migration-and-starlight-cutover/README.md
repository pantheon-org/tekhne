# Phase 02 — Route Migration and Starlight Cutover

## Goal

Switch all pages to the new layouts and remove Starlight in a single feature-branch
commit, so there is never a broken intermediate state. This is the point of no return.

## Gate

- [ ] `cd docs && bun run build` exits 0 with no `@astrojs/starlight` imports in the build graph
- [ ] `grep -r "starlight" docs/src` returns no results (excluding node_modules)
- [ ] `grep -r "Astro.locals.starlightRoute" docs/src` returns no results
- [ ] `grep -r "docsSchema" docs/src` returns no results
- [ ] `grep -r "virtual:starlight" docs/src` returns no results
- [ ] Static path count in build output equals 537 or more pages
- [ ] CI build passes on the feature branch

## Dependencies

- Phase 01 complete: all layouts and components exist and `bun run build` passes
- Phase 01 P01T10 slug prototype validated all 537 IDs with no collisions
- Feature branch created before any Phase 2 file edits

## Tasks

### P02T01 — Replace `docsSchema` with plain Zod baseline

Remove the `@astrojs/starlight/schema` import from `content.config.ts` and replace with a plain `z.object` baseline. Includes decision: build custom TOC generator or drop TOC on reference sub-pages.

[Full detail](tasks/task-P02T01-replace-docs-schema-with-zod.md)

### P02T02 — Refactor `Astro.locals.starlightRoute` from all three components

Replace `Astro.locals.starlightRoute` reads in `SkillPageTitle`, `SkillTabs`, and `SkillSidebar` with an explicit `entry: CollectionEntry<"docs">` prop. Hard blocker.

[Full detail](tasks/task-P02T02-refactor-starlight-route-from-components.md)

### P02T03 — Create `src/pages/index.astro`

Homepage using `DocsLayout`. Port hero and feature cards as native Astro components; remove all `@astrojs/starlight/components` imports.

[Full detail](tasks/task-P02T03-create-index-astro.md)

### P02T04 — Create `src/pages/tiles.astro`

Dynamic tiles catalog using `DocsLayout`. Queries `docs` collection at build time, groups by domain, renders skill cards.

[Full detail](tasks/task-P02T04-create-tiles-astro.md)

### P02T05 — Extend `prelink.mjs` to inject tile URLs

Walk up the directory tree from each SKILL.md to find the nearest `tile.json` and inject `tilePublishedUrl` and `tileVersion` into frontmatter.

[Full detail](tasks/task-P02T05-extend-prelink-tile-urls.md)

### P02T06 — Create `src/pages/skills/[...slug].astro`

`getStaticPaths` from `docs` collection. Routes skill pages to `SkillLayout` and reference sub-pages to `DocsLayout`.

[Full detail](tasks/task-P02T06-create-slug-route.md)

### P02T07 — Uninstall `@astrojs/starlight`

Remove `@astrojs/starlight` from `docs/package.json` and `bun.lock`.

[Full detail](tasks/task-P02T07-uninstall-starlight.md)

### P02T08 — Remove `starlight()` integration from `astro.config.mjs`

Replace `starlight()` with `mdx()` integration only.

[Full detail](tasks/task-P02T08-remove-starlight-integration.md)

### P02T09 — Remove Starlight config blocks from `astro.config.mjs`

Remove `components` override block and sidebar config; remove `buildSidebarNode` / `buildDomainSidebar` (now live in `LeftNav.astro`).

[Full detail](tasks/task-P02T09-remove-starlight-config-blocks.md)

### P02T10 — Delete obsolete Starlight content files

Delete `src/content/docs/tiles.md` and `src/content/docs/index.mdx` (replaced by `pages/tiles.astro` and `pages/index.astro`).

[Full detail](tasks/task-P02T10-delete-starlight-content-files.md)

### P02T11 — Fix `SkillSidebar` mobile pattern

Remove `virtual:starlight/components/*` imports; implement mobile right-sidebar pattern (e.g. `<details>` disclosure or bottom drawer).

[Full detail](tasks/task-P02T11-fix-skill-sidebar-mobile.md)
