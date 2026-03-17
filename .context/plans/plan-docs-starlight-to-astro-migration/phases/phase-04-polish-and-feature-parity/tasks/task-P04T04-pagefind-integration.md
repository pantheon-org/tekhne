# P04T04 — Pagefind integration

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Add Pagefind metadata attributes to skill and reference page wrappers, and
create a `SearchBar.astro` component that renders the Pagefind UI widget.

## File to create / modify

```
src/layouts/SkillLayout.astro
src/pages/docs/[...slug].astro
src/components/SearchBar.astro   (new)
src/layouts/BaseLayout.astro     (add SearchBar to header)
```

## Implementation

### 1. Add metadata attributes

In `SkillLayout.astro`, wrap the main content area:

```astro
<main data-pagefind-body data-pagefind-meta="type:skill">
  <slot />
</main>
```

In `[...slug].astro` for reference pages (non-skill docs), use:

```astro
<main data-pagefind-body data-pagefind-meta="type:reference">
```

### 2. Create SearchBar.astro

```astro
---
---
<div id="search" class="search-bar"></div>

<link rel="stylesheet" href="/pagefind/pagefind-ui.css" />
<script src="/pagefind/pagefind-ui.js" is:inline></script>
<script is:inline>
  window.addEventListener("DOMContentLoaded", () => {
    new PagefindUI({ element: "#search", showSubResults: true });
  });
</script>
```

### 3. Wire into BaseLayout header

```astro
<header class="site-header">
  <SearchBar />
  <ThemeToggle />
  <NavToggle />
</header>
```

### 4. Run Pagefind after build

Add to `package.json` scripts:

```json
"postbuild": "npx pagefind --source dist"
```

## Notes

- Pagefind UI CSS and JS are generated at build time; they won't exist in dev
  mode. The widget will gracefully not render in dev.
- `base: "/tekhne"` — Pagefind source path must match the output directory.

## Verification

```sh
bunx astro build && npx pagefind --source dist 2>&1 | tail -5
test -f dist/pagefind/pagefind.js && echo "pagefind index generated"
grep "data-pagefind-body" src/layouts/SkillLayout.astro && echo "metadata present"
```
