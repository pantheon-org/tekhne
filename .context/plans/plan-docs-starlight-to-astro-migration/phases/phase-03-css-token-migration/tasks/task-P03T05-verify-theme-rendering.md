# P03T05 — verify-theme-rendering

## Phase

Phase 03 — CSS Token Migration

## Goal

Confirm that light and dark theme rendering works correctly end-to-end: `data-theme="dark"` on `<html>` activates `--tk-*` dark palette tokens, `ThemeToggle` persists the choice in `localStorage`, and no `--sl-*` references remain in computed styles.

## File to create / modify

No file changes — this is a verification-only task.

## Implementation

Manual verification checklist:

1. Run `cd docs && bun run dev`
2. Open `http://localhost:4321/tekhne/` in browser
3. Open DevTools → Application → Local Storage → check `tk-theme` key
4. Toggle theme button — verify `data-theme` attribute changes on `<html>`
5. Reload page — verify theme persists (token is read from localStorage on load)
6. Open DevTools → Elements → Computed Styles on `<body>`
7. Search for any `--sl-` prefixed custom property — expect zero results
8. Verify `--tk-bg` value changes between light/dark modes

Automated check for zero `--sl-*` in source:

```sh
cd docs
grep -rn --include="*.css" --include="*.astro" -E "(--sl-[a-z-]+|var\(--sl-)" src/ \
  && echo "FAIL: --sl- references remain" || echo "ok: zero --sl- references"
```

## Notes

- FOUT (Flash of Unstyled Theme) prevention: the inline `<script>` in `BaseLayout.astro` runs before CSS paint. If users see a light flash on dark-mode reload, check that the script reads `localStorage["tk-theme"]` and applies `data-theme` before `<body>` renders.
- The localStorage key is `tk-theme` (set in both `ThemeToggle.astro` and `BaseLayout.astro` inline script). A mismatch between these two files causes the toggle to not persist.
- Test both `:root` (light) and `[data-theme="dark"]` variable values by inspecting the computed `--tk-bg` in each mode.

## Verification

```sh
cd docs
grep -rn --include="*.css" --include="*.astro" -E "(--sl-[a-z-]+|var\(--sl-)" src/ \
  && echo "FAIL: --sl- references remain" || echo "ok"
bunx astro build 2>&1 | grep -E "(error|Error)" | head -10
```
