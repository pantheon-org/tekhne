# P01T10 — Prototype Slug Mapping

## Phase

Phase 01 — Layout Scaffolding

## Goal

Produce a dry-run script that reads all `docs` collection entry IDs and prints the computed `[...slug]` route values, confirming no collisions or missing pages across all 537 entries before the Phase 2 cutover.

## File to create / modify

```
docs/scripts/check-slugs.mjs
```

## Implementation

```js
#!/usr/bin/env node
// Dry-run: prints computed slug for every docs collection entry.
// Run before Phase 2 cutover to confirm no collisions.
import { readdir, readFile } from "node:fs/promises";
import { join, relative } from "node:path";

const SKILLS_SRC = new URL("../../skills", import.meta.url).pathname;
const CONTENT_OUT = new URL("../src/content/docs/skills", import.meta.url).pathname;

async function walk(dir) {
  const entries = await readdir(dir, { withFileTypes: true });
  const files = [];
  for (const e of entries) {
    const full = join(dir, e.name);
    if (e.isDirectory()) files.push(...(await walk(full)));
    else files.push(full);
  }
  return files;
}

const all = await walk(CONTENT_OUT);
const mdFiles = all.filter((f) => f.endsWith(".md") || f.endsWith(".mdx"));

const slugs = mdFiles.map((f) => {
  // Collection ID: path relative to content/docs, no extension
  const id = relative(CONTENT_OUT, f).replace(/\.(md|mdx)$/, "");
  // Route slug: strip leading "skills/" prefix used by getStaticPaths
  const routeSlug = id.replace(/^skills\//, "");
  return { id, routeSlug };
});

// Detect duplicates
const seen = new Map();
let hasDupe = false;
for (const { id, routeSlug } of slugs) {
  if (seen.has(routeSlug)) {
    console.error(`COLLISION: ${routeSlug} — from "${id}" and "${seen.get(routeSlug)}"`);
    hasDupe = true;
  }
  seen.set(routeSlug, id);
}

console.log(`Total entries: ${slugs.length}`);
if (hasDupe) {
  process.exit(1);
} else {
  console.log("No collisions. Safe to proceed with Phase 2 cutover.");
}
```

## Notes

- Run `bun run prelink` first so `docs/src/content/docs/skills/` is populated before this script executes.
- The script must exit 0 before starting Phase 2; a non-zero exit means slug collisions exist and must be resolved first.
- Collection IDs produced by Astro's glob loader have **no file extension** (e.g. `skills/infrastructure/terraform/generator/skill`) — strip only the leading `skills/` prefix for the route.

## Verification

```sh
cd docs
node scripts/check-slugs.mjs
echo "Exit code: $?"
```
