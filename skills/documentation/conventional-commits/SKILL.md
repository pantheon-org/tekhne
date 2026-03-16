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

## Mindset

Conventional Commits exist to make commit history machine-readable. The type prefix is a contract: `feat` triggers a MINOR version bump, `fix` triggers PATCH, and `!` or `BREAKING CHANGE:` triggers MAJOR. Every commit is a semantic event, not just a description of code changes.

Write the header for the person reviewing a CHANGELOG six months from now, not for yourself today. The body explains *why* — the code already shows *what*.

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

## Anti-Patterns

### NEVER use past tense in the description

- **WHY**: The header completes the sentence "If applied, this commit will…" — past tense breaks that contract and reads as a changelog entry, not a commit instruction.
- **BAD**: `fix(auth): fixed token expiration bug`
- **GOOD**: `fix(auth): prevent token expiration on refresh`

### NEVER use `chore` as a catch-all

- **WHY**: `chore` is for maintenance tasks not covered by other types (e.g., bumping lock files). Using it for features, refactors, or docs hides semantic meaning from changelog generators.
- **BAD**: `chore: update auth module`, `chore: fix login bug`
- **GOOD**: `refactor(auth): extract token refresh logic`, `fix(auth): resolve null pointer on logout`

### NEVER exceed 72 characters in the header

- **WHY**: Git log, GitHub PR titles, and most CI tools truncate at 72 characters. Longer headers break tooling and lose critical context.
- **BAD**: `feat(checkout): add new payment processing integration with Stripe for recurring subscriptions`
- **GOOD**: `feat(checkout): add Stripe recurring subscription support`

### NEVER omit `BREAKING CHANGE:` footer when the API surface changes

- **WHY**: The `!` suffix alone is not sufficient for all tooling. `BREAKING CHANGE:` in the footer carries the migration description that changelog generators parse for release notes.
- **BAD**: `feat!: remove /v1 endpoints` (no footer)
- **GOOD**: `feat!: remove /v1 endpoints` + footer `BREAKING CHANGE: /v1/* routes deleted; migrate to /api/v2/*`

### NEVER put implementation details in the header

- **WHY**: The header is the commit summary visible in `git log --oneline`. Implementation details belong in the body.
- **BAD**: `fix(db): change SQL query from SELECT * to SELECT id,name in getUserById`
- **GOOD**: `fix(db): reduce getUserById query to required columns` (details in body)

## Team Tooling

For automated commit validation (commitlint + husky), CI enforcement (GitHub Actions), and CHANGELOG/release automation (conventional-changelog, semantic-release), see the [Conventional Commits tooling guide](https://www.conventionalcommits.org/) and [commitlint documentation](https://commitlint.js.org/).

## References

- [Conventional Commits Specification](https://www.conventionalcommits.org/) — canonical spec defining types, scopes, breaking changes, and CHANGELOG generation
- [Semantic Versioning](https://semver.org/) — versioning contract that Conventional Commits maps to (MAJOR/MINOR/PATCH)
- [commitlint Documentation](https://commitlint.js.org/) — automated commit message validation for CI and pre-commit hooks
