# P04T02 — verify-mobile-nav

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Confirm that `NavToggle.astro` correctly shows/hides `LeftNav` on mobile viewports (< 768 px) using the `aria-expanded` / `aria-controls` pattern, and that the nav overlay does not block scrolling on the main content area.

## File to create / modify

```
docs/src/components/NavToggle.astro   (fixes as needed)
docs/src/components/LeftNav.astro     (fixes as needed)
docs/src/styles/base.css              (mobile media query fixes as needed)
```

## Implementation

1. **Toggle wiring** — `NavToggle.astro` button should have `aria-controls="left-nav"` and toggle `aria-expanded`. The `<nav id="left-nav">` in `LeftNav.astro` responds via CSS: hidden by default on mobile, visible when `[data-open]` attribute is present.

```astro
<!-- NavToggle.astro button script -->
<script>
const btn = document.getElementById("nav-toggle");
const nav = document.getElementById("left-nav");
btn?.addEventListener("click", () => {
  const open = nav?.getAttribute("data-open") !== null;
  if (open) {
    nav?.removeAttribute("data-open");
    btn.setAttribute("aria-expanded", "false");
  } else {
    nav?.setAttribute("data-open", "");
    btn.setAttribute("aria-expanded", "true");
  }
});
</script>
```

2. **CSS** — in `base.css` under `@media (max-width: 767px)`:

```css
@media (max-width: 767px) {
  #left-nav {
    display: none;
    position: fixed;
    inset: 0;
    z-index: 50;
    overflow-y: auto;
    background: var(--tk-bg);
  }
  #left-nav[data-open] {
    display: block;
  }
}
```

3. **Escape key close** — pressing Escape should close the nav and return focus to the toggle button.

## Notes

- `NavToggle` is only visible on mobile; hide it with `@media (min-width: 768px) { .nav-toggle { display: none; } }` in base.css.
- Fixed positioning on the nav overlay means it does not push page content — no layout shift on open.

## Verification

```sh
cd docs
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
grep "aria-controls" src/components/NavToggle.astro && echo "aria-controls present" || echo "FAIL"
grep "data-open" src/components/LeftNav.astro && echo "data-open pattern present" || echo "FAIL"
```
