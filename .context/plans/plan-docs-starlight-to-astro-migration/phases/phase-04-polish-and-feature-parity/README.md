# Phase 04 — Polish and Feature Parity

## Goal

Verify and complete every UX detail needed to match (or exceed) the current
Starlight site before merging to main: left nav persistence, mobile overlay,
theme toggle, Pagefind search, edit-on-GitHub links, breadcrumbs, and 404.

## Gate

- [ ] `cd docs && bun run build` exits 0
- [ ] Left nav domain group state survives full page navigation (verified manually)
- [ ] Active ancestor group is force-expanded on load (verified manually)
- [ ] Hamburger overlay opens and closes; focus trap active while open (verified manually)
- [ ] Right sidebar collapses below mobile breakpoint (verified manually)
- [ ] Theme toggle changes theme; system preference respected on cold load; no FOUT (verified manually in both system dark and light modes)
- [ ] Pagefind index built; search returns results with `type:skill` metadata tags
- [ ] Edit-on-GitHub link present and correct on a skill page and a reference sub-page
- [ ] Breadcrumb present on a reference sub-page
- [ ] `404.astro` returns HTTP 404 (GitHub Pages config)

## Dependencies

- Phase 02 complete: all routes wired, build passes
- Phase 03 complete: no `--sl-*` references

## Tasks

### P04T01 — Verify left nav UX

Confirm `localStorage` persistence, active ancestor auto-expansion, and keyboard accessibility (Enter/Space on group toggles).

[Full detail](tasks/task-P04T01-verify-left-nav-ux.md)

### P04T02 — Verify mobile nav

Confirm hamburger overlay opens/closes `LeftNav`; focus trap in nav overlay; right sidebar collapses below breakpoint.

[Full detail](tasks/task-P04T02-verify-mobile-nav.md)

### P04T03 — Wire `ThemeToggle` into `BaseLayout`

Add `ThemeToggle` to the header slot in `BaseLayout.astro` and verify flash-prevention inline script fires before first paint.

[Full detail](tasks/task-P04T03-wire-theme-toggle.md)

### P04T04 — Configure Pagefind

Add `data-pagefind-meta="type:skill"` and `type:reference` attributes to layout wrappers; create `SearchBar.astro` with Pagefind UI.

[Full detail](tasks/task-P04T04-configure-pagefind.md)

### P04T05 — Add edit-on-GitHub link

Derive GitHub file URL from `entry.id` and render an edit link on every skill and reference page.

[Full detail](tasks/task-P04T05-add-edit-on-github-link.md)

### P04T06 — Add breadcrumb for reference sub-pages

Render "Skill name → Reference title" breadcrumb on reference sub-pages using `entry.id` segments.

[Full detail](tasks/task-P04T06-add-breadcrumb.md)

### P04T07 — Create `404.astro`

404 page using `BaseLayout` with a helpful message and a home link.

[Full detail](tasks/task-P04T07-create-404-page.md)
