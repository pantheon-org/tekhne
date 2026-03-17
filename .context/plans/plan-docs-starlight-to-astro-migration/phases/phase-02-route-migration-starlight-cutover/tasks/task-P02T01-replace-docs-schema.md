# P02T01 — Replace docs collection schema

## Goal

Remove the Starlight-shaped `docs` content collection schema and replace it with
a minimal custom schema that only captures fields the bespoke layout actually
reads.

## File

`src/content/config.ts`

## Context

The current `config.ts` imports `docsSchema` from `@astrojs/starlight/schema` and
extends it.  After this task the file must have zero imports from any
`@astrojs/starlight` package.

## Implementation

1. Open `src/content/config.ts`.
2. Remove the `@astrojs/starlight/schema` import.
3. Define a replacement schema using `zod` (already a peer dep of Astro):

```ts
import { defineCollection, z } from "astro:content";

const docs = defineCollection({
  type: "content",
  schema: z.object({
    title: z.string(),
    description: z.string().optional(),
    sidebar: z
      .object({
        label: z.string().optional(),
        order: z.number().optional(),
        hidden: z.boolean().optional(),
      })
      .optional(),
  }),
});

export const collections = { docs };
```

4. Keep the collection name `docs` — existing MDX frontmatter must not change.
5. If other collections exist (e.g. `tiles`), preserve them unchanged.

## Verification

```sh
bunx astro check 2>&1 | grep -c "error" | xargs -I{} test {} -eq 0 && echo "ok"
grep -r "@astrojs/starlight" src/content/config.ts && exit 1 || echo "no starlight in schema"
```
