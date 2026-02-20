# TypeScript Compiler Performance

## Performance Optimization Strategies

### Incremental Compilation

Enable caching for faster rebuilds:

```json
{
  "compilerOptions": {
    "incremental": true,
    "tsBuildInfoFile": ".tsbuildinfo"
  }
}
```

Benefits:
- Only recompile changed files
- 2-5x faster on subsequent builds
- Minimal disk usage (~few KB)

### Project References

For monorepos and large projects:

```json
{
  "compilerOptions": {
    "composite": true,
    "declaration": true
  },
  "references": [
    { "path": "../shared" }
  ]
}
```

Benefits:
- Parallel compilation
- Incremental builds across projects
- Better IDE performance
- Enforced dependency boundaries

Build with:
```bash
tsc --build
tsc --build --watch # Watch mode
tsc --build --force # Clean rebuild
```

### Skip Type Checking for Libraries

Skip checking `node_modules`:

```json
{
  "compilerOptions": {
    "skipLibCheck": true
  }
}
```

Trade-offs:
- Faster compilation (~30-50%)
- May miss errors in type definitions
- Safe if dependencies are well-typed

### Reduce Checked Files

```json
{
  "include": ["src/**/*"],
  "exclude": [
    "node_modules",
    "dist",
    "**/*.test.ts",
    "**/*.spec.ts"
  ]
}
```

Alternative: Separate test configuration
```json
// tsconfig.build.json (for production)
{
  "extends": "./tsconfig.json",
  "exclude": ["**/*.test.ts", "**/*.spec.ts"]
}
```

### Limit Type Instantiation Depth

Prevent runaway type computation:

```json
{
  "compilerOptions": {
    "noErrorTruncation": false, // Truncate long error messages
    "maxNodeModuleJsDepth": 0   // Don't infer types from .js files
  }
}
```

## Type System Performance

### Avoid Deep Nesting

```typescript
// Slow: Deep recursion
type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object
    ? DeepPartial<T[P]>
    : T[P];
};

// Limit recursion depth
type DeepPartial<T, Depth extends number = 5> = Depth extends 0
  ? T
  : {
      [P in keyof T]?: T[P] extends object
        ? DeepPartial<T[P], Prev[Depth]>
        : T[P];
    };

type Prev = [never, 0, 1, 2, 3, 4, 5];
```

### Simplify Union Types

```typescript
// Slow: Large union (100s of members)
type Color = "red" | "blue" | "green" | ... // 100+ values

// Fast: Use string index
type Color = string;
const COLORS = {
  red: "#ff0000",
  blue: "#0000ff",
  // ...
} as const;
type Color = keyof typeof COLORS;
```

### Cache Complex Types

```typescript
// Slow: Recalculate every time
function process<T>(value: ComplexType<ComplexType<T>>) {}

// Fast: Cache intermediate type
type CachedComplex<T> = ComplexType<ComplexType<T>>;
function process<T>(value: CachedComplex<T>) {}
```

### Use Type Aliases for Reuse

```typescript
// Recalculates type each time
function a(value: { id: string; name: string; age: number }) {}
function b(value: { id: string; name: string; age: number }) {}

// Fast: Reuse type
type User = { id: string; name: string; age: number };
function a(value: User) {}
function b(value: User) {}
```

## Build Tool Integration

### Parallel Type Checking

Use `fork-ts-checker-webpack-plugin` or similar:

```javascript
// webpack.config.js
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');

module.exports = {
  plugins: [
    new ForkTsCheckerWebpackPlugin({
      async: true, // Non-blocking
      typescript: {
        configFile: 'tsconfig.json'
      }
    })
  ]
};
```

### Use swc or esbuild for Transpilation

Only use TypeScript for type checking:

```bash
# Type check
tsc --noEmit

# Transpile (fast)
esbuild src/index.ts --bundle --outfile=dist/index.js
# or
swc src -d dist
```

Benefits:
- 10-100x faster transpilation
- TypeScript only for types
- Best of both worlds

### Watch Mode Optimization

```json
{
  "compilerOptions": {
    "assumeChangesOnlyAffectDirectDependencies": true
  },
  "watchOptions": {
    "excludeDirectories": ["**/node_modules", "dist"],
    "excludeFiles": ["**/*.test.ts"]
  }
}
```

## IDE Performance

### Reduce IntelliSense Scope

```json
{
  "exclude": [
    "node_modules",
    "dist",
    "build",
    "coverage",
    ".cache"
  ]
}
```

### Disable Automatic Type Acquisition

```json
{
  "compilerOptions": {
    "types": [] // Only include explicit @types packages
  }
}
```

### Use `@ts-nocheck` for Problematic Files

```typescript
// @ts-nocheck
// Large generated file or vendor code
```

## Profiling and Diagnostics

### Trace Type Checking

```bash
tsc --generateTrace trace
# Analyze with: https://ui.perfetto.dev/
```

### Measure Compilation Time

```bash
tsc --diagnostics
# or more detail
tsc --extendedDiagnostics
```

Output includes:
- Files checked
- Time per phase
- Memory usage
- Cache hits

### Find Slow Types

```bash
tsc --generateTrace trace --extendedDiagnostics
```

Look for:
- High instantiation counts
- Deep recursion
- Large union types

## Best Practices

1. **Enable incremental compilation** - 2-5x faster rebuilds
2. **Use project references** - For monorepos
3. **Enable skipLibCheck** - Safe for most projects
4. **Separate type checking from transpilation** - Use esbuild/swc
5. **Limit recursion depth** - Prevent runaway types
6. **Cache complex types** - Reuse type aliases
7. **Exclude tests from production builds** - Faster builds
8. **Use watch mode options** - Fewer unnecessary checks
9. **Profile slow compilations** - Use --generateTrace
10. **Keep dependencies updated** - Better typed libraries

## Performance Budget

Compilation times to aim for:
- Small project (<100 files): <2s
- Medium project (<500 files): <10s
- Large project (<2000 files): <30s
- Monorepo: <60s with project references

If slower:
1. Enable incremental compilation
2. Add project references
3. Profile with --generateTrace
4. Simplify complex types
5. Reduce checked files

## Common Bottlenecks

- **No incremental compilation** - Enable `incremental: true`
- **Checking all node_modules** - Enable `skipLibCheck: true`
- **No project references** - Add for monorepos
- **Deep type recursion** - Limit recursion depth
- **Large unions** - Use string index signatures
- **Checking test files in production** - Separate config
- **Overly complex mapped types** - Simplify or cache
