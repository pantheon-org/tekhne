# P01T01 — Add `@astrojs/mdx` dependency

## Phase

Phase 01 — Layout Scaffolding

## Goal

Add `@astrojs/mdx` as an explicit dependency in `docs/package.json` so it survives
Starlight removal in Phase 2.

## File to create / modify

```
docs/package.json
```

## Implementation

```sh
cd docs && bun add @astrojs/mdx
```

The `bun add` command updates both `package.json` and `bun.lockb`. No manual
edits to `package.json` are required.

## Notes

- `@astrojs/mdx` is currently only available transitively through `@astrojs/starlight`.
- Without this explicit dependency, `import { mdx } from "@astrojs/mdx"` in `astro.config.mjs` will fail after Phase 2 removes Starlight.

## Verification

```sh
cd docs && grep '"@astrojs/mdx"' package.json
```
