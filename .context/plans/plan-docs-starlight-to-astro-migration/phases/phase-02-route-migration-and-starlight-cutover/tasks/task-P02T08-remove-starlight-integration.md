# P02T08 — remove-starlight-integration

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Replace the `starlight()` integration call in `docs/astro.config.mjs` with the `mdx()` integration from `@astrojs/mdx`, which was already added in P01T01.

## File to create / modify

```
docs/astro.config.mjs
```

## Implementation

Remove the `starlight` import and its invocation; keep `mdx()` as the only integration:

```js
// BEFORE (remove these lines):
import starlight from "@astrojs/starlight";
// ...
integrations: [
  starlight({ ... }),
],

// AFTER:
import mdx from "@astrojs/mdx";
// ...
integrations: [
  mdx(),
],
```

The full minimal config after the change should look like:

```js
import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";

export default defineConfig({
  site: "https://pantheon-org.github.io",
  base: "/tekhne",
  integrations: [mdx()],
  // pagefind config remains (added in Phase 4)
});
```

## Notes

- `starlight()` implicitly configured the content collection schema, routing, and virtual modules. All of these are now handled by our custom layouts and `config.ts` (P02T01).
- The `mdx()` integration was already installed in P01T01; this task only changes the config to use it.
- If other integrations exist (e.g. `@astrojs/sitemap`), keep them; only remove `starlight`.

## Verification

```sh
cd docs
node -e "const cfg = require('./astro.config.mjs'); console.log('loaded')" 2>&1 | head -5
grep "starlight" astro.config.mjs && echo "FAIL: starlight still in config" || echo "ok"
bunx astro check 2>&1 | grep -E "(error|Error)" | head -10
```
