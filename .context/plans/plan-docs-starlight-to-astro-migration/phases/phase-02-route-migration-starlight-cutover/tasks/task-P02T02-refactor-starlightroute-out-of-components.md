# P02T02 — Remove StarlightRoute from shared components

## Goal

Locate every component or layout file that renders `<StarlightRoute>` (or imports
from `@astrojs/starlight`) and replace those references with the new bespoke
layouts created in Phase 1.

## Files

All files under `src/components/` and `src/layouts/` that import from
`@astrojs/starlight`.

## Context

Starlight injects its own routing via `<StarlightRoute>`.  Any wrapper component
that delegates to it must be replaced before Starlight is uninstalled in P02T07.

## Implementation

1. Find every file importing from Starlight:

```sh
grep -rl "@astrojs/starlight" src/components src/layouts 2>/dev/null
```

2. For each file found:
   - If the file is a layout wrapper around `<StarlightRoute>`, replace it with
     the equivalent bespoke layout (`DocsLayout` or `SkillLayout` from Phase 1).
   - If the file only re-exports Starlight CSS variables, delete it (those tokens
     will be replaced in Phase 3).
   - If the file uses Starlight utility types, replace with local types or inline.

3. Delete files that existed solely to wrap Starlight and have no post-migration
   purpose.

4. Ensure no remaining file under `src/components/` or `src/layouts/` imports
   from `@astrojs/starlight`.

## Verification

```sh
grep -r "@astrojs/starlight" src/components src/layouts 2>/dev/null \
  && echo "FAIL: starlight imports remain" && exit 1 \
  || echo "ok"
bunx astro check 2>&1 | grep -c "error" | xargs -I{} test {} -eq 0 && echo "typecheck ok"
```
