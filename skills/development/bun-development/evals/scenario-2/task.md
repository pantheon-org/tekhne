# Task: Refactor SQL Queries to Use Prepared Statements

A `bun:sqlite` data access module has been flagged for SQL injection vulnerabilities. Every raw string-interpolated query must be refactored to use `db.prepare()` with bound parameters. Review the code, identify each vulnerable query, explain the injection risk, and produce a safe version.

```typescript
import { Database } from "bun:sqlite";

const db = new Database("app.db");

export function findUserByEmail(email: string) {
  return db.query(`SELECT * FROM users WHERE email = '${email}'`).all();
}

export function getUserPosts(userId: string, status: string) {
  return db.query(
    `SELECT * FROM posts WHERE user_id = ${userId} AND status = '${status}' ORDER BY created_at DESC`
  ).all();
}

export function searchProducts(term: string, category: string) {
  const sql = `SELECT id, name, price FROM products WHERE name LIKE '%${term}%' AND category = '${category}'`;
  return db.query(sql).all();
}

export function deleteOldSessions(userId: string) {
  db.query(`DELETE FROM sessions WHERE user_id = '${userId}' AND expires_at < datetime('now')`).run();
}

export function getAdminUsers() {
  return db.query("SELECT id, email, role FROM users WHERE role = 'admin'").all();
}
```

Produce the refactored module. For each vulnerable query, show the original, explain what input could exploit it, and show the prepared-statement replacement.
