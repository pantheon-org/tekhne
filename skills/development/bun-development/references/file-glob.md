# Bun Glob Pattern Matching

## Overview
Bun.Glob provides fast file pattern matching with support for globstar (**), wildcards (*), and character classes ([abc]).

## Basic Usage

```typescript
import { Glob } from "bun";

// Create glob pattern
const glob = new Glob("**/*.ts");

// Scan current directory
const files = glob.scan(".");

// Convert async iterator to array
const allFiles = await Array.fromAsync(files);
```

## Glob Patterns

```typescript
// Match all TypeScript files recursively
const tsGlob = new Glob("**/*.ts");

// Match specific directory
const srcGlob = new Glob("src/**/*.{ts,tsx}");

// Match at specific depth
const testGlob = new Glob("**/test/*.test.ts");

// Multiple extensions
const codeGlob = new Glob("**/*.{js,ts,jsx,tsx}");
```

## Scanning with Options

```typescript
const glob = new Glob("**/*.md");

// Scan with dot files
const withDot = glob.scan({
  cwd: ".",
  dot: true,  // Include hidden files
  absolute: false,  // Relative paths
  followSymlinks: false
});

// Absolute paths
const absolutePaths = glob.scan({
  cwd: process.cwd(),
  absolute: true
});
```

## Filtering Results

```typescript
const glob = new Glob("**/*.ts");
const files = glob.scan(".");

for await (const file of files) {
  // Filter programmatically
  if (!file.includes("node_modules")) {
    console.log(file);
  }
}
```

## Performance Patterns

```typescript
// Lazy iteration (memory efficient)
const glob = new Glob("**/*");
for await (const file of glob.scan(".")) {
  await processFile(file);
}

// Collect all (use for small sets)
const allFiles = await Array.fromAsync(glob.scan("."));

// Count matches
let count = 0;
for await (const _ of glob.scan(".")) count++;
```

## Common Use Cases

### Find Test Files
```typescript
const testGlob = new Glob("**/*.test.ts");
const testFiles = await Array.fromAsync(testGlob.scan("src"));
```

### Find Config Files
```typescript
const configGlob = new Glob("**/{.env,.env.*,*.config.{js,ts}}");
const configs = await Array.fromAsync(configGlob.scan("."));
```

### Exclude Patterns
```typescript
const glob = new Glob("**/*.ts");
const files = glob.scan(".");

for await (const file of files) {
  // Manual exclusion
  if (!file.includes("node_modules") && !file.includes("dist")) {
    console.log(file);
  }
}
```

## Best Practices
- Use Bun.Glob instead of fs.readdir for pattern matching
- Prefer lazy iteration with for await for large file sets
- Use absolute paths when passing to Bun.file()
- Exclude node_modules and dist manually if needed
- Use specific patterns to reduce scan scope

## Anti-Patterns
- Don't use node:fs/promises for globbing
- Avoid Array.fromAsync for very large file sets (memory)
- Don't scan from root without specific patterns
- Avoid overly broad patterns like **/*
