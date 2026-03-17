# P02T04 — Create tiles index page

## Goal

Create `src/pages/docs/tiles/index.astro` that lists all skill tiles from the
`tiles` content collection using `DocsLayout`.

## File

`src/pages/docs/tiles/index.astro`

## Context

If a `tiles` collection exists, Starlight previously auto-routed it.  This task
makes the route explicit.  If no `tiles` collection exists yet, create a stub
that renders a placeholder message and can be extended in Phase 5.

## Implementation

```astro
---
import { getCollection } from "astro:content";
import DocsLayout from "../../../layouts/DocsLayout.astro";

let tiles: Awaited<ReturnType<typeof getCollection<"tiles">>> = [];
try {
  tiles = await getCollection("tiles");
} catch {
  // collection may not exist yet
}

const sorted = [...tiles].sort((a, b) =>
  a.data.title.localeCompare(b.data.title),
);
---

<DocsLayout title="Tiles">
  <h1>Skill Tiles</h1>
  {sorted.length === 0 ? (
    <p>No tiles indexed yet.</p>
  ) : (
    <ul>
      {sorted.map((tile) => (
        <li>
          <a href={`/tekhne/docs/tiles/${tile.slug}/`}>{tile.data.title}</a>
          {tile.data.description && <p>{tile.data.description}</p>}
        </li>
      ))}
    </ul>
  )}
</DocsLayout>
```

## Verification

```sh
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
test -f dist/tekhne/docs/tiles/index.html && echo "tiles page generated"
```
