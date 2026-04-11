# Scenario 02: Refactor SQL Queries to Use Prepared Statements

## User Prompt

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

## Expected Behavior

1. Identify `findUserByEmail`, `getUserPosts`, `searchProducts`, and `deleteOldSessions` as vulnerable to SQL injection via string interpolation; correctly note `getAdminUsers` as safe
2. Provide a concrete example of an exploitable input for at least one vulnerable query (e.g., `email = "' OR '1'='1"` for `findUserByEmail`)
3. Refactor all four vulnerable queries to use `db.prepare(sql)` with `?` placeholders and pass values as arguments to `.all()` or `.run()`
4. Handle the LIKE wildcard pattern safely by including `%` characters in the bound value (e.g., `db.prepare('... LIKE ?').all('%' + term + '%')`)
5. Leave `getAdminUsers` unchanged (or note it as a style-only change if modified)
6. Ensure no template literal interpolation remains inside any query string passed to `db.query()` or `db.prepare()`

## Success Criteria

- **All four interpolated queries identified as vulnerable**: Agent identifies `findUserByEmail`, `getUserPosts`, `searchProducts`, and `deleteOldSessions` as using string interpolation with user-controlled input; `getAdminUsers` correctly identified as safe (no interpolation)
- **Injection example provided for at least one query**: Agent provides a concrete example of an input that would exploit at least one query
- **db.prepare() used for all four refactored queries**: All four vulnerable queries use `db.prepare(sql)` with `?` placeholders and pass values as arguments to `.all()` or `.run()`
- **LIKE pattern with bound parameter**: `searchProducts` uses a prepared statement where the LIKE wildcard pattern is constructed safely (e.g., `% characters` included in the bound value)
- **getAdminUsers left unchanged**: Agent does not modify `getAdminUsers` since it contains no interpolated user input; or if modified, explains why as a style choice only
- **No interpolation in produced module**: The produced module contains no template literal interpolation inside any query string passed to `db.query()` or `db.prepare()`

## Failure Conditions

- Agent misses one or more of the four vulnerable functions
- Agent does not provide a concrete injection example for any function
- Agent refactors queries using string concatenation instead of `db.prepare()` with bound parameters
- Agent incorrectly handles the LIKE wildcard, placing `%` characters in the SQL string rather than the bound value
- Agent modifies `getAdminUsers` without explaining why
- Agent leaves template literal interpolation in any refactored query
