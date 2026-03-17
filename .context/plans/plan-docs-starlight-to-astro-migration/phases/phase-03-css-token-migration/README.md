# Phase 03 — CSS Token Migration

## Goal

Replace all `--sl-*` Starlight variable references and `sl-` utility class usages
in custom components with `--tk-*` tokens defined in `tokens.css`, and remove
`src/styles/custom.css` entirely.

## Gate

- [ ] `grep -r "\-\-sl-" docs/src --include="*.astro" --include="*.css"` returns empty
- [ ] `grep -r "sl-hidden\|sl-block\|lg:sl-" docs/src --include="*.astro"` returns empty
- [ ] `bun run build` exits 0
- [ ] `docs/src/styles/custom.css` does not exist

## Dependencies

- Phase 02 complete (Starlight removed, all routes on new layout system)

## Tasks

### P03T01 — Audit `--sl-*` variable usages

List all `--sl-*` references across `SkillTabs.astro`, `SkillSidebar.astro`,
`SkillPageTitle.astro`, and `custom.css`. Produce a mapping table to corresponding
`--tk-*` tokens.

[Full detail](tasks/task-P03T01-audit-sl-variable-usages.md)

### P03T02 — Replace `--sl-*` vars in custom components

Apply the mapping from P03T01. Replace each `--sl-*` reference with the
corresponding `--tk-*` token in all three custom components.

[Full detail](tasks/task-P03T02-replace-sl-vars-in-components.md)

### P03T03 — Replace Starlight utility classes

Replace `sl-hidden`, `lg:sl-block`, and similar Starlight utility classes with
plain CSS media queries in component `<style>` blocks.

[Full detail](tasks/task-P03T03-replace-sl-utility-classes.md)

### P03T04 — Delete `src/styles/custom.css`

Merge any surviving rules into `tokens.css` or relevant component `<style>`
blocks, then delete `custom.css`.

[Full detail](tasks/task-P03T04-delete-custom-css.md)

### P03T05 — Verify dark and light themes

Visually verify all custom components render correctly in both `[data-theme="dark"]`
and `[data-theme="light"]` modes.

[Full detail](tasks/task-P03T05-verify-dark-light-themes.md)
