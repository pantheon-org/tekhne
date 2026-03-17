# Phase 05 — Registry Enhancements

## Goal

Add interactive enhancements beyond Starlight parity: URL-driven tab state,
tiles-page filters, Pagefind type filter, and upgraded skill cards with rich
metadata display.

## Gate

- [ ] `bun run build` exits 0
- [ ] `?tab=skill|audit|evals|refs` URL param activates the correct tab on page load (manual verification)
- [ ] Domain filter chips on tiles page show/hide skill cards correctly (manual verification)
- [ ] Grade filter chips (`A`, `B+`, `B`, `C`) filter cards correctly (manual verification)
- [ ] Pagefind filter chip toggles between all-results and skills-only (manual verification)

## Dependencies

- Phase 04 complete (full feature parity achieved)

## Tasks

### P05T01 — URL-driven tab state

On tab click, call `history.replaceState` to set `?tab=skill|audit|evals|refs`.
On page load, read `?tab` param, activate matching tab, dispatch `sk-tab-change`
event.

[Full detail](tasks/task-P05T01-url-driven-tab-state.md)

### P05T02 — Tiles page filters

Domain filter chips (client-side show/hide); grade filter (`A`, `B+`, `B`, `C`);
skill count badge per domain group.

[Full detail](tasks/task-P05T02-tiles-page-filters.md)

### P05T03 — Pagefind filter chip

Filter chip in `SearchBar` to toggle between all results and skills-only
(`type:skill`).

[Full detail](tasks/task-P05T03-pagefind-filter-chip.md)

### P05T04 — Upgraded skill cards

Grade badge colour, eval count, audit date, and domain tag on each skill card.

[Full detail](tasks/task-P05T04-upgraded-skill-cards.md)
