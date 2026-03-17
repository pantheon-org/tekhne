# P04T02 — Mobile responsiveness

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Ensure the hamburger button opens/closes the `LeftNav` overlay on narrow
viewports, a focus trap is active while the overlay is open, and the right
sidebar collapses below the breakpoint.

## File to create / modify

```
src/components/NavToggle.astro
src/components/LeftNav.astro
src/layouts/DocsLayout.astro
src/layouts/SkillLayout.astro
src/styles/tokens.css          (breakpoint token if not present)
```

## Implementation

1. **Overlay pattern** — `NavToggle.astro` toggles a CSS class (e.g.
   `nav-open`) on `<body>`.  `LeftNav.astro` uses:

```css
@media (max-width: 56rem) {
  .left-nav {
    position: fixed;
    inset: 0;
    transform: translateX(-100%);
    transition: transform 200ms ease;
    z-index: var(--tk-z-overlay);
  }
  body.nav-open .left-nav {
    transform: translateX(0);
  }
}
```

2. **Focus trap** — when the overlay opens, move focus to the first nav link.
   On close (toggle button or `Escape` key), return focus to the toggle button.

3. **Right sidebar** — in `SkillLayout.astro`, add a media query to hide
   `.right-sidebar` below `72rem` (or the breakpoint defined in `tokens.css`).

4. **Backdrop** — add a semi-transparent backdrop element behind the nav overlay
   that closes the nav on click.

## Notes

- Do not rely on `<details>` / `<summary>` for the mobile overlay — the CSS
  transform approach gives better animation control.

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
grep "position: fixed" src/components/LeftNav.astro && echo "fixed overlay present"
grep "focus" src/components/NavToggle.astro && echo "focus management present"
```
