# P01T01 — Add `@astrojs/mdx` Dependency

## Phase

Phase 01 — Layout Scaffolding

## Goal

Add `@astrojs/mdx` as an explicit dependency in `docs/package.json` so MDX rendering survives Starlight removal in Phase 2.

## File to create / modify

```
docs/package.json
```

## Implementation

```sh
cd docs && bun add @astrojs/mdx
```

This adds `@astrojs/mdx` under `dependencies` in `docs/package.json` and updates `bun.lock`.

## Notes

- Currently `@astrojs/mdx` is only available transitively through `@astrojs/starlight`. When Starlight is removed in Phase 2, MDX rendering will break unless this dependency is explicit.
- Do **not** add `mdx()` to `astro.config.mjs` yet — Starlight's own MDX integration is still active in Phase 1.

## Verification

```sh
cd docs
grep '"@astrojs/mdx"' package.json
bun run build
```
