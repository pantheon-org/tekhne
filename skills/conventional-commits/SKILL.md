---
name: conventional-commits
description: Generates and formats git commit messages following the Conventional Commits specification. Use when the user asks for help writing commit messages, formatting git commits, wants to write a commit msg for staged changes, needs to produce a changelog entry, or mentions conventional commits, semantic versioning, or version bumps. Analyzes diffs or change descriptions, selects the correct type/scope, writes an imperative-mood header under 72 characters, and composes optional body and footer sections including breaking-change notation.
metadata:
  title: Writing Conventional Commit Messages
  type: skill
  category: Development
  version: 1.0.0
  tags:
    - git
    - commits
    - version-control
    - semantic-versioning
    - best-practices
  last_updated: 2026-01-26
---

## Commit Message Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Rules:**
- Description: imperative mood (`add` not `added`), lowercase, no trailing period, ≤ 72 characters
- Body: separated by one blank line, lines wrapped at 72 characters, explains *why* not *what*
- Footers: `Token: value` format (e.g. `Closes: #123`, `BREAKING CHANGE: description`)

## Type Reference

| Type        | SemVer Impact         | Use When                          |
| ----------- | --------------------- | --------------------------------- |
| `feat`      | MINOR (1.0.0 → 1.1.0) | New feature or user-facing capability |
| `fix`       | PATCH (1.0.0 → 1.0.1) | Bug fix or incorrect behavior     |
| `build`     | None                  | Build system or dependency changes |
| `ci`        | None                  | CI/CD configuration changes       |
| `docs`      | None                  | Documentation only                |
| `style`     | None                  | Code formatting, no logic change  |
| `refactor`  | None                  | Restructuring, same behavior      |
| `perf`      | None                  | Measurable performance improvement |
| `test`      | None                  | Adding or updating tests          |
| `chore`     | None                  | Maintenance not covered above     |
| `revert`    | Varies                | Reverting a previous commit       |

**Breaking changes:** append `!` after type/scope (`feat!:`) and/or add a `BREAKING CHANGE:` footer — triggers a MAJOR version bump.

## Examples

**Simple feature:**
```
feat(auth): add OAuth2 login support
```

**Bug fix with context:**
```
fix(checkout): prevent duplicate order submissions

Race condition when users double-clicked submit during network
latency. Implemented debouncing with a 2-second window.

Fixes: #234
```

**Breaking change with migration info:**
```
refactor!: consolidate API endpoints under /api/v2

BREAKING CHANGE: All endpoints moved from /endpoints/* to /api/v2/*

Migration:
- /endpoints/users → /api/v2/users
- /endpoints/auth  → /api/v2/auth

See docs/migration-v2.md for the full migration guide.
```

**Performance improvement:**
```
perf(search): cache query results with Redis

Cache TTL set to 5 minutes. Reduces average query time from 500ms to 50ms.

Benchmark (1000 req/s sustained):
- Before: 500ms avg, 2s p99
- After:  50ms avg, 200ms p99
```

## Quick Reference

```
feat: add user export
fix(auth): resolve token expiration
docs: update API guide
feat!: remove deprecated endpoint
```

## Team Tooling

For automated commit validation (commitlint + husky), CI enforcement (GitHub Actions), and CHANGELOG/release automation (conventional-changelog, semantic-release), see the [Conventional Commits tooling guide](https://www.conventionalcommits.org/) and [commitlint documentation](https://commitlint.js.org/).

## Related Resources

- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [commitlint Documentation](https://commitlint.js.org/)
