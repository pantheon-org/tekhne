# Phase 05 — Registry Enhancements

## Goal

Add the registry-specific UX features deferred from Phase 2: URL-driven tab state,
tiles page filters, Pagefind filter chip, and upgraded skill cards with grade
badges and eval counts.

## Gate

- [ ] `cd docs && bun run build` exits 0
- [ ] Navigating to a skill page with `?tab=audit` activates the audit tab (verified manually)
- [ ] Refreshing a `?tab=audit` URL restores the same tab (verified manually)
- [ ] Domain filter chips on tiles page show/hide the correct skill groups (verified manually)
- [ ] Grade filter on tiles page filters correctly (`A`, `B+`, `B`, `C`) (verified manually)
- [ ] Pagefind filter chip in search bar toggles skills-only results (verified manually)
- [ ] Skill cards on tiles page show grade badge colour, eval count, and domain tag

## Dependencies

- Phase 04 complete: feature parity with Starlight site
- `skillAudit`, `skillEvals` frontmatter fields populated by `prelink.mjs`
- `tilePublishedUrl` frontmatter field populated by Phase 02 `prelink.mjs` extension

## Tasks

### P05T01 — URL-driven tab state

On tab click, call `history.replaceState` to set `?tab=skill|audit|evals|refs`. On page load, read `?tab` param, activate matching tab, dispatch `sk-tab-change` event.

[Full detail](tasks/task-P05T01-url-driven-tab-state.md)

### P05T02 — Tiles page filters

Domain filter chips (client-side show/hide) and grade filter (`A`, `B+`, `B`, `C`) with skill count badge per domain group.

[Full detail](tasks/task-P05T02-tiles-page-filters.md)

### P05T03 — Pagefind filter chip

Filter chip in `SearchBar` that toggles between all results and skills-only (`type:skill`).

[Full detail](tasks/task-P05T03-pagefind-filter-chip.md)

### P05T04 — Upgraded skill cards

Add grade badge colour, eval count, audit date, domain tag, and Tessl registry link to skill cards on the tiles page.

[Full detail](tasks/task-P05T04-upgraded-skill-cards.md)
