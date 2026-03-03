# HTTP Server

Bun's HTTP server (Bun.serve) is optimized for speed with built-in WebSocket support.

## Basic HTTP Server

```typescript
const server = Bun.serve({
  port: 3000,
  fetch(req) {
    const url = new URL(req.url);
    
    if (url.pathname === "/") {
      return new Response("Hello Bun!");
    }
    
    if (url.pathname === "/api/users") {
      return Response.json({ users: ["Alice", "Bob"] });
    }
    
    return new Response("Not Found", { status: 404 });
  },
});

console.log(`Server running on http://localhost:${server.port}`);
```

## Request Handling

### JSON Requests

```typescript
fetch(req) {
  if (req.method === "POST") {
    const body = await req.json();
    // Process body
    return Response.json({ success: true });
  }
}
```

### Form Data & File Uploads

```typescript
fetch(req) {
  const formData = await req.formData();
  const file = formData.get("file") as File;
  
  await Bun.write(`uploads/${file.name}`, file);
  
  return Response.json({ uploaded: file.name });
}
```

### Headers

```typescript
fetch(req) {
  const auth = req.headers.get("Authorization");
  
  return new Response("OK", {
    headers: {
      "Content-Type": "application/json",
      "X-Custom-Header": "value",
    },
  });
}
```

## WebSocket Support

```typescript
const server = Bun.serve({
  fetch(req, server) {
    const url = new URL(req.url);
    
    if (url.pathname === "/ws") {
      // Upgrade to WebSocket
      const success = server.upgrade(req);
      if (success) return undefined;
      return new Response("WebSocket upgrade failed", { status: 500 });
    }
    
    return new Response("Use /ws for WebSocket");
  },
  
  websocket: {
    open(ws) {
      console.log("Client connected");
      ws.send("Welcome!");
    },
    
    message(ws, message) {
      console.log("Received:", message);
      ws.send(`Echo: ${message}`);
    },
    
    close(ws) {
      console.log("Client disconnected");
    },
  },
});
```

## Server Options

```typescript
Bun.serve({
  port: 3000,
  hostname: "127.0.0.1", // Bind to localhost only
  
  development: Bun.env.NODE_ENV !== "production",
  
  // Request body size limit
  maxRequestBodySize: 1024 * 1024 * 10, // 10MB
  
  fetch(req) {
    return new Response("OK");
  },
  
  error(error) {
    return new Response(`Server Error: ${error.message}`, { 
      status: 500 
    });
  },
});
```

## Best Practices

1. **Validate request paths** to prevent traversal attacks
2. **Set maxRequestBodySize** to prevent DoS
3. **Bind to 127.0.0.1** for dev, configure for production
4. **Authenticate before WebSocket upgrade**
5. **Use Response.json()** for JSON responses
6. **Handle errors** with custom error handler

## Common Patterns

### RESTful API

```typescript
fetch(req) {
  const url = new URL(req.url);
  const route = url.pathname;
  const method = req.method;
  
  if (route === "/api/items" && method === "GET") {
    return Response.json({ items: getItems() });
  }
  
  if (route.startsWith("/api/items/") && method === "GET") {
    const id = route.split("/")[3];
    return Response.json(getItem(id));
  }
  
  if (route === "/api/items" && method === "POST") {
    const item = await req.json();
    return Response.json(createItem(item), { status: 201 });
  }
  
  return new Response("Not Found", { status: 404 });
}
```

### Static File Serving

```typescript
fetch(req) {
  const url = new URL(req.url);
  const filepath = `./public${url.pathname}`;
  
  const file = Bun.file(filepath);
  if (await file.exists()) {
    return new Response(file);
  }
  
  return new Response("Not Found", { status: 404 });
}
```

## Anti-patterns

- **DON'T** bind to 0.0.0.0 in development unnecessarily
- **DON'T** forget to handle 404s
- **DON'T** upgrade WebSocket without authentication
- **DON'T** ignore maxRequestBodySize (DoS risk)

## References

- https://bun.sh/docs/api/http
- https://bun.sh/docs/api/websockets
