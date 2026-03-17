# Phase 03 — CSS Token Migration

## Goal

Eliminate all `--sl-*` Starlight variable references and `sl-` utility classes from
custom components, replacing them with `--tk-*` tokens from `tokens.css`. After
this phase the codebase has zero Starlight CSS coupling.

## Gate

- [ ] `grep -r "\-\-sl-" docs/src/components` returns no results
- [ ] `grep -r " sl-" docs/src/components` returns no results (class usage)
- [ ] `grep -r "sl-hidden\|lg:sl-block\|md:sl-flex" docs/src` returns no results
- [ ] `docs/src/styles/custom.css` does not exist
- [ ] `cd docs && bun run build` exits 0
- [ ] Manual visual check: dark and light themes render correctly in browser

## Dependencies

- Phase 02 complete: Starlight removed, `--tk-*` tokens defined in `tokens.css`
- `docs/src/styles/tokens.css` contains a complete token set for both themes

## Tasks

### P03T01 — Audit `--sl-*` usages

Enumerate every `--sl-*` reference in `SkillTabs.astro`, `SkillSidebar.astro`, and `SkillPageTitle.astro`. Produce a mapping table: `--sl-X` → `--tk-Y`.

[Full detail](tasks/task-P03T01-audit-sl-variable-usages.md)

### P03T02 — Replace `--sl-*` variables

Apply the mapping table: replace each `--sl-*` reference with the corresponding `--tk-*` token.

[Full detail](tasks/task-P03T02-replace-sl-variables.md)

### P03T03 — Replace `sl-` utility classes

Replace `sl-hidden`, `lg:sl-block`, and similar Starlight utility classes with plain CSS media queries or scoped `<style>` rules.

[Full detail](tasks/task-P03T03-replace-sl-utility-classes.md)

### P03T04 — Delete `custom.css`

Merge any surviving rules from `src/styles/custom.css` into `tokens.css` or relevant component `<style>` blocks, then delete `custom.css`.

[Full detail](tasks/task-P03T04-delete-custom-css.md)

### P03T05 — Verify theme rendering

Manual and build-level verification that both dark and light themes render correctly for all three custom components.

[Full detail](tasks/task-P03T05-verify-theme-rendering.md)
