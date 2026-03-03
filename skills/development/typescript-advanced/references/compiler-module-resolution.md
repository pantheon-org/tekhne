# Module Resolution in TypeScript

## Module Resolution Strategies

TypeScript uses different strategies to locate module files.

### Node16 / NodeNext

For Node.js projects with ESM support:

```json
{
  "compilerOptions": {
    "module": "NodeNext",
    "moduleResolution": "NodeNext"
  }
}
```

Behavior:
- Respects `package.json` `"type": "module"` or `"type": "commonjs"`
- Requires file extensions in imports: `.js`, `.mjs`, `.cjs`
- Follows Node.js ESM resolution algorithm

```typescript
// Must include .js extension (not .ts)
import { helper } from "./utils.js";
import type { User } from "./types.js";
```

### Bundler

For projects using bundlers (Vite, esbuild, Webpack):

```json
{
  "compilerOptions": {
    "module": "ESNext",
    "moduleResolution": "bundler"
  }
}
```

Behavior:
- No file extensions required
- Supports package.json `"exports"` field
- Allows mixing ESM and CJS
- Optimized for build tools

```typescript
// Extensions optional
import { helper } from "./utils";
import { Component } from "my-library";
```

### Node (Classic)

Legacy Node.js resolution (not recommended for new projects):

```json
{
  "compilerOptions": {
    "moduleResolution": "node"
  }
}
```

## Path Mapping

### baseUrl and paths

Map module names to locations:

```json
{
  "compilerOptions": {
    "baseUrl": "./src",
    "paths": {
      "@/*": ["./*"],
      "@components/*": ["./components/*"],
      "@utils/*": ["./utils/*"],
      "@types": ["./types/index.ts"]
    }
  }
}
```

Usage:
```typescript
import { Button } from "@components/Button";
import { formatDate } from "@utils/date";
import type { User } from "@types";
```

**Important**: Build tools need separate configuration
- Vite: `vite.config.ts` with `resolve.alias`
- Webpack: `webpack.config.js` with `resolve.alias`
- Jest: `jest.config.js` with `moduleNameMapper`

### rootDirs

Treat multiple directories as a single root:

```json
{
  "compilerOptions": {
    "rootDirs": ["src", "generated"]
  }
}
```

Use case: Import from generated code as if it's in source

## Module Types

### ES Modules (ESM)

Modern JavaScript modules:

```typescript
// Export
export const value = 42;
export function helper() {}
export default class MyClass {}

// Import
import MyClass, { value, helper } from "./module";
import * as Module from "./module";
import type { MyType } from "./module";
```

### CommonJS (CJS)

Node.js traditional modules:

```typescript
// Export
exports.value = 42;
module.exports = { value: 42 };

// Import
const { value } = require("./module");
const Module = require("./module");
```

## Type-Only Imports/Exports

Explicitly mark type imports for better tree-shaking:

```typescript
// Type-only import
import type { User, Post } from "./types";
import { type User, fetchUser } from "./api"; // Mixed

// Type-only export
export type { User, Post };
export { type User, fetchUser }; // Mixed
```

Benefits:
- Clearer intent
- Better dead code elimination
- Required with `isolatedModules` and `verbatimModuleSyntax`

## Package.json Exports

Modern packages use `"exports"` field:

```json
{
  "name": "my-library",
  "type": "module",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.js",
      "require": "./dist/index.cjs"
    },
    "./utils": {
      "types": "./dist/utils.d.ts",
      "import": "./dist/utils.js"
    }
  }
}
```

Import paths:
```typescript
import { main } from "my-library";
import { helper } from "my-library/utils";
```

## Declaration Files

### Generating Declarations

```json
{
  "compilerOptions": {
    "declaration": true,
    "declarationMap": true,
    "emitDeclarationOnly": false
  }
}
```

- `declaration`: Generate `.d.ts` files
- `declarationMap`: Generate `.d.ts.map` for "Go to Definition"
- `emitDeclarationOnly`: Only emit declarations (useful with Babel)

### Consuming Declarations

TypeScript looks for types in this order:
1. `package.json` `"types"` or `"typings"` field
2. `index.d.ts` in package root
3. `@types/<package>` package

```json
{
  "name": "my-library",
  "types": "./dist/index.d.ts",
  "main": "./dist/index.js"
}
```

## Monorepo Configuration

### Project References

```json
// packages/shared/tsconfig.json
{
  "compilerOptions": {
    "composite": true,
    "declaration": true,
    "declarationMap": true,
    "outDir": "dist"
  }
}
```

```json
// packages/app/tsconfig.json
{
  "compilerOptions": {
    "composite": true
  },
  "references": [
    { "path": "../shared" }
  ]
}
```

```typescript
// packages/app/src/index.ts
import { helper } from "@my-org/shared";
```

Benefits:
- Incremental builds
- Type checking across packages
- Enforced dependency graph
- Better IDE performance

## Best Practices

1. **Use `bundler` for bundled apps** - Most flexible
2. **Use `NodeNext` for Node.js libraries** - Correct ESM/CJS handling
3. **Always use path mapping** - Cleaner imports
4. **Use type-only imports** - Better tree-shaking
5. **Enable `isolatedModules`** - Ensures transpiler compatibility
6. **Use project references** - For monorepos
7. **Configure build tools** - Match TypeScript paths
8. **Use `"exports"` in package.json** - For libraries

## Common Pitfalls

- **Missing file extensions with NodeNext** - Required for ESM
- **Path mapping not configured in build tool** - Import errors at runtime
- **Wrong moduleResolution** - Causes resolution failures
- **Not using project references** - Slow monorepo builds
- **Mixing ESM and CJS incorrectly** - Runtime errors
- **Using relative paths everywhere** - Hard to refactor
