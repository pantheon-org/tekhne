# P01T09 — Create `src/components/NavToggle.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/components/NavToggle.astro` — a hamburger button that toggles
the `LeftNav` overlay on mobile viewports.

## File to create / modify

```
docs/src/components/NavToggle.astro
```

## Implementation

```astro
---
---

<button
  id="nav-toggle"
  aria-label="Open navigation"
  aria-expanded="false"
  aria-controls="left-nav"
>
  <span aria-hidden="true">&#9776;</span>
</button>

<script>
  const btn = document.getElementById("nav-toggle") as HTMLButtonElement | null;
  const nav = document.getElementById("left-nav");

  if (btn && nav) {
    btn.addEventListener("click", () => {
      const open = nav.getAttribute("data-open") === "true";
      nav.setAttribute("data-open", String(!open));
      btn.setAttribute("aria-expanded", String(!open));
      btn.setAttribute("aria-label", open ? "Open navigation" : "Close navigation");
    });

    // Close on outside click
    document.addEventListener("click", (e) => {
      if (!nav.contains(e.target as Node) && !btn.contains(e.target as Node)) {
        nav.setAttribute("data-open", "false");
        btn.setAttribute("aria-expanded", "false");
        btn.setAttribute("aria-label", "Open navigation");
      }
    });
  }
</script>

<style>
  button {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--tk-text);
    font-size: 1.25rem;
    padding: 0.25rem 0.5rem;
  }

  @media (min-width: 769px) {
    button { display: none; }
  }
</style>
```

## Notes

- Only visible below 769px (mobile breakpoint).
- The `LeftNav` component must add CSS rules to show/hide itself based on `[data-open="true"]` on `#left-nav` — wired in Phase 4 (P04T02) mobile pass.
- Focus trap inside the nav overlay is implemented in Phase 4 (P04T02).

## Verification

```sh
test -f docs/src/components/NavToggle.astro && echo ok
grep "aria-expanded" docs/src/components/NavToggle.astro
grep "aria-controls" docs/src/components/NavToggle.astro
```
