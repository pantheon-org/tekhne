# P03T02 — Replace `--sl-*` vars in custom components

## Phase

Phase 03 — CSS Token Migration

## Goal

Apply the P03T01 mapping table to replace every `--sl-*` CSS variable reference
in `SkillTabs.astro`, `SkillSidebar.astro`, `SkillPageTitle.astro`, and any other
custom component with the corresponding `--tk-*` token.

## File to create / modify

```
src/components/SkillTabs.astro
src/components/SkillSidebar.astro
src/components/SkillPageTitle.astro
src/styles/custom.css          (if referenced here too)
```

## Implementation

For each component, open the `<style>` block and replace:

```css
/* before */
color: var(--sl-color-text);
background: var(--sl-color-bg);
border-color: var(--sl-color-accent);

/* after */
color: var(--tk-color-text);
background: var(--tk-color-bg);
border-color: var(--tk-color-accent);
```

Use the full mapping from `sl-token-map.md`.  If a mapping is marked `⚠ MISSING`,
add the token to `src/styles/tokens.css` in both `:root` and
`[data-theme="dark"]` blocks before replacing the reference.

## Notes

- Do not change any other part of the component — only CSS variable names.
- Run `astro check` after each file to catch any syntax error early.

## Verification

```sh
grep -r -- "--sl-" src/components src/styles --include="*.astro" --include="*.css" \
  && echo "FAIL: --sl- vars remain" && exit 1 \
  || echo "ok"
bunx astro check 2>&1 | grep -c "error" | xargs -I{} test {} -eq 0 && echo "typecheck ok"
```
