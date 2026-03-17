# P04T04 — configure-pagefind

## Phase

Phase 04 — Polish and Feature Parity

## Goal

Configure `@pagefind/astro` (or the pagefind post-build script) to index the built site, add a `<PagefindSearch>` UI component to `BaseLayout.astro`, and verify that search returns results for skill titles and descriptions.

## File to create / modify

```
docs/astro.config.mjs              (add pagefind integration or post-build hook)
docs/src/layouts/BaseLayout.astro  (add search UI slot)
docs/src/components/Search.astro   (CREATE — search input wrapper)
```

## Implementation

Install and wire pagefind:

```sh
cd docs
bun add @pagefind/astro
```

Add to `astro.config.mjs`:

```js
import pagefind from "@pagefind/astro";

export default defineConfig({
  // ...
  integrations: [mdx(), pagefind()],
});
```

Create `docs/src/components/Search.astro`:

```astro
---
import { Search as PagefindSearch } from "@pagefind/astro";
---
<PagefindSearch />
```

Add to `BaseLayout.astro` header:

```astro
<Search />
```

Add `data-pagefind-body` to the main content wrapper in `DocsLayout.astro` and `SkillLayout.astro` to scope indexing to content only (exclude nav/sidebar):

```html
<article data-pagefind-body>
  <slot />
</article>
```

## Notes

- `data-pagefind-ignore` should be added to `<aside class="skill-sidebar">` to prevent sidebar metadata from appearing in search results.
- Pagefind runs after `astro build`; it does not index during `bun run dev`. Test search with `bunx astro build && bunx pagefind --site dist`.
- If `@pagefind/astro` is not available, use the alternative: add a post-build script in `package.json` that runs `npx pagefind --site dist`.

## Verification

```sh
cd docs
bunx astro build 2>&1 | tail -5
[ -d dist/pagefind ] && echo "pagefind index generated" || echo "FAIL: no pagefind index"
```
