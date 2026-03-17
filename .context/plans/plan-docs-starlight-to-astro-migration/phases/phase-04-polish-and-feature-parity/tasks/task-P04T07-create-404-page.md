# P04T07 — create-404-page

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Create `docs/src/pages/404.astro` as a custom not-found page that uses `BaseLayout`, shows a helpful message, and provides a link back to the home page and the skill catalogue.

## File to create / modify

```
docs/src/pages/404.astro   (CREATE)
```

## Implementation

```astro
---
import BaseLayout from "../layouts/BaseLayout.astro";
const BASE = import.meta.env.BASE_URL.replace(/\/$/, "");
---
<BaseLayout title="Page Not Found — Tekhne">
  <main class="error-page">
    <h1>404</h1>
    <p>The page you are looking for does not exist or has moved.</p>
    <nav class="error-links">
      <a href={`${BASE}/`}>Go to home</a>
      <a href={`${BASE}/tiles`}>Browse skills</a>
    </nav>
  </main>
</BaseLayout>
```

## Notes

- Astro serves `404.astro` automatically for unmatched routes when deploying to static hosts that support custom 404 pages (GitHub Pages, Netlify, Vercel).
- For GitHub Pages with `base: "/tekhne"`, the 404 page is served at `/tekhne/404.html`. Some redirects from old Starlight URLs may land here; consider adding a client-side redirect script if the URL pattern allows inference of the correct destination.
- Keep the 404 page minimal — no LeftNav (navigation context is lost anyway); just a clear message and two action links.

## Verification

```sh
cd docs
[ -f src/pages/404.astro ] && echo "404.astro exists" || echo "FAIL"
bunx astro build 2>&1 | grep -E "(error|Error)" | head -10
[ -f dist/404.html ] && echo "404.html in build output" || echo "FAIL: 404.html missing from dist"
```
