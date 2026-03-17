# P02T09 — remove-starlight-config-blocks

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Delete the `buildSidebarNode` and `buildDomainSidebar` helper functions from `docs/astro.config.mjs`, along with any `sidebar:` configuration block passed to the removed `starlight()` call, leaving the config file clean with no dead code.

## File to create / modify

```
docs/astro.config.mjs
```

## Implementation

Read `astro.config.mjs` and identify all code that:

1. Defines `buildSidebarNode` — a helper that constructs Starlight sidebar tree nodes from collection entries
2. Defines `buildDomainSidebar` — a helper that groups entries by domain for the Starlight sidebar
3. Passes a `sidebar:` key to `starlight()`
4. Any `import` statements used exclusively by these functions (e.g. `getCollection` from `astro:content`, `fs` used only for sidebar building)

Delete all identified code. The resulting file should contain only:
- The `defineConfig` import
- The `mdx` import (from P02T08)
- The exported `defineConfig({ ... })` call with `site`, `base`, `integrations`

## Notes

- These helpers exist solely to feed Starlight's sidebar API. LeftNav.astro (P01T07) now handles navigation entirely in the component layer via `getCollection`.
- Verify that no other code in `astro.config.mjs` calls `buildSidebarNode` or `buildDomainSidebar` before deleting.
- If these helpers are defined in a separate module (e.g. `docs/scripts/sidebar.mjs`), delete that file too.

## Verification

```sh
cd docs
grep -E "buildSidebarNode|buildDomainSidebar" astro.config.mjs && echo "FAIL: dead code remains" || echo "ok"
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
```
