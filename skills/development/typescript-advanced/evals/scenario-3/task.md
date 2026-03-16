# Scenario 3: Create a Strict-Mode tsconfig.json

## Context

A new Node.js/TypeScript project is being bootstrapped. The project uses ES modules,
targets Node 20, emits output to a `dist/` directory, and includes source files from
`src/`. There is no existing `tsconfig.json`.

## Task

Produce a `tsconfig.json` file for this project that:

1. Enables `strict: true` as the baseline.
2. Sets `target` and `module` appropriately for Node 20 with native ESM support.
3. Enables at least two additional strictness flags beyond the `strict` bundle
   (e.g., `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitReturns`,
   `noFallthroughCasesInSwitch`).
4. Sets `rootDir` to `src` and `outDir` to `dist`.
5. Does NOT set `strict: false` or any option that disables a default strict check.
6. Includes a companion file `tsconfig-notes.md` that explains each non-obvious compiler
   option chosen (one sentence per option is sufficient).

## Output Specification

Produce two files:

- `tsconfig.json` — the compiler configuration
- `tsconfig-notes.md` — explanations of the chosen options
