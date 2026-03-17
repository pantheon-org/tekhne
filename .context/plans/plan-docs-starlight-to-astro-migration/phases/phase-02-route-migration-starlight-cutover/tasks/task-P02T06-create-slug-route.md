# P02T06 — Create dynamic docs slug route

## Goal

Create `src/pages/docs/[...slug].astro` to replace the Starlight catch-all route
and render individual MDX doc pages using `DocsLayout`.

## File

`src/pages/docs/[...slug].astro`

## Context

Starlight's integration registers its own catch-all route for the `docs/`
collection.  After Starlight is removed (P02T07), this file becomes the sole
renderer for individual doc pages.  The slug mapping prototype from P01T10 must
be consulted to ensure all existing URLs are preserved.

## Implementation

```astro
---
import { getCollection, getEntry } from "astro:content";
import DocsLayout from "../../layouts/DocsLayout.astro";

export async function getStaticPaths() {
  const entries = await getCollection("docs");
  return entries.map((entry) => ({
    params: { slug: entry.slug },
    props: { entry },
  }));
}

const { entry } = Astro.props;
const { Content, headings } = await entry.render();
---

<DocsLayout title={entry.data.title} description={entry.data.description} {headings}>
  <Content />
</DocsLayout>
```

Notes:
- `DocsLayout` must accept an optional `headings` prop for the right-side TOC
  (add that prop in P04 if not already present).
- Do not hardcode `/tekhne/` in hrefs — let `base` in `astro.config.mjs` handle
  the prefix.

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
# spot-check a known slug
SLUG=$(bunx astro --version > /dev/null && ls src/content/docs | head -1 | sed 's/\.mdx\?$//')
test -f "dist/tekhne/docs/${SLUG}/index.html" && echo "slug route ok"
```
