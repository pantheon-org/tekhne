# P04T07 — Create `src/pages/404.astro`

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Create a custom 404 page using `BaseLayout` so unmatched routes render a branded
error page instead of the host's default.

## File to create / modify

```
src/pages/404.astro   (new)
```

## Implementation

```astro
---
import BaseLayout from "../layouts/BaseLayout.astro";
---

<BaseLayout title="Page Not Found">
  <main class="not-found">
    <h1>404 — Page Not Found</h1>
    <p>The page you requested doesn't exist.</p>
    <a href="/tekhne/docs/">Go to the documentation home</a>
  </main>
</BaseLayout>

<style>
  .not-found {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 60vh;
    gap: var(--tk-space-4);
    text-align: center;
  }
</style>
```

## Notes

- Astro automatically serves `src/pages/404.astro` for unmatched routes in SSG
  mode; no additional config is needed.
- The link points to `/tekhne/docs/` (with the base path prefix).

## Verification

```sh
test -f src/pages/404.astro && echo "404 page exists"
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
test -f dist/tekhne/404.html && echo "404.html generated in dist"
```
