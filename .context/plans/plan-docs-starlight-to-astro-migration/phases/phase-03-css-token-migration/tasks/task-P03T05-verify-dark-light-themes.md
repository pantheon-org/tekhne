# P03T05 — Verify dark and light themes

## Phase

Phase 03 — CSS Token Migration

## Goal

Confirm that all custom components render correctly in both
`[data-theme="dark"]` and `[data-theme="light"]` modes with no
hardcoded colours, no invisible text, and no layout breakage.

## File to create / modify

Read-only verification — no source files are modified unless a defect is found.
If defects are found, fix them in `tokens.css` or the relevant component.

## Implementation

1. Start the dev server:

```sh
bunx astro dev
```

2. Open a browser at `http://localhost:4321/tekhne/docs/`.

3. Set `data-theme="dark"` via browser console:

```js
document.documentElement.setAttribute("data-theme", "dark")
```

4. Check each custom component:
   - `SkillTabs` — tab borders and active indicator visible
   - `SkillSidebar` — background and text legible
   - `SkillPageTitle` — heading and domain tag legible
   - Left nav — group labels, active link highlight, hover state

5. Repeat with `data-theme="light"`.

6. Check that the theme toggle button switches correctly, that `localStorage`
   retains the choice on reload, and that the initial state respects
   `prefers-color-scheme` when no `localStorage` value exists.

## Notes

- This task has no automated pixel-diff assertion — it is a manual smoke test.
- If a token is missing or misconfigured, fix it in `tokens.css` and re-run.

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok — manual visual check required"
grep -r -- "--sl-" src/ --include="*.astro" --include="*.css" \
  && echo "FAIL: residual --sl- vars found" && exit 1 || echo "no --sl- vars"
test ! -f src/styles/custom.css && echo "custom.css absent"
```
