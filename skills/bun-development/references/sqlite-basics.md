# Bun SQLite Basics

## Overview

Bun includes a built-in, high-performance SQLite driver (`bun:sqlite`) with zero dependencies.

## Opening a Database

```typescript
import { Database } from "bun:sqlite";

// Open or create database file
const db = new Database("mydb.sqlite");

// In-memory database
const memDb = new Database(":memory:");

// Read-only database
const readOnlyDb = new Database("mydb.sqlite", { readonly: true });

// Create database with options
const db = new Database("mydb.sqlite", {
  create: true,      // Create if doesn't exist (default: true)
  readonly: false,   // Read-only mode (default: false)
  strict: true       // Enable strict mode (default: false)
});
```

## Basic Queries

### Creating Tables

```typescript
import { Database } from "bun:sqlite";

const db = new Database("mydb.sqlite");

// Create table
db.run(`
  CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    created_at INTEGER DEFAULT (unixepoch())
  )
`);
```

### Inserting Data

```typescript
// Insert single row
db.run(
  "INSERT INTO users (name, email) VALUES (?, ?)",
  "Alice",
  "alice@example.com"
);

// Get last inserted ID
const result = db.run(
  "INSERT INTO users (name, email) VALUES (?, ?)",
  "Bob",
  "bob@example.com"
);
console.log("Inserted ID:", result.lastInsertRowid);
console.log("Changes:", result.changes);
```

### Querying Data

```typescript
// Get single row
const user = db.query("SELECT * FROM users WHERE id = ?").get(1);
console.log(user); // { id: 1, name: "Alice", email: "alice@example.com", ... }

// Get all rows
const users = db.query("SELECT * FROM users").all();
console.log(users); // [{ id: 1, ... }, { id: 2, ... }]

// Get specific values only
const emails = db.query("SELECT email FROM users").values();
console.log(emails); // [["alice@example.com"], ["bob@example.com"]]
```

### Updating Data

```typescript
const result = db.run(
  "UPDATE users SET name = ? WHERE id = ?",
  "Alice Smith",
  1
);
console.log("Rows updated:", result.changes);
```

### Deleting Data

```typescript
const result = db.run("DELETE FROM users WHERE id = ?", 1);
console.log("Rows deleted:", result.changes);
```

## Query Methods

### `.run()` - Execute without returning rows

```typescript
// For INSERT, UPDATE, DELETE, CREATE, DROP
const result = db.run("INSERT INTO users (name, email) VALUES (?, ?)", "Alice", "alice@example.com");
// Returns: { changes: number, lastInsertRowid: number }
```

### `.query().get()` - Get single row

```typescript
const query = db.query("SELECT * FROM users WHERE id = ?");
const user = query.get(1);
// Returns: object | undefined
```

### `.query().all()` - Get all rows

```typescript
const query = db.query("SELECT * FROM users");
const users = query.all();
// Returns: array of objects
```

### `.query().values()` - Get raw values

```typescript
const query = db.query("SELECT name, email FROM users");
const values = query.values();
// Returns: [["Alice", "alice@example.com"], ["Bob", "bob@example.com"]]
```

## Parameterized Queries

### Positional Parameters (`?`)

```typescript
const user = db.query("SELECT * FROM users WHERE id = ?").get(1);

const user2 = db.query("SELECT * FROM users WHERE name = ? AND email = ?")
  .get("Alice", "alice@example.com");
```

### Named Parameters

```typescript
const user = db.query("SELECT * FROM users WHERE id = $id")
  .get({ $id: 1 });

const user2 = db.query("SELECT * FROM users WHERE name = $name AND email = $email")
  .get({ $name: "Alice", $email: "alice@example.com" });
```

## Closing the Database

```typescript
db.close();

// Check if closed
console.log(db.closed); // true
```

## Error Handling

```typescript
import { Database } from "bun:sqlite";

try {
  const db = new Database("mydb.sqlite");
  
  db.run("INSERT INTO users (name, email) VALUES (?, ?)", "Alice", "alice@example.com");
  
  db.close();
} catch (error) {
  console.error("Database error:", error);
}
```

## Common Patterns

### CRUD Operations Class

```typescript
import { Database } from "bun:sqlite";

class UserRepository {
  private db: Database;
  
  constructor(dbPath: string) {
    this.db = new Database(dbPath);
    this.initTable();
  }
  
  private initTable() {
    this.db.run(`
      CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT UNIQUE NOT NULL,
        created_at INTEGER DEFAULT (unixepoch())
      )
    `);
  }
  
  create(name: string, email: string) {
    return this.db.run(
      "INSERT INTO users (name, email) VALUES (?, ?)",
      name,
      email
    );
  }
  
  findById(id: number) {
    return this.db.query("SELECT * FROM users WHERE id = ?").get(id);
  }
  
  findAll() {
    return this.db.query("SELECT * FROM users").all();
  }
  
  update(id: number, name: string, email: string) {
    return this.db.run(
      "UPDATE users SET name = ?, email = ? WHERE id = ?",
      name,
      email,
      id
    );
  }
  
  delete(id: number) {
    return this.db.run("DELETE FROM users WHERE id = ?", id);
  }
  
  close() {
    this.db.close();
  }
}
```

### Checking if Table Exists

```typescript
const tableExists = db.query(`
  SELECT name FROM sqlite_master 
  WHERE type='table' AND name=?
`).get("users");

if (!tableExists) {
  db.run("CREATE TABLE users (...)");
}
```

## Best Practices

1. **Always use parameterized queries** - Prevent SQL injection
2. **Close database connections** - Call `db.close()` when done
3. **Check query results** - `.get()` returns `undefined` if no row found
4. **Handle errors** - Wrap database operations in try/catch
5. **Use appropriate query methods** - `.run()` for mutations, `.query()` for selects

## Anti-patterns

- ❌ String interpolation for queries (SQL injection risk)
- ❌ Not closing database connections
- ❌ Ignoring query errors
- ❌ Using `.run()` for SELECT queries
- ❌ Not checking if results exist before accessing

## Performance Tips

- Use prepared statements for repeated queries (next section)
- Use transactions for bulk operations (see sqlite-transactions.md)
- Create indexes on frequently queried columns
- Use `EXPLAIN QUERY PLAN` to optimize slow queries
