# P02T09 — Clean up old Starlight-generated routes

## Goal

Delete any page files under `src/pages/` that were created as thin wrappers for
Starlight routes and are now superseded by the bespoke pages from P02T03–P02T06.

## Files

Any files matching:
- `src/pages/[...slug].astro` (if it was the Starlight catch-all)
- `src/pages/index.astro` (if it only redirected to a Starlight page)
- Any empty or pass-through wrapper under `src/pages/docs/`

## Context

Starlight typically hijacks the root catch-all or lives inside a custom pages
integration.  Orphaned wrappers after uninstall will either conflict with the new
routes or produce dead 404 pages.

## Implementation

1. List all files under `src/pages/`:

```sh
find src/pages -name "*.astro" | sort
```

2. For each file, determine if it still serves a purpose:
   - If it references `@astrojs/starlight` → delete.
   - If it is an empty wrapper that just renders `<StarlightRoute>` → delete.
   - If it is a legitimate standalone page (e.g. `src/pages/index.astro` with
     real content) → keep.

3. Delete confirmed orphans:

```sh
rm src/pages/orphan-file.astro   # example — replace with actual path
```

4. After deletion, run `astro build` and confirm no 404s for previously working
   URLs.

## Verification

```sh
grep -r "@astrojs/starlight" src/pages/ 2>/dev/null \
  && echo "FAIL: starlight refs remain in pages" && exit 1 \
  || echo "ok"
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
```
