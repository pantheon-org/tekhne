# P02T01 — replace-docs-schema-with-zod

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Replace the Starlight-specific `docsSchema` in `src/content/config.ts` with a plain Zod schema that removes all Starlight-specific fields and reflects the actual frontmatter used across docs and skills pages.

## File to create / modify

```
docs/src/content/config.ts
```

## Implementation

```typescript
import { z, defineCollection } from "astro:content";

const docsCollection = defineCollection({
  type: "content",
  schema: z.object({
    title: z.string(),
    description: z.string().optional(),
    skill: z.string().optional(),
    domain: z.string().optional(),
    grade: z.string().optional(),
    sidebar: z
      .object({
        label: z.string().optional(),
        order: z.number().optional(),
        hidden: z.boolean().optional(),
      })
      .optional(),
    tilePublishedUrl: z.string().url().optional(),
    tileVersion: z.string().optional(),
  }),
});

export const collections = {
  docs: docsCollection,
};
```

## Notes

- `docsSchema` from `@astrojs/starlight/schema` must be removed; it depends on Starlight internals that will not exist after P02T07.
- `tilePublishedUrl` and `tileVersion` are injected at build time by `prelink.mjs` (see P02T05); they are optional here so the schema still validates unprocessed entries.
- TOC field is intentionally omitted — ref sub-pages do not need per-page TOC control after removing Starlight's built-in TOC component.
- `sidebar.hidden` is kept to allow hiding entries from LeftNav without deleting the page.

## Verification

```sh
cd docs
node --input-type=module <<'EOF'
import { collections } from "./src/content/config.ts";
console.assert(collections.docs, "docs collection missing");
console.log("schema ok");
EOF
bunx astro check 2>&1 | grep -v "^$" | tail -5
```
