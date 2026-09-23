# Scenario: Diagnose a Module Resolution Error

A project has this `tsconfig.json` and fails to build with the error:

```text
error TS2307: Cannot find module './helpers.js' or its corresponding type declarations.
```

The project's actual source file is `src/helpers.ts` (no `.js` file exists — the import uses a `.js` extension per Node's native-ESM convention), and the current config is:

```jsonc
// tsconfig.json
{
  "compilerOptions": {
    "target": "es2022",
    "module": "commonjs",
    "moduleResolution": "node",
    "outDir": "dist",
    "rootDir": "src"
  }
}
```

The import causing the error is:

```typescript
// src/index.ts
import { formatDate } from "./helpers.js";
```

The project ships as native ESM (its `package.json` has `"type": "module"`) and needs Node to resolve emitted `.js` files at runtime.

Produce a corrected `tsconfig.json` and a short explanation file `resolution-notes.md` that:

1. Identify which compiler option combination is actually mismatched (module system vs. resolution strategy vs. the project's runtime module type).
2. Fix the `tsconfig.json` so the `.js`-suffixed relative import resolves correctly against the `.ts` source file, given the project is native ESM.
3. Explain in `resolution-notes.md`, in plain terms, why the original combination failed and why the fix works.

## Output Specification

Produce two files:

- `tsconfig.json` — the corrected compiler configuration
- `resolution-notes.md` — a short explanation of the diagnosis and fix
