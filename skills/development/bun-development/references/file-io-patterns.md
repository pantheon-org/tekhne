# Bun.file I/O Patterns

## Reading Files

### Basic File Reading

```typescript
// Read as text
const file = Bun.file("./data.json");
const text = await file.text();

// Read as JSON
const json = await file.json();

// Read as ArrayBuffer
const buffer = await file.arrayBuffer();

// Read as Uint8Array
const bytes = await file.bytes();
```

### Check File Exists

```typescript
const file = Bun.file("./config.json");
if (await file.exists()) {
  const config = await file.json();
} else {
  console.log("File not found");
}
```

### File Metadata

```typescript
const file = Bun.file("./document.pdf");
console.log(file.size);  // File size in bytes
console.log(file.type);  // MIME type
console.log(file.name);  // File name
```

## Writing Files

### Basic File Writing

```typescript
// Write string
await Bun.write("output.txt", "Hello, Bun!");

// Write JSON
await Bun.write("data.json", { key: "value" });

// Write Blob
const blob = new Blob(["content"], { type: "text/plain" });
await Bun.write("file.txt", blob);

// Write Response
const response = await fetch("https://example.com/data");
await Bun.write("fetched.html", response);

// Copy file
await Bun.write("destination.txt", Bun.file("source.txt"));
```

### Incremental Writing

```typescript
const file = Bun.file("large-file.txt");
const writer = file.writer();

writer.write("First chunk\n");
writer.write("Second chunk\n");
writer.write("Third chunk\n");

await writer.end();
```

## Streaming Large Files

```typescript
const file = Bun.file("large-dataset.json");
const stream = file.stream();

for await (const chunk of stream) {
  // Process chunk
  console.log(chunk);
}
```

## File Operations

### Delete File

```typescript
const file = Bun.file("temporary.txt");
await file.delete();
```

### Use node:fs for Directories

```typescript
import { mkdir, readdir } from "node:fs/promises";

// Create directory
await mkdir("output", { recursive: true });

// Read directory
const files = await readdir("./src");
```

## Repo Patterns (from bun-file-io skill)

- Prefer Bun APIs over Node `fs` for file access
- Check `Bun.file(...).exists()` before reading
- For binary/large files use `arrayBuffer()` and MIME checks via `file.type`
- Use `Bun.Glob` + `Array.fromAsync` for scans
- For large writes, use `Bun.write(Bun.file(path), text)`
- Use `path.join`/`path.resolve` for paths
- Prefer promise `.catch(...)` over `try/catch` when possible
