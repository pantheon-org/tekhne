# Scenario: Split a God-Module into Single-Function Modules with a Barrel Export

A `stringUtils.ts` file has grown into a grab-bag of unrelated string helpers, all exported from one file, making it unclear which helpers are actually related and used where.

```typescript
// stringUtils.ts

export function slugify(input: string): string {
  return input.toLowerCase().trim().replace(/[^a-z0-9]+/g, "-").replace(/(^-|-$)/g, "");
}

export function truncate(input: string, maxLength: number): string {
  return input.length > maxLength ? input.slice(0, maxLength) + "…" : input;
}

export function capitalize(input: string): string {
  return input.charAt(0).toUpperCase() + input.slice(1);
}
```

Reorganize this into a `string-utils/` directory that:

1. Puts each function in its own file named after the function (`slugify.ts`, `truncate.ts`, `capitalize.ts`), each with a single named export and no other unrelated exports.
2. Adds an `index.ts` barrel file that re-exports all three functions, so existing callers can still `import { slugify, truncate, capitalize } from "./string-utils"` unchanged.
3. Keeps each function's implementation and behavior identical to the original.
4. Does not introduce circular imports between the three files.

## Output Specification

Produce four files as a single combined listing (with clear `// filename` headers): `string-utils/slugify.ts`, `string-utils/truncate.ts`, `string-utils/capitalize.ts`, and `string-utils/index.ts`.
