# P02T05 — extend-prelink-tile-urls

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Extend `docs/scripts/prelink.mjs` to walk up the file-system from each skills markdown file to the nearest `tile.json`, then inject `tilePublishedUrl` and `tileVersion` into the frontmatter so the build has registry metadata without manual edits.

## File to create / modify

```
docs/scripts/prelink.mjs
```

## Implementation

Add a `findTileJson(filePath)` helper and call it for each skill entry during the frontmatter injection pass:

```js
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

/**
 * Walk up directory tree from `startDir` looking for tile.json.
 * Returns parsed tile object or null if not found before reaching repoRoot.
 */
function findTileJson(startDir, repoRoot) {
  let dir = resolve(startDir);
  while (dir !== repoRoot && dir !== dirname(dir)) {
    const candidate = join(dir, "tile.json");
    if (existsSync(candidate)) {
      try {
        return JSON.parse(readFileSync(candidate, "utf8"));
      } catch {
        return null;
      }
    }
    dir = dirname(dir);
  }
  return null;
}

// In the main processing loop, after reading each .md file:
// const tile = findTileJson(dirname(absoluteFilePath), REPO_ROOT);
// if (tile) {
//   frontmatter.tilePublishedUrl = tile.publishedUrl ?? tile.url ?? undefined;
//   frontmatter.tileVersion = tile.version ?? undefined;
// }
```

Integrate into the existing `prelink.mjs` injection loop. The exact integration point depends on the current script structure — read the file first and add the `findTileJson` call alongside existing frontmatter mutations.

## Notes

- `REPO_ROOT` should be the monorepo root (two levels above `docs/`); use `resolve(import.meta.dirname, "../../")` or derive from the `skills/` directory path.
- `tile.json` fields vary; check both `publishedUrl` and `url` for backward compatibility with older tile formats.
- This injection runs at build time; the resulting frontmatter is baked into the static output so no client-side fetch is needed.
- Only skill files under `skills/` prefix benefit from this; docs-only pages skip the walk silently.

## Verification

```sh
cd docs
node scripts/prelink.mjs --dry-run 2>&1 | grep "tilePublishedUrl" | head -5
# Expect: at least one line showing injected tilePublishedUrl for a published tile
node scripts/prelink.mjs --dry-run 2>&1 | grep -E "error|Error" | head -5
# Expect: no error lines
```
