# Phase 02 — Route Migration & Starlight Cutover

## Goal

Switch all pages to new layouts and remove Starlight in a single atomic step on
a feature branch. After this phase there are no Starlight imports anywhere and
all 537+ pages render via the new layout system.

## Gate

- [ ] `bun run build` (inside `docs/`) exits 0 with no Starlight imports
- [ ] `grep -r "starlight" docs/src --include="*.ts" --include="*.astro" --include="*.mjs" | grep -v node_modules` returns empty
- [ ] All 537+ pages render (verify via build artifact count matching pre-cutover baseline)
- [ ] CI build passes on feature branch before merge

## Dependencies

- Phase 01 complete (all layout components and slug prototype validated)

## Tasks

### P02T01 — Replace `docsSchema` in `content.config.ts`

Replace `docsSchema` from `@astrojs/starlight/schema` with a plain Zod baseline.
Decide: (a) build custom TOC generator for reference sub-pages or (b) drop TOC
on non-skill pages, and implement accordingly.

[Full detail](tasks/task-P02T01-replace-docs-schema.md)

### P02T02 — Refactor `Astro.locals.starlightRoute` out of all three components

Hard blocker. Replace `entry` source in `SkillPageTitle`, `SkillTabs`, and
`SkillSidebar` from `Astro.locals.starlightRoute` to an explicit `entry` prop.
Remove `starlightRoute.toc` reference from `SkillSidebar`.

[Full detail](tasks/task-P02T02-refactor-starlightroute-out-of-components.md)

### P02T03 — Create `src/pages/index.astro`

Homepage using `DocsLayout`. Replaces `index.mdx`. Port hero and feature cards
as native Astro components (remove `@astrojs/starlight/components` imports).

[Full detail](tasks/task-P02T03-create-index-page.md)

### P02T04 — Create `src/pages/tiles.astro`

Dynamic tiles catalog using `DocsLayout`. Queries `docs` collection for all skill
pages, groups by domain, renders skill cards with grade badge, eval count, and
Tessl registry link.

[Full detail](tasks/task-P02T04-create-tiles-page.md)

### P02T05 — Extend `prelink.mjs` to inject tile metadata

For each SKILL.md being copied, walk up the source directory tree to find nearest
`tile.json`, extract `publishedUrl` and `version`, inject as `tilePublishedUrl`
and `tileVersion` into frontmatter. Add fields to `skillExtraSchema`.

[Full detail](tasks/task-P02T05-extend-prelink-tile-metadata.md)

### P02T06 — Create `src/pages/skills/[...slug].astro`

`getStaticPaths()` from `docs` collection; strip leading `skills/` prefix from
IDs. Uses `SkillLayout` for skill pages, `DocsLayout` for reference sub-pages,
passing `entry` as explicit prop. Renders markdown via `render(entry)`.

[Full detail](tasks/task-P02T06-create-slug-route.md)

### P02T07 — Uninstall `@astrojs/starlight`

Remove `@astrojs/starlight` from `docs/package.json`.

[Full detail](tasks/task-P02T07-uninstall-starlight.md)

### P02T08 — Update `astro.config.mjs`

Remove `starlight()` integration and sidebar/components config block. Replace
with `mdx()` integration only.

[Full detail](tasks/task-P02T08-update-astro-config.md)

### P02T09 — Delete old Starlight content routes

Delete `src/content/docs/tiles.md` and `src/content/docs/index.mdx`.

[Full detail](tasks/task-P02T09-clean-up-old-routes.md)

### P02T10 — Remove `virtual:starlight` imports from `SkillSidebar.astro`

Remove `virtual:starlight/components/*` imports. Define mobile right-sidebar
pattern (e.g. `<details>` disclosure or bottom drawer triggered by a button in
`SkillLayout` on narrow viewports).

[Full detail](tasks/task-P02T10-remove-starlight-virtual-imports.md)
