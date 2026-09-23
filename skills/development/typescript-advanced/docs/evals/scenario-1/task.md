# Scenario: Configure TypeDoc to Generate API Docs from an Existing Source Tree

A small library has a `src/index.ts` entry point re-exporting a few documented functions and classes, but the project has no TypeDoc configuration and no documentation has ever been generated.

```typescript
// src/index.ts
export { retryWithBackoff } from "./retryWithBackoff";
export { Cache } from "./cache";
```

Both `retryWithBackoff` and `Cache` already have JSDoc comments in their own files.

Produce a `typedoc.json` configuration and a short `docs-setup.md` that:

1. Sets the entry point to `src/index.ts` and the output directory to `docs/`.
2. Excludes internal/private members from the generated output (not everything in `src/` should become public documentation, only what `index.ts` re-exports).
3. Configures a readable output format suited to being committed and browsed without a server (state which format was chosen and why).
4. `docs-setup.md` documents the exact command to run TypeDoc and what a contributor should check in the generated output before considering the docs "done" (e.g. that both exports actually appear, that no `@internal`-marked API leaked through).

## Output Specification

Produce two files:

- `typedoc.json` — the TypeDoc configuration
- `docs-setup.md` — the command to run and what to verify afterward
