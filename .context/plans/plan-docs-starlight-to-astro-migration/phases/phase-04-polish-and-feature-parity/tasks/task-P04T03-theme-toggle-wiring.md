# P04T03 — Theme toggle wiring

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Wire `ThemeToggle.astro` into the `BaseLayout` header so the toggle is visible
on every page, and verify the flash-prevention inline script fires before paint
in both system-dark and system-light modes.

## File to create / modify

```
src/layouts/BaseLayout.astro
src/components/ThemeToggle.astro
```

## Implementation

### 1. Flash-prevention script in BaseLayout

The inline `<script>` in `<head>` (created in P01T04) must run before any CSS
paint.  Confirm the following pattern is present — do not move it or make it
`defer`/`async`:

```html
<script is:inline>
  const stored = localStorage.getItem("theme");
  const system = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  document.documentElement.setAttribute("data-theme", stored ?? system);
</script>
```

### 2. Place ThemeToggle in the header

In the `<header>` slot of `BaseLayout.astro`:

```astro
<header class="site-header">
  <a href="/tekhne/" class="site-logo">Tekhne</a>
  <nav class="header-nav">
    <ThemeToggle />
    <NavToggle />
  </nav>
</header>
```

### 3. ThemeToggle implementation

`ThemeToggle.astro` must:

- Read `data-theme` from `<html>` to render the correct initial icon.
- On click, toggle between `"dark"` and `"light"`, set `data-theme`, and
  persist to `localStorage`.

```astro
---
---
<button id="theme-toggle" aria-label="Toggle theme">
  <span class="icon-sun">☀</span>
  <span class="icon-moon">☽</span>
</button>

<script>
  const btn = document.getElementById("theme-toggle")!;
  btn.addEventListener("click", () => {
    const next = document.documentElement.getAttribute("data-theme") === "dark"
      ? "light" : "dark";
    document.documentElement.setAttribute("data-theme", next);
    localStorage.setItem("theme", next);
  });
</script>
```

## Notes

- System-pref detection only runs once (on initial load) — subsequent page loads
  use `localStorage` if set.

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
grep "data-theme" src/layouts/BaseLayout.astro && echo "flash-prevention present"
grep "ThemeToggle" src/layouts/BaseLayout.astro && echo "ThemeToggle wired"
```
