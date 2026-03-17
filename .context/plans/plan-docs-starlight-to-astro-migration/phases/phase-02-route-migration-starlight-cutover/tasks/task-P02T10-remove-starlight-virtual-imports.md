# P02T10 — Remove Starlight virtual module imports

## Goal

Locate and remove all remaining `import ... from "virtual:starlight/..."` and
`import ... from "astro:content"` Starlight-specific re-exports that are now
dead after uninstall.

## Files

Any `.astro`, `.ts`, or `.js` file under `src/` that still imports from:
- `virtual:starlight/*`
- `@astrojs/starlight/components`
- `@astrojs/starlight/props`
- `@astrojs/starlight/utils`

## Context

Starlight exposes virtual modules (e.g. `virtual:starlight/components`,
`virtual:starlight/user-config`).  Any file that imports these will cause a Vite
resolve error once Starlight is uninstalled.  This is the final sweep before the
phase gate check.

## Implementation

1. Search for remaining virtual imports:

```sh
grep -r "virtual:starlight\|@astrojs/starlight" src/ --include="*.astro" \
  --include="*.ts" --include="*.tsx" --include="*.js"
```

2. For each hit:
   - If the import provided a component → replace with the bespoke equivalent
     from Phase 1 or remove if unused.
   - If the import provided a type → replace with a local type definition.
   - If the import was only used in a file already deleted in P02T09 → no action
     needed (file is gone).

3. Ensure every touched file still compiles:

```sh
bunx astro check
```

## Verification

```sh
grep -r "virtual:starlight\|@astrojs/starlight" src/ 2>/dev/null \
  && echo "FAIL: virtual imports remain" && exit 1 \
  || echo "all starlight imports removed"
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok"
```
