# P02T05 — Extend PreLink tile metadata schema

## Goal

Add any fields required by the new bespoke tile detail layout to the `tiles`
collection schema (or `prelink` schema if that is what the collection is called),
so the `SkillLayout` created in Phase 1 can read them without a runtime error.

## File

`src/content/config.ts`

## Context

`SkillLayout` (P01T06) expects fields such as `title`, `description`, `domain`,
`version`, and optionally `tags`.  If the existing tiles schema lacks these
fields, add them now.  Do not remove existing fields — only extend.

## Implementation

1. Read the current `tiles` (or `prelink`) collection definition in
   `src/content/config.ts`.
2. Add missing fields to the zod schema.  Example additions:

```ts
const tiles = defineCollection({
  type: "data",
  schema: z.object({
    title: z.string(),
    description: z.string().optional(),
    domain: z.string().optional(),
    version: z.string().optional(),
    tags: z.array(z.string()).optional(),
    // …keep any existing fields…
  }),
});
```

3. Export the updated collection in `collections`.
4. Update any mock or fixture data files under `src/content/tiles/` to be
   compatible with the new schema (add `title` if missing).

## Verification

```sh
bunx astro check 2>&1 | grep -c "error" | xargs -I{} test {} -eq 0 && echo "ok"
```
