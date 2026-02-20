# TypeScript Compiler Configuration

## tsconfig.json Structure

```json
{
  "compilerOptions": {
    // Type Checking
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true,
    "strictFunctionTypes": true,
    "strictBindCallApply": true,
    "strictPropertyInitialization": true,
    "noImplicitThis": true,
    "alwaysStrict": true,
    
    // Additional Checks
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,
    
    // Module Resolution
    "moduleResolution": "bundler",
    "module": "ESNext",
    "target": "ES2022",
    "lib": ["ES2022"],
    
    // Emit
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "outDir": "./dist",
    "rootDir": "./src",
    
    // Interop Constraints
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "isolatedModules": true,
    "verbatimModuleSyntax": true,
    
    // Language and Environment
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true,
    
    // Projects
    "composite": true,
    "incremental": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

## Essential Compiler Options

### Type Checking Options

**`strict`** - Enable all strict type checking options
- Best practice: Always enable in new projects
- Enables: noImplicitAny, strictNullChecks, strictFunctionTypes, etc.

**`noImplicitAny`** - Error on expressions and declarations with implied `any`
- Prevents accidental `any` usage
- Forces explicit type annotations

**`strictNullChecks`** - `null` and `undefined` require explicit handling
- Prevents null reference errors
- Must check for nullish values before use

**`noUncheckedIndexedAccess`** - Index signatures return `T | undefined`
```typescript
const config: Record<string, string> = {};
const value = config['key']; // string | undefined (not just string)
```

**`exactOptionalPropertyTypes`** - Distinguish between `undefined` and missing properties
```typescript
interface User {
  name?: string;
}
const user1: User = { name: undefined }; // Error with exactOptionalPropertyTypes
const user2: User = {}; // OK
```

### Module Resolution

**`moduleResolution`** - Strategy for resolving module imports
- `"node16"` / `"nodenext"` - For Node.js ESM/CJS hybrid projects
- `"bundler"` - For bundlers (Vite, esbuild, Webpack)

**`module`** - Output module system
- `"ESNext"` - Modern ESM
- `"CommonJS"` - Node.js CJS
- `"NodeNext"` - Node.js with package.json `"type": "module"`

**`baseUrl` and `paths`** - Path mapping
```json
{
  "compilerOptions": {
    "baseUrl": "./src",
    "paths": {
      "@/*": ["./*"],
      "@components/*": ["./components/*"]
    }
  }
}
```

### Emit Options

**`declaration`** - Generate `.d.ts` files
- Required for libraries
- Enables type checking for consumers

**`declarationMap`** - Generate source maps for `.d.ts` files
- Enables "Go to Definition" to source code

**`sourceMap`** - Generate `.js.map` files for debugging

**`outDir`** - Output directory for compiled files

**`rootDir`** - Root directory of source files (for maintaining structure)

### Interop Constraints

**`isolatedModules`** - Ensure each file can be safely transpiled independently
- Required for Babel, esbuild, swc
- Prevents re-export of types without `type` keyword

**`esModuleInterop`** - Enable CommonJS/ESM interop
- Allows `import React from "react"` instead of `import * as React`

**`verbatimModuleSyntax`** - Require `import type` for type-only imports
- More explicit than `isolatedModules`
- Forces clarity about runtime vs type imports

## Project References

For monorepos and large projects:

```json
{
  "compilerOptions": {
    "composite": true,
    "declaration": true,
    "declarationMap": true,
    "incremental": true
  },
  "references": [
    { "path": "../shared" },
    { "path": "../utils" }
  ]
}
```

Benefits:
- Faster builds with caching
- Enforce dependency graph
- Better IDE performance

## Best Practices

1. **Enable strict mode** - Start with `"strict": true`
2. **Use noUncheckedIndexedAccess** - Safer index access
3. **Enable exactOptionalPropertyTypes** - Clearer optional semantics
4. **Use project references** - For monorepos
5. **Match runtime target** - Set `target` and `lib` appropriately
6. **Use verbatimModuleSyntax** - Explicit type imports
7. **Enable incremental** - Faster subsequent builds

## Common Pitfalls

- **Disabling strict mode** - Loses type safety benefits
- **Wrong moduleResolution** - Causes import resolution errors
- **Missing isolatedModules** - Breaks with modern bundlers
- **Overly permissive include** - Slows down compilation
- **Not using project references** - Slow monorepo builds
