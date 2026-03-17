# Phase 04 — Polish and Feature Parity

## Goal

Reach full feature parity with the current Starlight site: left nav UX, mobile
responsiveness, theme toggle wiring, Pagefind search, per-page edit links,
breadcrumbs for reference sub-pages, and a 404 page.

## Gate

- [ ] `bun run build` exits 0
- [ ] Left nav group state persists across page navigations (manual verification)
- [ ] Active ancestor domain auto-expands on deep links (manual verification)
- [ ] Hamburger overlay opens/closes on mobile viewport (manual verification)
- [ ] Theme toggle switches correctly; no flash on first load in both system-dark and system-light modes (manual verification)
- [ ] Pagefind search returns results (run `bun run build` then `npx pagefind --source docs/dist` and verify output)
- [ ] Edit-on-GitHub link appears on each page pointing to correct source file
- [ ] Breadcrumb renders on reference sub-pages
- [ ] `docs/src/pages/404.astro` exists

## Dependencies

- Phase 03 complete (no `--sl-*` or Starlight utility class references)

## Tasks

### P04T01 — Left nav UX

Verify `localStorage` persistence across navigations; active ancestor
auto-expansion; keyboard accessibility (Enter/Space on group toggles).

[Full detail](tasks/task-P04T01-left-nav-ux.md)

### P04T02 — Mobile responsiveness

Hamburger overlay opens/closes `LeftNav`. Focus trap in nav overlay. Right
sidebar collapses below breakpoint.

[Full detail](tasks/task-P04T02-mobile-responsiveness.md)

### P04T03 — Theme toggle wiring

Wire `ThemeToggle` into `BaseLayout` header. Verify flash-prevention script
fires before paint in both system dark and light modes.

[Full detail](tasks/task-P04T03-theme-toggle-wiring.md)

### P04T04 — Pagefind integration

Add `data-pagefind-meta="type:skill"` to skill page wrapper in `SkillLayout`.
Add `data-pagefind-meta="type:reference"` to reference pages. Create
`SearchBar.astro` with Pagefind UI.

[Full detail](tasks/task-P04T04-pagefind-integration.md)

### P04T05 — Edit-on-GitHub link

Add per-page edit-on-GitHub link using `entry.id` to construct the GitHub
file URL.

[Full detail](tasks/task-P04T05-edit-on-github-link.md)

### P04T06 — Breadcrumb for reference sub-pages

Render breadcrumb (skill name → reference title) on reference sub-pages.

[Full detail](tasks/task-P04T06-breadcrumb-reference-pages.md)

### P04T07 — Create `src/pages/404.astro`

404 page using `BaseLayout`.

[Full detail](tasks/task-P04T07-create-404-page.md)
