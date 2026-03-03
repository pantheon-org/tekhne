# bun-development

## Overview

Complete Bun.js ecosystem guidance covering runtime APIs, file I/O, package management, testing, SQLite, and security.

## Structure

```
bun-development/
  SKILL.md       # Main navigation hub
  AGENTS.md      # This file - complete reference guide
  references/    # Detailed reference files by category
```

## Usage

1. Read `SKILL.md` to understand when to use Bun-specific features
2. Navigate to categories based on your task
3. Load reference files on-demand
4. Prefer Bun-native APIs over Node.js equivalents for performance

## Reference Categories

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Runtime & Core APIs | CRITICAL | `runtime-` |
| 2 | File I/O Operations | CRITICAL | `file-` |
| 3 | Testing Framework | HIGH | `testing-` |
| 4 | SQLite Integration | HIGH | `sqlite-` |
| 5 | Package Management | MEDIUM | `package-` |
| 6 | Security Practices | MEDIUM | `security-` |

## Available References

**Runtime & Core APIs** (`runtime-`):
- `references/runtime-globals.md` - Bun.file, Bun.serve, Bun.write, Bun.env
- `references/runtime-http-server.md` - Bun.serve HTTP and WebSocket APIs
- `references/runtime-shell.md` - Bun.$ shell command execution
- `references/runtime-password.md` - Bun.password hashing APIs

**File I/O Operations** (`file-`):
- `references/file-io-patterns.md` - Bun.file, Bun.write, streaming
- `references/file-glob.md` - Bun.Glob for file scanning
- `references/file-vs-node.md` - When to use Bun vs node:fs

**Testing Framework** (`testing-`):
- `references/testing-bun-test.md` - Test runner, describe, test, expect
- `references/testing-matchers.md` - Built-in matchers and assertions
- `references/testing-mocking.md` - Mock functions and modules
- `references/testing-snapshots.md` - Snapshot testing

**SQLite Integration** (`sqlite-`):
- `references/sqlite-basics.md` - bun:sqlite Database class
- `references/sqlite-prepared-statements.md` - Performance with prepared statements
- `references/sqlite-transactions.md` - ACID transactions
- `references/sqlite-migrations.md` - Schema migrations

**Package Management** (`package-`):
- `references/package-installation.md` - bun install, add, remove
- `references/package-workspaces.md` - Monorepo workspace configuration
- `references/package-lockfile.md` - bun.lockb binary lockfile

**Security Practices** (`security-`):
- `references/security-shell-escaping.md` - Preventing command injection
- `references/security-sql-injection.md` - SQL injection prevention
- `references/security-serve.md` - Securing Bun.serve endpoints

---

*Consolidates 6 original skills: bun-file-io, bun-package-manager, bun-runtime, bun-sqlite, bun-testing, security-bun*

*22 reference files across 6 categories*
