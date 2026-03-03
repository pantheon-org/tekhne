---
category: runtime
priority: CRITICAL
source: bun-runtime
---

# Bun Global APIs

Bun provides globally available APIs optimized for performance that are significantly faster than Node.js equivalents. These APIs are available without imports.

## Core Globals

### Bun.file()

Fast file reading with automatic content-type detection and streaming support.

```typescript
// Basic file reading
const file = Bun.file("./data.json");
const text = await file.text();       // Read as string
const json = await file.json();       // Parse JSON automatically
const buffer = await file.arrayBuffer(); // Read as ArrayBuffer
const blob = await file.blob();       // Read as Blob

// File metadata
console.log(file.name);    // Filename
console.log(file.size);    // Size in bytes
console.log(file.type);    // MIME type (auto-detected)

// Check existence
if (await file.exists()) {
  const content = await file.text();
}

// Streaming large files
const largeFile = Bun.file("./large-video.mp4");
const stream = largeFile.stream();

for await (const chunk of stream) {
  // Process chunk by chunk
  console.log(chunk);
}
```

**Performance:** 10x faster than `fs.readFile` for JSON parsing.

### Bun.write()

High-performance file writing with automatic serialization.

```typescript
// Write string
await Bun.write("output.txt", "Hello, Bun!");

// Write JSON (auto-stringify)
await Bun.write("data.json", { key: "value", count: 42 });

// Write Blob
const blob = new Blob(["content"], { type: "text/plain" });
await Bun.write("output.txt", blob);

// Write Response
const response = await fetch("https://example.com/data.json");
await Bun.write("downloaded.json", response);

// Write to specific path
await Bun.write(Bun.file("./output/result.txt"), "data");

// Append to file (not supported directly, use streams)
```

**Performance:** 3x faster than `fs.writeFile`.

### Bun.serve()

Ultra-fast HTTP server with built-in WebSocket support.

```typescript
const server = Bun.serve({
  port: 3000,
  hostname: "localhost",
  
  fetch(req, server) {
    const url = new URL(req.url);

    // Handle routes
    if (url.pathname === "/") {
      return new Response("Hello, World!");
    }

    if (url.pathname === "/api/data") {
      return Response.json({
        message: "Fast JSON response",
        timestamp: Date.now()
      });
    }

    // Serve static files
    if (url.pathname.startsWith("/static/")) {
      const filePath = `.${url.pathname}`;
      return new Response(Bun.file(filePath));
    }

    // WebSocket upgrade
    if (url.pathname === "/ws") {
      if (server.upgrade(req)) {
        return; // Upgraded to WebSocket
      }
      return new Response("WebSocket upgrade failed", { status: 400 });
    }

    return new Response("Not Found", { status: 404 });
  },

  // WebSocket handlers
  websocket: {
    message(ws, message) {
      console.log("Received:", message);
      ws.send(`Echo: ${message}`);
    },
    
    open(ws) {
      console.log("Client connected");
      ws.subscribe("chat-room");
    },
    
    close(ws, code, reason) {
      console.log("Client disconnected", code, reason);
    },
  },

  // Server lifecycle
  error(error) {
    return new Response("Internal Server Error", { status: 500 });
  },
});

console.log(`Server running on http://localhost:${server.port}`);

// Graceful shutdown
process.on("SIGINT", () => {
  server.stop();
  process.exit(0);
});
```

**Performance:** 4x faster than Node.js HTTP server, handles 100k+ req/s.

### Bun.env

Type-safe environment variable access.

```typescript
// Access environment variables
const apiKey = Bun.env.API_KEY;
const port = Bun.env.PORT ?? "3000";
const nodeEnv = Bun.env.NODE_ENV ?? "development";

// All environment variables
console.log(Bun.env);

// Type-safe with TypeScript
declare module "bun" {
  interface Env {
    API_KEY: string;
    DATABASE_URL: string;
    PORT?: string;
  }
}

// Now TypeScript knows these exist
const dbUrl: string = Bun.env.DATABASE_URL; // Type-safe!
```

**Prefer Bun.env over process.env** for consistency.

### Bun.$

Safe shell command execution with template literals.

```typescript
import { $ } from "bun";

// Execute shell commands
const output = await $`ls -la`.text();
console.log(output);

// Get current git branch
const branch = await $`git branch --show-current`.text();
console.log(`Current branch: ${branch.trim()}`);

// With error handling
try {
  await $`npm run build`;
  console.log("Build successful");
} catch (error) {
  console.error("Build failed:", error);
}

// Pipe commands
const result = await $`cat package.json | grep "version"`.text();

// Capture exit code
const proc = await $`exit 42`;
console.log(proc.exitCode); // 42

// Escape special characters automatically
const filename = "file with spaces.txt";
await $`cat ${filename}`; // Safely escaped
```

**Security:** Automatically escapes variables to prevent injection.

### Bun.password

Built-in password hashing with bcrypt/argon2.

```typescript
// Hash password
const password = "super-secret-password";
const hash = await Bun.password.hash(password);
console.log(hash); // $argon2id$...

// Verify password
const isValid = await Bun.password.verify(password, hash);
console.log(isValid); // true

// Custom algorithm (default is argon2id)
const bcryptHash = await Bun.password.hash(password, {
  algorithm: "bcrypt",
  cost: 12,
});

// Verify works with any algorithm
const isBcryptValid = await Bun.password.verify(password, bcryptHash);
```

**Security:** No external dependencies needed (no `bcrypt` npm package).

## File System Operations

### Glob Pattern Matching

```typescript
import { Glob } from "bun";

// Find all TypeScript files
const glob = new Glob("**/*.ts");

// Scan directory
for await (const file of glob.scan(".")) {
  console.log(file);
}

// Get all matches as array
const tsFiles = await Array.fromAsync(glob.scan("."));
console.log(`Found ${tsFiles.length} TypeScript files`);

// Multiple patterns
const sourceFiles = new Glob("src/**/*.{ts,tsx}");
```

### File Watching

```typescript
import { watch } from "fs";

// Watch file or directory
const watcher = watch("./src", { recursive: true }, (event, filename) => {
  console.log(`${event}: ${filename}`);
});

// Stop watching
watcher.close();
```

## HTTP Client

Use native `fetch` (Web API standard):

```typescript
// GET request
const response = await fetch("https://api.example.com/data");
const data = await response.json();

// POST request with JSON
const postResponse = await fetch("https://api.example.com/users", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    name: "Alice",
    email: "alice@example.com",
  }),
});

// Handle errors
if (!postResponse.ok) {
  throw new Error(`HTTP error! status: ${postResponse.status}`);
}

// Download file
const fileResponse = await fetch("https://example.com/file.pdf");
await Bun.write("downloaded.pdf", fileResponse);
```

## Performance Comparisons

| Operation | Node.js | Bun | Speedup |
|-----------|---------|-----|---------|
| Read JSON file | 100ms | 10ms | **10x** |
| Write file | 50ms | 17ms | **3x** |
| HTTP req/s | 25k | 100k+ | **4x** |
| Password hash | 300ms | 50ms | **6x** |
| SQLite query | 50ms | 5ms | **10x** |

## Best Practices

### Use Bun.file() for All File Operations

```typescript
// GOOD - Fast Bun API
const data = await Bun.file("data.json").json();
await Bun.write("output.json", { result: data });

// AVOID - Slower Node.js APIs
import fs from "fs/promises";
const text = await fs.readFile("data.json", "utf-8");
const data = JSON.parse(text);
await fs.writeFile("output.json", JSON.stringify({ result: data }));
```

### Handle File Existence

```typescript
// GOOD - Check before reading
const file = Bun.file("config.json");
if (await file.exists()) {
  const config = await file.json();
} else {
  console.error("Config file not found");
}

// AVOID - Catching errors
try {
  const config = await Bun.file("config.json").json();
} catch {
  // File doesn't exist or invalid JSON?
}
```

### Structured Error Handling

```typescript
// GOOD - Specific error handling
try {
  const file = Bun.file("data.json");
  
  if (!(await file.exists())) {
    throw new Error("File not found");
  }
  
  const data = await file.json();
  
  if (!data.version) {
    throw new Error("Invalid data format");
  }
  
  return data;
} catch (error) {
  if (error instanceof SyntaxError) {
    console.error("Invalid JSON");
  } else if (error instanceof Error) {
    console.error(error.message);
  }
}
```

### Stream Large Files

```typescript
// GOOD - Stream large files
const largeFile = Bun.file("video.mp4");
const stream = largeFile.stream();

const response = new Response(stream, {
  headers: {
    "Content-Type": "video/mp4",
    "Content-Length": largeFile.size.toString(),
  },
});

// AVOID - Loading entire file in memory
const largeFile = Bun.file("video.mp4");
const buffer = await largeFile.arrayBuffer(); // OOM risk!
```

## Common Pitfalls

### Async/Await Required

```typescript
// WRONG - Missing await
const text = Bun.file("data.txt").text(); // Returns Promise!
console.log(text); // Promise { <pending> }

// CORRECT
const text = await Bun.file("data.txt").text();
console.log(text); // Actual content
```

### File Path Resolution

```typescript
// WRONG - Relative to current directory (may change)
const file = Bun.file("data.json");

// CORRECT - Relative to script location
import { fileURLToPath } from "url";
import { dirname, join } from "path";

const __dirname = dirname(fileURLToPath(import.meta.url));
const file = Bun.file(join(__dirname, "data.json"));
```

### Response Body Can Only Be Read Once

```typescript
// WRONG - Body already consumed
const response = await fetch("https://example.com/api");
const json1 = await response.json();
const json2 = await response.json(); // ERROR!

// CORRECT - Clone response if needed twice
const response = await fetch("https://example.com/api");
const clone = response.clone();
const json1 = await response.json();
const json2 = await clone.json();
```

## Related References

- @see file-io-patterns.md - File I/O best practices
- @see runtime-http-server.md - Bun.serve() details
- @see runtime-shell.md - Bun.$ command execution
- @see security-shell-escaping.md - Preventing injection attacks
