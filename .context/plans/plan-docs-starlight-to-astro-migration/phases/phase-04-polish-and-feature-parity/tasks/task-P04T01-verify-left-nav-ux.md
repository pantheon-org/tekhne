# P04T01 — verify-left-nav-ux

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Audit `LeftNav.astro` UX against the Starlight sidebar: confirm domain groups collapse/expand with keyboard, active page ancestor is auto-expanded, and long nav trees scroll independently without obscuring page content.

## File to create / modify

```
docs/src/components/LeftNav.astro   (fixes as needed)
docs/src/styles/base.css            (scroll/overflow fixes as needed)
```

## Implementation

Checklist to verify and fix:

1. **Active ancestor expansion** — navigate to a skill page; confirm all parent `<details>` in the nav are `open` without user interaction. Implementation: in `LeftNav.astro`, after rendering, an inline `<script>` should walk up from the active `<a>` and set `open` on parent `<details>` elements.

```js
// LeftNav.astro inline script
const active = document.querySelector('.nav-link[aria-current="page"]');
if (active) {
  let el = active.parentElement;
  while (el) {
    if (el.tagName === "DETAILS") el.open = true;
    el = el.parentElement;
  }
}
```

2. **Keyboard accessibility** — `<details>`/`<summary>` is natively keyboard-accessible (Enter/Space to toggle). Verify with Tab navigation.

3. **Scroll independence** — LeftNav column must have `overflow-y: auto` and `height: 100vh` (or `calc(100vh - header-height)`) so it scrolls independently. Add to `.left-nav` in base.css if missing.

4. **localStorage persistence** — collapsed group state should persist across page loads. Verify the script reads/writes `localStorage["nav-collapsed"]`.

## Notes

- `aria-current="page"` on the active link is set in `LeftNav.astro` by comparing `entry.id` against `Astro.url.pathname`. Verify this comparison strips `BASE_URL` correctly.
- The scroll container must be the nav column itself, not `<body>`, to avoid the page scrolling when moving the mouse into the nav area.

## Verification

```sh
cd docs
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
grep "aria-current" src/components/LeftNav.astro && echo "aria-current present" || echo "FAIL: aria-current missing"
grep "overflow-y" src/styles/base.css && echo "scroll rule present" || echo "WARN: check scroll behaviour manually"
```
