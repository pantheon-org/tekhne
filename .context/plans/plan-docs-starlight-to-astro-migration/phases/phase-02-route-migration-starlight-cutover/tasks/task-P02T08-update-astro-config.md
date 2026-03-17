# P02T08 — Update astro.config.mjs

## Goal

Remove the Starlight integration from `astro.config.mjs` and ensure the config
is correct for the bespoke layout: `base: "/tekhne"`, `@astrojs/mdx` present,
no `starlight({})` call.

## File

`astro.config.mjs`

## Context

After Starlight is uninstalled the config file will have a dead import and
integration call.  This task cleans that up and validates the final config shape.

## Implementation

Before (example):

```js
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import mdx from "@astrojs/mdx";

export default defineConfig({
  base: "/tekhne",
  integrations: [
    starlight({ title: "Tekhne", /* … */ }),
    mdx(),
  ],
});
```

After:

```js
import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";

export default defineConfig({
  base: "/tekhne",
  integrations: [mdx()],
});
```

Rules:
- `base: "/tekhne"` must be preserved exactly.
- `mdx()` must remain (added explicitly in P01T01).
- Remove only the `starlight` import and the `starlight({})` entry in
  `integrations`.  Do not modify any other config keys.

## Verification

```sh
grep "starlight" astro.config.mjs && echo "FAIL: starlight still in config" && exit 1 \
  || echo "ok"
grep 'base.*"/tekhne"' astro.config.mjs && echo "base preserved" || echo "FAIL: base missing"
bunx astro check 2>&1 | grep -c "error" | xargs -I{} test {} -eq 0 && echo "typecheck ok"
```
