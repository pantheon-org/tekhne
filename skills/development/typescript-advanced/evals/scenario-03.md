# Scenario 03: Create a Strict-Mode tsconfig.json

## User Prompt

A new Node.js/TypeScript project is being bootstrapped. The project uses ES modules, targets Node 20, emits output to a `dist/` directory, and includes source files from `src/`. There is no existing `tsconfig.json`.

Produce a `tsconfig.json` file for this project that:

1. Enables `strict: true` as the baseline.
2. Sets `target` and `module` appropriately for Node 20 with native ESM support.
3. Enables at least two additional strictness flags beyond the `strict` bundle (e.g., `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitReturns`, `noFallthroughCasesInSwitch`).
4. Sets `rootDir` to `src` and `outDir` to `dist`.
5. Does NOT set `strict: false` or any option that disables a default strict check.
6. Includes a companion file `tsconfig-notes.md` that explains each non-obvious compiler option chosen (one sentence per option is sufficient).

## Output Specification

Produce two files:

- `tsconfig.json` — the compiler configuration
- `tsconfig-notes.md` — explanations of the chosen options

## Expected Behavior

1. Set `"strict": true` in `compilerOptions` without any flag that disables a strict-mode sub-flag (e.g., no `"strictNullChecks": false`)
2. Enable at least two additional flags beyond the `strict` bundle (e.g., `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitReturns`, `noFallthroughCasesInSwitch`)
3. Set `target` to `ES2022` or later and `module` to `NodeNext` or `Node16` (not `CommonJS`) to reflect Node 20 native ESM
4. Set `rootDir` to `"src"` and `outDir` to `"dist"`
5. Provide at least one sentence per non-obvious option in `tsconfig-notes.md` explaining what it does and why it was chosen

## Success Criteria

- **`strict: true` is present**: `compilerOptions` contains `"strict": true` and does not contain any option that directly disables a strict-mode sub-flag
- **At least two extra strictness flags enabled**: The config includes at least two additional flags beyond the `strict` bundle
- **Target and module settings appropriate for Node 20 ESM**: `target` is `ES2022` or later and `module` is `NodeNext` or `Node16` (not `CommonJS`)
- **`rootDir` and `outDir` are correctly set**: `rootDir` is `"src"` and `outDir` is `"dist"`
- **tsconfig-notes.md covers each non-obvious option**: The notes file provides at least one sentence per non-obvious option explaining what it does and why it was chosen

## Failure Conditions

- Agent omits `"strict": true` from `compilerOptions`
- Agent adds `"strictNullChecks": false` or any other flag that disables a strict-mode sub-flag
- Agent enables fewer than two additional strictness flags beyond the `strict` bundle
- Agent sets `module` to `CommonJS` instead of `NodeNext` or `Node16`
- Agent sets `rootDir` or `outDir` incorrectly
- Agent omits `tsconfig-notes.md` or provides no explanations for non-obvious options
