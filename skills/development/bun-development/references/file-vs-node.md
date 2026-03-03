# Bun File APIs vs Node.js fs

## When to Use Bun APIs

### Prefer Bun for:
- Reading file contents (text, JSON, binary)
- Writing files (strings, buffers, Blobs)
- Checking file existence
- File metadata (size, type, name)
- Pattern matching (Bun.Glob)
- Streaming file contents

### Prefer Node.js fs for:
- Directory operations (mkdir, readdir, rmdir)
- Recursive file operations
- File permissions (chmod)
- Symbolic links
- Directory watching
- Complex file system operations

## Reading Files

```typescript
// ✅ Bun (preferred)
const file = Bun.file("config.json");
const data = await file.json();
const text = await file.text();
const buffer = await file.arrayBuffer();

// ❌ Node.js (avoid unless necessary)
import { readFile } from "node:fs/promises";
const buffer = await readFile("config.json");
const text = buffer.toString();
```

## Writing Files

```typescript
// ✅ Bun (preferred)
await Bun.write("output.txt", "Hello, World!");
await Bun.write("data.json", JSON.stringify(obj));
await Bun.write("file.bin", arrayBuffer);

// ❌ Node.js (avoid unless necessary)
import { writeFile } from "node:fs/promises";
await writeFile("output.txt", "Hello, World!");
```

## File Existence

```typescript
// ✅ Bun (preferred)
const file = Bun.file("config.json");
const exists = await file.exists();

// ❌ Node.js (avoid)
import { access } from "node:fs/promises";
try {
  await access("config.json");
  // exists
} catch {
  // doesn't exist
}
```

## Directory Operations

```typescript
// ✅ Node.js (required for directories)
import { mkdir, readdir, rm } from "node:fs/promises";

await mkdir("output", { recursive: true });
const entries = await readdir("src");
await rm("temp", { recursive: true, force: true });

// ❌ Bun (no directory APIs)
// Not available
```

## File Metadata

```typescript
// ✅ Bun (preferred for files)
const file = Bun.file("document.pdf");
console.log(file.size);  // bytes
console.log(file.type);  // MIME type
console.log(file.name);  // filename

// ✅ Node.js (required for detailed stats)
import { stat } from "node:fs/promises";
const stats = await stat("document.pdf");
console.log(stats.mtime);  // modified time
console.log(stats.mode);   // permissions
```

## Pattern Matching

```typescript
// ✅ Bun (preferred)
import { Glob } from "bun";
const glob = new Glob("**/*.ts");
const files = await Array.fromAsync(glob.scan("."));

// ❌ Node.js (manual recursion needed)
import { readdir } from "node:fs/promises";
// Requires manual recursive implementation
```

## Streaming

```typescript
// ✅ Bun (preferred for file streams)
const file = Bun.file("large.log");
for await (const chunk of file.stream()) {
  // Process chunk
}

// ❌ Node.js (more verbose)
import { createReadStream } from "node:fs";
const stream = createReadStream("large.log");
for await (const chunk of stream) {
  // Process chunk
}
```

## Decision Matrix

| Operation | Use Bun API | Use Node.js fs |
|-----------|------------|----------------|
| Read file | ✅ | ❌ |
| Write file | ✅ | ❌ |
| Check exists | ✅ | ❌ |
| File metadata | ✅ | For detailed stats |
| Pattern match | ✅ | ❌ |
| Stream file | ✅ | ❌ |
| Create directory | ❌ | ✅ |
| List directory | ❌ | ✅ |
| Remove directory | ❌ | ✅ |
| File permissions | ❌ | ✅ |
| Symlinks | ❌ | ✅ |
| Watch files | ❌ | ✅ |

## Best Practices
- Default to Bun APIs for file operations
- Use Node.js fs only when Bun doesn't provide the functionality
- Don't mix APIs unnecessarily - be consistent
- Leverage Bun's lazy file loading (no upfront read)
- Use Bun.Glob instead of manual directory traversal

## Anti-Patterns
- Using Node.js fs for simple read/write operations
- Mixing Bun.file() and fs APIs in same module
- Using fs.readdir for pattern matching (use Bun.Glob)
- Ignoring Bun's lazy loading benefits
