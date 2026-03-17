# P01T08 — Create `src/components/ThemeToggle.astro`

## Phase

Phase 01 — Layout Scaffolding

## Goal

Create `docs/src/components/ThemeToggle.astro` — a sun/moon icon button that
toggles `data-theme` on `<html>` and persists the choice in `localStorage`.

## File to create / modify

```
docs/src/components/ThemeToggle.astro
```

## Implementation

```astro
---
---

<button
  id="theme-toggle"
  aria-label="Toggle theme"
  title="Toggle theme"
>
  <span class="icon-sun" aria-hidden="true">&#9728;</span>
  <span class="icon-moon" aria-hidden="true">&#9790;</span>
</button>

<script>
  const btn = document.getElementById("theme-toggle");
  const html = document.documentElement;

  const apply = (theme: string) => {
    html.setAttribute("data-theme", theme);
    localStorage.setItem("tk-theme", theme);
  };

  if (btn) {
    btn.addEventListener("click", () => {
      apply(html.getAttribute("data-theme") === "dark" ? "light" : "dark");
    });
  }
</script>

<style>
  button {
    background: none;
    border: 1px solid var(--tk-border);
    border-radius: 0.375rem;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
    color: var(--tk-text);
    font-size: 1rem;
    line-height: 1;
  }

  button:hover {
    background: var(--tk-bg-hover);
  }

  [data-theme="dark"] .icon-sun  { display: none; }
  [data-theme="light"] .icon-moon { display: none; }
</style>
```

## Notes

- The initial `data-theme` is set by the inline script in `BaseLayout.astro` before first paint — this component just toggles it.
- Uses Unicode sun/moon glyphs; replace with SVG icons if a design system is added later.

## Verification

```sh
test -f docs/src/components/ThemeToggle.astro && echo ok
grep "localStorage" docs/src/components/ThemeToggle.astro
grep "data-theme" docs/src/components/ThemeToggle.astro
```
