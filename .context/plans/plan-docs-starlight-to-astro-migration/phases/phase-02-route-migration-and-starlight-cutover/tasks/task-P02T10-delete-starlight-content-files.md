# P02T10 — delete-starlight-content-files

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Delete `docs/src/content/docs/tiles.md` and `docs/src/content/docs/index.mdx` — the Starlight-era content files that are superseded by the new Astro pages created in P02T03 and P02T04.

## File to create / modify

```
docs/src/content/docs/tiles.md       (DELETE)
docs/src/content/docs/index.mdx      (DELETE)
```

## Implementation

```sh
rm docs/src/content/docs/tiles.md
rm docs/src/content/docs/index.mdx
```

## Notes

- `tiles.md` is replaced by `src/pages/tiles.astro` (P02T04); keeping it would produce a duplicate route conflict at `/tekhne/tiles`.
- `index.mdx` is replaced by `src/pages/index.astro` (P02T03); same conflict applies at `/tekhne/`.
- Confirm no other file in `src/` references these paths before deletion. A `grep -r "tiles.md\|index.mdx" docs/src/` check should return no matches.
- If the collection has additional root-level files that are now served by the new pages route, evaluate whether they also need removal.

## Verification

```sh
[ ! -f docs/src/content/docs/tiles.md ] && echo "tiles.md deleted ok" || echo "FAIL: tiles.md still exists"
[ ! -f docs/src/content/docs/index.mdx ] && echo "index.mdx deleted ok" || echo "FAIL: index.mdx still exists"
cd docs && bunx astro build 2>&1 | grep -E "Route conflict|error|Error" | head -10
```
