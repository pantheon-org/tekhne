# P01T09 — Create `NavToggle.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/components/NavToggle.astro` — a hamburger button that opens and closes the `LeftNav` overlay on mobile viewports.

## File to create / modify

```
docs/src/components/NavToggle.astro
```

## Implementation

```astro
---
// No server-side state needed
---

<button
  id="nav-toggle"
  class="nav-toggle"
  aria-label="Open navigation"
  aria-expanded="false"
  aria-controls="left-nav"
  type="button"
>
  <span class="bar"></span>
  <span class="bar"></span>
  <span class="bar"></span>
</button>

<script>
  const toggle = document.getElementById("nav-toggle");
  const nav = document.getElementById("left-nav");

  if (toggle && nav) {
    toggle.addEventListener("click", () => {
      const open = toggle.getAttribute("aria-expanded") === "true";
      toggle.setAttribute("aria-expanded", String(!open));
      nav.classList.toggle("nav-open", !open);
      toggle.setAttribute("aria-label", !open ? "Close navigation" : "Open navigation");
      if (!open) {
        nav.focus();
      }
    });

    nav.addEventListener("keydown", (e) => {
      if (e.key === "Escape") {
        toggle.setAttribute("aria-expanded", "false");
        nav.classList.remove("nav-open");
        toggle.setAttribute("aria-label", "Open navigation");
        toggle.focus();
      }
    });
  }
</script>

<style>
  .nav-toggle {
    display: none;
    flex-direction: column;
    justify-content: space-between;
    width: 1.5rem;
    height: 1.25rem;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
  }

  .bar {
    display: block;
    height: 2px;
    background: var(--tk-text);
    border-radius: 2px;
  }

  @media (max-width: 768px) {
    .nav-toggle {
      display: flex;
    }
  }
</style>
```

## Notes

- The `aria-controls="left-nav"` attribute links this button to `LeftNav`'s `id="left-nav"`.
- Full focus trap inside the nav overlay is implemented in Phase 4 (P04T02).
- On desktop the button is hidden via `display: none`.

## Verification

```sh
test -f docs/src/components/NavToggle.astro
grep 'aria-expanded' docs/src/components/NavToggle.astro
```
