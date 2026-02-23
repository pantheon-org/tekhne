---
name: bun-development
description: Complete Bun.js ecosystem guidance for runtime APIs, file I/O, package management, testing, SQLite, and security; use proactively when setting up Bun projects, replacing Node.js APIs with Bun-native APIs, writing bun test suites, implementing Bun.serve services, using bun:sqlite with prepared statements, configuring workspaces and lockfiles, hardening shell and SQL boundaries, or optimizing Bun performance and migration workflows.
allowed-tools:
  - Read
  - Bash
  - Write
  - Edit
---

# Bun Development

Navigation hub for Bun guidance focused on production-safe implementation.

## When to Apply

Use this skill when:

- You need Bun-native runtime APIs (`Bun.file`, `Bun.write`, `Bun.serve`)
- You are writing or debugging tests with `bun test`
- You are adding `bun:sqlite` usage with transactions or prepared statements
- You are setting dependency/workspace policy in a Bun project
- You need deterministic security guardrails for shell/SQL inputs

## When Not to Use

- The project standard runtime is strictly Node.js with no Bun support
- You need framework-specific guidance not covered by Bun APIs
- The task is only package-agnostic JavaScript refactoring

## Workflow

1. Confirm Bun is available and lock dependency strategy.
2. Choose the category (runtime, file I/O, testing, sqlite, package, security).
3. Implement with copy/paste commands from Quick Commands.
4. Apply anti-pattern checks before finalizing.
5. Validate behavior with tests or execution checks.
6. Document decisions with links to exact references used.

## Quick Commands

### Verify Runtime

```bash
bun --version
```

Expected: a Bun version is printed.

### Install Dependencies

```bash
bun install
```

Expected: lockfile is updated consistently (`bun.lock`/`bun.lockb` per project setup).

### Run Script

```bash
bun run src/index.ts
```

Expected: script executes without Node.js runtime shims.

### Execute Tests

```bash
bun test
```

Expected: failing assertions clearly identify behavior regressions.

### Start HTTP Service

```bash
bun run server.ts
```

Expected: service binds configured port and handles requests via `Bun.serve`.

### Validate SQL Pattern

```bash
rg -n "query\\(|prepare\\(" src
```

Expected: queries in new code paths use prepared statements where input is user-controlled.

## Categories by Priority

| Priority | Category | Impact | Prefix |
| --- | --- | --- | --- |
| 1 | Runtime & Core APIs | CRITICAL | `runtime-` |
| 2 | File I/O Operations | CRITICAL | `file-` |
| 3 | Testing Framework | HIGH | `testing-` |
| 4 | SQLite Integration | HIGH | `sqlite-` |
| 5 | Package Management | MEDIUM | `package-` |
| 6 | Security Practices | MEDIUM | `security-` |

## Anti-Patterns

### NEVER mix Node.js fs calls into Bun-native file workflows

**WHY**: mixed APIs create inconsistent behavior and miss Bun performance advantages.
BAD: `readFileSync("./config.json")` for Bun-managed file reads. GOOD: `await Bun.file("./config.json").text()`.

**BAD**:

```ts
import { readFileSync } from "node:fs";
const config = readFileSync("./config.json", "utf8");
```

**GOOD**:

```ts
const config = await Bun.file("./config.json").text();
```

### NEVER run `npm install` in Bun-managed repositories

**WHY**: mixed package managers cause lockfile drift and nondeterministic installs.
**BAD**: `npm install`
**GOOD**: `bun install`

### NEVER interpolate untrusted input into SQL strings

**WHY**: direct interpolation can introduce SQL injection vulnerabilities.
BAD: interpolate untrusted values into query strings. GOOD: bind values with prepared statements.

**BAD**:

```ts
db.query(`SELECT * FROM users WHERE email = '${email}'`).all();
```

**GOOD**:

```ts
db.prepare("SELECT * FROM users WHERE email = ?").all(email);
```

### NEVER pass unescaped user input into shell commands

**WHY**: shell interpolation enables command injection.
BAD: `await Bun.spawn(["sh", "-c", userInput]).exited`. GOOD: use direct API calls or fixed command arguments.

**BAD**:

```ts
await Bun.spawn(["sh", "-c", userInput]).exited;
```

**GOOD**:

```ts
const safePath = Bun.file(userProvidedPath);
await safePath.text();
```

## Reference Map

- Runtime core: `references/runtime-globals.md`, `references/runtime-http-server.md`
- File I/O: `references/file-io-patterns.md`, `references/file-vs-node.md`, `references/file-glob.md`
- Testing: `references/testing-bun-test.md`, `references/testing-matchers.md`, `references/testing-mocking.md`
- SQLite: `references/sqlite-basics.md`
- Package/workspaces: `references/pm-workspaces-agent-instructions.md`
- Security: `references/runtime-shell.md`, `references/runtime-password.md`

## References

- [Bun Docs](https://bun.sh/docs)
- [Bun File I/O](https://bun.sh/docs/api/file-io)
- [Bun Test CLI](https://bun.sh/docs/cli/test)
- [Bun SQLite API](https://bun.sh/docs/api/sqlite)
