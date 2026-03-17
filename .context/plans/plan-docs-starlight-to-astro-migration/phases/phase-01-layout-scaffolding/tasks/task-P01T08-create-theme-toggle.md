# P01T08 — Create `ThemeToggle.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/components/ThemeToggle.astro` — a sun/moon icon button that toggles `data-theme` on `<html>` and persists the choice to `localStorage`.

## File to create / modify

```
docs/src/components/ThemeToggle.astro
```

## Implementation

```astro
---
// No server-side props needed; all state is client-side
---

<button
  id="theme-toggle"
  aria-label="Toggle theme"
  class="theme-toggle"
  type="button"
>
  <span class="icon icon-dark" aria-hidden="true">🌙</span>
  <span class="icon icon-light" aria-hidden="true">☀️</span>
</button>

<script>
  const btn = document.getElementById("theme-toggle");
  if (btn) {
    btn.addEventListener("click", () => {
      const html = document.documentElement;
      const current = html.getAttribute("data-theme") ?? "dark";
      const next = current === "dark" ? "light" : "dark";
      html.setAttribute("data-theme", next);
      localStorage.setItem("tk-theme", next);
    });
  }
</script>

<style>
  .theme-toggle {
    background: none;
    border: 1px solid var(--tk-border);
    border-radius: 6px;
    padding: 0.3rem 0.5rem;
    cursor: pointer;
    color: var(--tk-text);
    line-height: 1;
  }

  .theme-toggle:hover {
    background: var(--tk-bg-subtle);
  }

  .icon-light {
    display: none;
  }

  [data-theme="light"] .icon-dark {
    display: none;
  }

  [data-theme="light"] .icon-light {
    display: inline;
  }
</style>
```

## Notes

- The `localStorage` key `"tk-theme"` must match the key read in the `BaseLayout` FOUT-prevention inline script.
- Replace emoji icons with SVG in Phase 4 if desired; the logic is identical.

## Verification

```sh
test -f docs/src/components/ThemeToggle.astro
grep 'tk-theme' docs/src/components/ThemeToggle.astro
```
