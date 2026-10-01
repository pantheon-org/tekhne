---
name: bun-development
description: Complete Bun.js ecosystem guidance for runtime APIs, file I/O, package management, testing, SQLite, and security; use proactively when setting up Bun projects, replacing Node.js APIs with Bun-native APIs, writing bun test suites, implementing Bun.serve services, using bun:sqlite with prepared statements, configuring workspaces and lockfiles, hardening shell and SQL boundaries, or optimizing Bun performance and migration workflows.
allowed-tools: Read, Bash, Write, Edit
---

# Bun Development

Navigation hub for Bun guidance focused on production-safe implementation.

## Philosophy

- Prefer Bun-native APIs over Node.js shims: one runtime, one set of semantics.
- Treat every string that crosses a shell or SQL boundary as hostile until it is bound or passed as an argument.
- Keep installs deterministic: one package manager, one lockfile, no drift.
- Validate every change by running it, not by reading it.

## When to Use

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
   - If `bun install` fails: check `bun.lock`/`bun.lockb` for merge conflicts, resolve them, and re-run `bun install`.
   - If tests fail after dependency changes: run `bun test` to isolate regressions before proceeding.
5. Validate behavior with tests or execution checks.
   - If validation fails: revert the last change, confirm the error, and address the specific failure before re-validating.
6. For SQL validation (`rg` finds non-prepared statements in existing code): refactor each flagged call to use `db.prepare(...)` with bound parameters before merging; do not leave raw interpolations in place.
7. Document decisions with links to exact references used.

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

Expected: queries in new code paths use prepared statements where input is user-controlled. If raw `query(` calls with interpolated values are found in existing code, refactor them to `db.prepare(...)` with bound parameters before proceeding.

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

**WHY:** mixed APIs create inconsistent behaviour and miss Bun performance advantages.

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

**WHY:** mixed package managers cause lockfile drift and nondeterministic installs.

**BAD**: `npm install`

**GOOD**: `bun install`

### NEVER interpolate untrusted input into SQL strings

**WHY:** direct interpolation can introduce SQL injection vulnerabilities.

**BAD**:

```ts
db.query(`SELECT * FROM users WHERE email = '${email}'`).all();
```

**GOOD**:

```ts
db.prepare("SELECT * FROM users WHERE email = ?").all(email);
```

### NEVER pass unescaped user input into shell commands

**WHY:** shell interpolation enables command injection.

**BAD**:

```ts
await Bun.spawn(["sh", "-c", userInput]).exited;
```

**GOOD**:

```ts
const safePath = Bun.file(userProvidedPath);
await safePath.text();
```

### NEVER leave raw interpolated `query(` calls in existing code once flagged

**WHY:** the SQL validation step exists to find them, and an unrefactored hit is a live injection path.

**BAD**: merging while an interpolated `db.query` call that `rg` flagged is still present.

**GOOD**: refactor each flagged call to `db.prepare(...)` with bound parameters before merging.

### NEVER ignore a lockfile merge conflict when `bun install` fails

**WHY:** an unresolved `bun.lock`/`bun.lockb` conflict leaves installs nondeterministic.

**BAD**: re-running `bun install` repeatedly without opening the lockfile.

**GOOD**: resolve the conflict markers, then re-run `bun install`.

### NEVER skip `bun test` after changing dependencies

**WHY:** dependency changes can cause regressions that only a test run isolates.

**BAD**: updating packages and moving straight to the next step.

**GOOD**: run `bun test` straight after the change and fix regressions before proceeding.

### NEVER keep going after a failed validation

**WHY:** stacking changes on a failing state hides which change broke behaviour.

**BAD**: continuing to edit while the validation error is unexplained.

**GOOD**: revert the last change, confirm the error, then address the specific failure before re-validating.

### NEVER finalise a change without running the anti-pattern checks

**WHY:** the checks are the guard against the injection and package-manager mistakes above.

**BAD**: finishing straight after implementation.

**GOOD**: apply the anti-pattern checks (step 4 of the Workflow) before finalising.

### NEVER record a decision without linking the reference you used

**WHY:** unlinked decisions cannot be re-checked when Bun behaviour changes.

**BAD**: "Used Bun.file because it is better."

**GOOD**: "Used `Bun.file` per `references/file-io-patterns.md`."

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Runtime globals | `references/runtime-globals.md` | Using Bun runtime globals |
| HTTP server | `references/runtime-http-server.md` | Building services with `Bun.serve` |
| File I/O patterns | `references/file-io-patterns.md` | Reading and writing files |
| Bun vs Node file APIs | `references/file-vs-node.md` | Migrating from `node:fs` |
| Globbing | `references/file-glob.md` | Matching files by pattern |
| Test runner | `references/testing-bun-test.md` | Writing `bun test` suites |
| Matchers | `references/testing-matchers.md` | Choosing assertions |
| Mocking | `references/testing-mocking.md` | Mocking modules and functions |
| Snapshots | `references/testing-snapshots.md` | Snapshot testing |
| SQLite | `references/sqlite-basics.md` | Using `bun:sqlite` |
| Workspaces | `references/pm-workspaces-agent-instructions.md` | Package and workspace policy |
| Shell safety | `references/runtime-shell.md` | Running subprocesses safely |
| Passwords | `references/runtime-password.md` | Hashing and verifying passwords |

## External Links

- [Bun Docs](https://bun.sh/docs)
- [Bun File I/O](https://bun.sh/docs/api/file-io)
- [Bun Test CLI](https://bun.sh/docs/cli/test)
- [Bun SQLite API](https://bun.sh/docs/api/sqlite)
