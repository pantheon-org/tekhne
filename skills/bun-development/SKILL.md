---
name: bun-development
description: |-
  Complete Bun.js ecosystem guidance covering runtime APIs, file I/O, package management, testing, SQLite, and security. Use when: working with Bun.file, Bun.serve, bun test, bun:sqlite, package management, or securing Bun applications.
  
  Keywords: Bun, Bun.file, Bun.serve, Bun.write, bun test, bun:sqlite, workspaces, lockfile, shell escaping, HTTP server, WebSocket, password hashing
allowed-tools:
  - Read
  - Bash
  - Write
  - Edit
---

# Bun Development

Comprehensive Bun.js ecosystem guidance covering runtime, file I/O, testing, SQLite, package management, and security best practices.

## When to Apply

Use this skill when:
- Working with Bun's runtime APIs (file I/O, HTTP servers)
- Writing tests with `bun test`
- Using bun:sqlite for database operations
- Managing packages and workspaces
- Securing Bun applications against injection attacks
- Optimizing performance with Bun-native APIs

## Categories by Priority

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Runtime & Core APIs | CRITICAL | `runtime-` |
| 2 | File I/O Operations | CRITICAL | `file-` |
| 3 | Testing Framework | HIGH | `testing-` |
| 4 | SQLite Integration | HIGH | `sqlite-` |
| 5 | Package Management | MEDIUM | `package-` |
| 6 | Security Practices | MEDIUM | `security-` |

## How to Use

Read individual reference files for detailed guidance:

```
references/runtime-globals.md
references/file-io-patterns.md
references/testing-bun-test.md
```

Each reference file contains:
- API documentation and usage
- Practical code examples
- Best practices and patterns
- Common pitfalls to avoid
- Performance considerations

## References

- https://bun.sh/docs
- https://bun.sh/docs/api/file-io
- https://bun.sh/docs/cli/test
- https://bun.sh/docs/api/sqlite
