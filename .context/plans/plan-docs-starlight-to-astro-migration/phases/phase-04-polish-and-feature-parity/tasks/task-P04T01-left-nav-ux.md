# P04T01 — Left nav UX

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Verify and harden `LeftNav.astro` so that `localStorage` group state persists
across navigations, active ancestors auto-expand, and keyboard users can
toggle groups with Enter/Space.

## File to create / modify

```
src/components/LeftNav.astro
```

## Implementation

Open `LeftNav.astro` and review the `<script>` block.  Ensure:

1. **Persistence** — on group toggle, write `lnav:<domain>` key to
   `localStorage` with value `"open"` or `"closed"`.

2. **Restore on load** — on `DOMContentLoaded`, read all `lnav:*` keys and
   set `aria-expanded` accordingly.

3. **Active-ancestor expansion** — after restoring state, find the
   `[aria-current="page"]` link, walk up to its parent `<details>` or
   group button, and force `aria-expanded="true"` / `open` regardless of
   stored value.

4. **Keyboard support** — group toggle buttons must respond to `keydown`
   with `key === "Enter"` or `key === " "`.

Example skeleton for the script block:

```js
const KEY = (domain) => `lnav:${domain}`;

document.addEventListener("DOMContentLoaded", () => {
  // restore stored state
  document.querySelectorAll("[data-nav-group]").forEach((btn) => {
    const domain = btn.dataset.navGroup;
    const stored = localStorage.getItem(KEY(domain));
    if (stored) btn.setAttribute("aria-expanded", stored === "open" ? "true" : "false");
  });

  // force-expand active ancestor
  const active = document.querySelector("[aria-current='page']");
  if (active) {
    active.closest("[data-nav-group-content]")
      ?.previousElementSibling
      ?.setAttribute("aria-expanded", "true");
  }
});

document.querySelectorAll("[data-nav-group]").forEach((btn) => {
  const handler = () => {
    const next = btn.getAttribute("aria-expanded") !== "true";
    btn.setAttribute("aria-expanded", String(next));
    localStorage.setItem(KEY(btn.dataset.navGroup), next ? "open" : "closed");
  };
  btn.addEventListener("click", handler);
  btn.addEventListener("keydown", (e) => {
    if (e.key === "Enter" || e.key === " ") { e.preventDefault(); handler(); }
  });
});
```

## Notes

- `data-nav-group` attribute must be set to the domain slug on each group button.
- `data-nav-group-content` must be set on the collapsible content wrapper.

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
grep "localStorage" src/components/LeftNav.astro && echo "localStorage present"
grep "aria-expanded" src/components/LeftNav.astro && echo "aria-expanded used"
```
