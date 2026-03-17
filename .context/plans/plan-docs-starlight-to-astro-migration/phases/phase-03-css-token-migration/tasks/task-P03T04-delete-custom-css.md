# P03T04 — Delete `src/styles/custom.css`

## Phase

Phase 03 — CSS Token Migration

## Goal

Merge any surviving rules from `src/styles/custom.css` into `tokens.css` or
relevant component `<style>` blocks, then delete `custom.css` so it no longer
exists in the project.

## File to create / modify

```
src/styles/custom.css     (deleted)
src/styles/tokens.css     (may receive migrated rules)
src/components/*.astro    (may receive migrated component-specific rules)
```

## Implementation

1. Read the full contents of `custom.css`.
2. Categorise each rule:
   - Global token override → move to `tokens.css` under the appropriate
     `:root` or `[data-theme]` block.
   - Component-specific override → move into that component's `<style>` block.
   - Dead code (overriding Starlight rules now removed) → discard.
3. Remove the `@import "./custom.css"` line from any layout or entry file that
   imports it.
4. Delete `custom.css`:

```sh
rm src/styles/custom.css
```

## Notes

- Verify no other file imports `custom.css` after deletion.

## Verification

```sh
test ! -f src/styles/custom.css && echo "custom.css deleted"
grep -r "custom.css" src/ && echo "FAIL: still imported" && exit 1 || echo "no imports"
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
```
