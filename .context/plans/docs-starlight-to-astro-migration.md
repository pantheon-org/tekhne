---
date: 2026-03-17
title: Docs — Starlight to Base Astro Migration
---

## Goal

Replace the Starlight wrapper in `docs/` with a bespoke Astro layout, removing the
framework constraint while preserving the content pipeline, skill pages, and tabs UI.

## Decisions

| Decision | Choice |
|----------|--------|
| Theme default | System preference (`prefers-color-scheme`), user can override, persisted in `localStorage` |
| Left nav state | Persisted per domain group in `localStorage`; active page's ancestor always force-expanded |
| Pagefind scope | Index everything; tag pages with Pagefind metadata (`type:skill`, `type:reference`); filter chip in Phase 5 |
| Route coexistence | Phase 2 is hard cutover — Starlight and new routes do not coexist |
| Tiles page | Dynamic `tiles.astro` querying `docs` collection at build time; `prelink.mjs` injects tile registry data from `tile.json` |
| URL tab state | Client-side `history.replaceState` with `?tab=` param; restored on page load — in scope for Phase 5 |

## Current state

- Astro + Starlight site under `docs/`
- Three Starlight component overrides: `PageTitle → SkillPageTitle`, `MarkdownContent → SkillTabs`, `PageSidebar → SkillSidebar`
- Content pipeline: `docs/scripts/prelink.mjs` copies `skills/` → `docs/src/content/docs/skills/` and injects frontmatter (`skillAudit`, `skillAudits`, `skillEvals`, `skillRefs`)
- Custom components: `SkillPageTitle.astro`, `SkillTabs.astro`, `SkillSidebar.astro`
- Left sidebar auto-generated from `skills/` directory tree in `astro.config.mjs`
- CSS: Starlight design tokens (`--sl-*`), custom overrides in `src/styles/custom.css`
- `content.config.ts` imports `docsSchema` from `@astrojs/starlight/schema` — hard Starlight dependency
- `@astrojs/mdx` is not listed in `package.json`; bundled transitively through Starlight today

## What survives unchanged

- `docs/src/content/` collection structure and `content.config.ts` schemas (except `docsSchema` replacement)
- `docs/scripts/prelink.mjs` preprocessing (extended in Phase 2 only)
- `SkillTabs.astro` component logic (tabs, panels, `sk-tab-change` event dispatch) — **requires `Astro.locals.starlightRoute` → `Astro.props` refactor; not truly zero-change**
- `SkillSidebar.astro` picker logic (minus `virtual:starlight` imports and `Astro.locals.starlightRoute`)
- `SkillPageTitle.astro` header card (minus `Astro.locals.starlightRoute`)
- GitHub Pages deploy workflow

> **Note on tab sync:** `SkillSidebar` already synchronises its panel visibility to the active tab via the `sk-tab-change` custom event. This event-driven sync survives Phase 2 unchanged. The `?tab=` URL-persistence feature deferred to Phase 5 is a separate, additive concern.

---

## Migration phases

### Phase 1 — Layout scaffolding (Starlight still active)

Build all new layouts and components without touching Starlight routes.

**Tasks:**

1. Add `@astrojs/mdx` explicitly to `package.json` (currently only available via Starlight's transitive dependency — will break on Starlight removal)
2. Create `src/styles/tokens.css` — define both `[data-theme="dark"]` and `[data-theme="light"]` token sets with `--tk-*` variables. Mirror the current zinc-950/green palette for dark; define a clean light counterpart.
3. Create `src/styles/base.css` — CSS reset, base typography, body background
4. Create `src/layouts/BaseLayout.astro`:
   - HTML shell with `<head>` meta, font import, token + base CSS
   - All stylesheet/asset `href` values must use Astro `import` + Vite asset handling — **no hardcoded absolute paths** — to respect `base: "/tekhne"` in `astro.config.mjs`
   - Inline `<script>` at top of `<head>` to apply stored theme **before first paint** (prevents flash-of-wrong-theme): reads `localStorage`, falls back to `prefers-color-scheme`
   - Slots: `header`, `default`
5. Create `src/layouts/DocsLayout.astro` — two-column: `LeftNav` + content area (for non-skill pages, index, tiles)
6. Create `src/layouts/SkillLayout.astro` — three-column: `LeftNav` + content + right sidebar (for skill pages and reference sub-pages)
7. Create `src/components/LeftNav.astro`:
   - Server-side: queries `docs` collection and builds domain-grouped skill tree (port `buildSidebarNode` / `buildDomainSidebar` logic from `astro.config.mjs`)
   - Active page link highlighted via `Astro.url`
   - Active page's ancestor domain group always force-expanded regardless of stored state
   - Domain group collapsed/expanded state persisted in `localStorage` (key per domain slug)
   - Client `<script>` handles toggle clicks, localStorage reads/writes
8. Create `src/components/ThemeToggle.astro` — sun/moon icon button, updates `data-theme` on `<html>` and writes to `localStorage`
9. Create `src/components/NavToggle.astro` — hamburger button, toggles `LeftNav` overlay on mobile
10. **Prototype slug mapping** — write a throwaway script (or `bun run astro check`-safe dry-run) that reads all `docs` collection entry IDs and prints the computed `[...slug]` values; diff against the 537 entries to confirm no collisions or missing pages **before Phase 2 cutover**

**Done when:** `bun run build` still passes; new layout components exist but are unused by routes; slug prototype validates all 537 IDs.

---

### Phase 2 — Route migration and Starlight cutover (single step)

Switch all pages to new layouts and remove Starlight simultaneously. **Execute on a feature branch; merge only after CI build passes.**

**Tasks:**

1. Replace `docsSchema` in `content.config.ts` with a plain Zod baseline (**hard blocker for Starlight removal**):
   ```ts
   import { z } from "astro:content";
   const baseSchema = z.object({ title: z.string(), description: z.string().optional() });
   schema: withSkillTitle(baseSchema.merge(skillExtraSchema))
   ```
   Note: `docsSchema` also provided `toc` data consumed by `SkillSidebar` for non-skill pages. Decide at this task whether to (a) build a custom TOC generator for reference sub-pages or (b) drop TOC on non-skill pages — and implement accordingly before proceeding.
2. **Refactor `Astro.locals.starlightRoute` out of all three custom components** — this is a hard blocker; all three components (`SkillPageTitle`, `SkillTabs`, `SkillSidebar`) read `entry` from `Astro.locals.starlightRoute`, which is Starlight-only. Replace with an explicit `entry` prop passed from `[...slug].astro`:
   ```ts
   // In each component frontmatter:
   interface Props { entry: CollectionEntry<"docs"> }
   const { entry } = Astro.props;
   ```
   Also remove `starlightRoute.toc` reference from `SkillSidebar` (line 132).
3. Create `src/pages/index.astro` — homepage using `DocsLayout`, replaces `index.mdx`. Port hero and feature cards as native Astro components (remove `@astrojs/starlight/components` imports).
4. Create `src/pages/tiles.astro` — dynamic tiles catalog using `DocsLayout`:
   - Queries `docs` collection for all skill pages (`id.endsWith("/skill")`)
   - Groups by domain
   - Renders skill cards with: title, description, grade badge (from `skillAudit`), eval count (from `skillEvals`), Tessl registry link (from `tilePublishedUrl` frontmatter — see task 5)
5. Extend `prelink.mjs` to inject `tilePublishedUrl` and `tileVersion` into each skill's frontmatter during the copy step:
   - For each SKILL.md being copied, walk up the source directory tree to find the nearest `tile.json` (check parent, grandparent, great-grandparent — nesting depth varies by skill)
   - Read `tile.json` and extract `publishedUrl` (or equivalent registry URL field) and `version`
   - Inject as `tilePublishedUrl` and `tileVersion` into the frontmatter block
   - Add `tilePublishedUrl: z.string().optional()` and `tileVersion: z.string().optional()` to `skillExtraSchema` in `content.config.ts`
6. Create `src/pages/skills/[...slug].astro`:
   - `getStaticPaths()` from `docs` collection; use entry IDs as-is (the glob loader in `content.config.ts` produces IDs without extensions, e.g. `skills/infrastructure/terraform/generator/skill`) — strip only the leading `skills/` prefix
   - Uses `SkillLayout` for skill pages (`entry.id.endsWith("/skill")`), passing `entry` as prop
   - Uses `DocsLayout` for reference sub-pages, passing `entry` as prop
   - Renders markdown via `render(entry)` and passes `Content` to layout slot
7. Uninstall `@astrojs/starlight` from `package.json`
8. Remove `starlight()` integration from `astro.config.mjs`; replace with `mdx()` integration only
9. Remove `components` override block and sidebar config from `astro.config.mjs`
10. Delete `src/content/docs/tiles.md` and `src/content/docs/index.mdx`
11. Remove `virtual:starlight/components/*` imports from `SkillSidebar.astro`; define the mobile right-sidebar pattern (e.g. a `<details>` disclosure or bottom drawer triggered by a button in `SkillLayout` on narrow viewports)

**Done when:** `bun run build` succeeds with no Starlight imports; all 537+ pages render; CI build passes on feature branch.

---

### Phase 3 — CSS token migration

Replace all `--sl-*` variable references in custom components with `--tk-*` tokens.

**Tasks:**

1. Audit all `--sl-*` usages across `SkillTabs.astro`, `SkillSidebar.astro`, `SkillPageTitle.astro`
2. Replace each with corresponding `--tk-*` token from `tokens.css`
3. Replace Starlight utility classes (`sl-hidden`, `lg:sl-block`) with plain CSS media queries
4. Delete `src/styles/custom.css`; merge any surviving rules into `tokens.css` or relevant component `<style>` blocks
5. Verify dark and light themes render correctly for all components

**Done when:** no `--sl-*` or `sl-` class references remain; both themes pass visual inspection.

---

### Phase 4 — Polish and feature parity

**Tasks:**

1. **Left nav UX**: verify `localStorage` persistence across page navigations; verify active ancestor auto-expansion; keyboard accessibility (Enter/Space on group toggles)
2. **Mobile**: hamburger overlay opens/closes `LeftNav`; focus trap in nav overlay; right sidebar collapses below breakpoint
3. **Theme toggle**: wire `ThemeToggle` into `BaseLayout` header; verify flash-prevention script fires before paint in both system dark and light modes
4. **Pagefind**: add `data-pagefind-meta="type:skill"` to skill page wrapper in `SkillLayout`; add `data-pagefind-meta="type:reference"` to reference pages; create `SearchBar.astro` with Pagefind UI
5. **Edit-on-GitHub** link per page using `entry.id` to construct the GitHub file URL
6. **Breadcrumb** for reference sub-pages (skill name → reference title)
7. **404.astro** page using `BaseLayout`

**Done when:** feature parity with current Starlight site; UX details verified manually on desktop and mobile.

---

### Phase 5 — Registry enhancements

**Tasks:**

1. **URL-driven tab state**: on tab click, call `history.replaceState` to set `?tab=skill|audit|evals|refs`; on page load, read `?tab` param, activate matching tab, and dispatch `sk-tab-change` event
2. **Tiles page filters**: domain filter chips (client-side show/hide); grade filter (`A`, `B+`, `B`, `C`); skill count badge per domain group
3. **Pagefind filter chip** in `SearchBar` to toggle between all results and skills-only (`type:skill`)
4. **Upgraded skill cards**: grade badge colour, eval count, audit date, domain tag

---

## File structure after migration

```
docs/
  src/
    layouts/
      BaseLayout.astro
      DocsLayout.astro
      SkillLayout.astro
    components/
      LeftNav.astro
      NavToggle.astro
      SearchBar.astro
      ThemeToggle.astro
      SkillPageTitle.astro      ← unchanged
       SkillTabs.astro           ← --sl-* vars replaced with --tk-*; Astro.locals.starlightRoute → Astro.props
       SkillSidebar.astro        ← virtual:starlight imports removed; Astro.locals.starlightRoute → Astro.props
    pages/
      index.astro               ← replaces index.mdx
      tiles.astro               ← replaces tiles.md (dynamic)
      404.astro
      skills/
        [...slug].astro
    styles/
      tokens.css                ← --tk-* vars, dark + light themes
      base.css                  ← reset + typography
    content/                    ← unchanged
  scripts/
    prelink.mjs                 ← extended: injects tilePublishedUrl + tileVersion
  astro.config.mjs              ← Starlight removed; defineConfig + mdx() only
```

## Risk areas

| Risk | Mitigation |
|------|-----------|
| `docsSchema` hard dependency | Explicit Phase 2 task; plain Zod replacement defined above |
| `@astrojs/mdx` missing after Starlight removal | Explicit Phase 1 task to add it to `package.json` |
| `Astro.locals.starlightRoute` in all three components | Explicit Phase 2 task 2; replace with `Astro.props.entry` before Starlight removal |
| `docsSchema` TOC data loss | Phase 2 task 1 requires decision: build custom TOC generator or drop TOC on reference sub-pages |
| Left nav tree generation | Port `buildSidebarNode` logic incrementally; diff output against current sidebar before cutover |
| Slug mapping in `[...slug].astro` | Phase 1 task 10: prototype dry-run script validates all 537 IDs before cutover; collection IDs have no file extension (strip `skills/` prefix only) |
| Flash of wrong theme | Inline script in `<head>` before any CSS; test in both system dark and light modes |
| Mobile nav regression | Build mobile-first in Phase 1; verify on real device before Phase 2 cutover |
| Pagefind without Starlight | Works standalone — https://pagefind.app/docs/astro/ |
| `base: "/tekhne"` asset paths | Use Astro `import` + Vite asset handling in `BaseLayout`; never hardcode absolute paths |
| `tile.json` traversal in `prelink.mjs` | Walk up directory tree per SKILL.md; nesting depth varies — test across all 12 domains before Phase 2 cutover |
| Phase 2 breakage with no rollback path | Execute on feature branch; merge only after CI build gate passes |

## Definition of done

- `bun run build` succeeds; no Starlight imports anywhere
- All 537+ pages render correctly
- Left nav matches current domain/skill tree; group state persists across navigation; active ancestor auto-expands
- Dark/light theme toggle works; system preference respected on first load; no flash
- Skill tabs + sidebar picker work as before
- `?tab=` URL param syncs with active tab
- Search works via Pagefind
- Tiles page dynamically reflects live collection data
- GitHub Pages deploy workflow unchanged
