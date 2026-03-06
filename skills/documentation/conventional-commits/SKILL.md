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

## When to Use

- User asks for help writing a commit message or needs a commit for staged changes
- User mentions conventional commits, semantic versioning, or changelog generation
- PR description or release note follows commit message conventions

## When Not to Use

- Setting up commitlint, husky, or semantic-release tooling → see Team Tooling section
- Writing general git commands unrelated to message format

## Workflow

1. Identify the change type from the Type Reference table below
2. Determine scope (optional) — the module, service, or area affected
3. Write the header: `<type>[optional scope]: <description>` — imperative mood, ≤ 72 characters
4. Add body if the change needs explanation of *why* (not *what*), separated by a blank line
5. Add footers for issue references (`Closes: #N`) or breaking changes (`BREAKING CHANGE: <desc>`)

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

## Anti-Patterns

### NEVER write past-tense or noun-phrase descriptions

- **WHY**: Conventional Commits mandate imperative mood; past tense breaks changelog parsers and violates the spec.
- **BAD**: `feat(auth): added OAuth2 login` / `feat(auth): OAuth2 login addition`
- **GOOD**: `feat(auth): add OAuth2 login`

### NEVER omit BREAKING CHANGE footer for breaking changes

- **WHY**: Semantic-release and changelog tools detect MAJOR version bumps from the footer; omitting it silently breaks automated versioning.
- **BAD**: `refactor: rename all API endpoints` with no footer on a breaking rename.
- **GOOD**: `refactor!: rename all API endpoints` with `BREAKING CHANGE: /endpoints/* moved to /api/v2/*` in footer.

### NEVER exceed 72 characters on the header line

- **WHY**: Exceeding 72 chars truncates display in `git log`, GitHub PR views, and terminal output; commitlint will fail CI.
- **BAD**: `feat(user-profile): add support for uploading and cropping avatar images in the settings page`
- **GOOD**: `feat(user-profile): add avatar upload and crop support`

### NEVER use vague or inconsistent scopes

- **WHY**: Inconsistent scopes make changelog filtering impossible and confuse reviewers about change boundaries.
- **BAD**: `fix(stuff): resolve issue` alongside `fix(auth): fix token expiry` in the same repo.
- **GOOD**: Agree on a scope list (module names, service names) and apply it consistently across the team.

## Team Tooling

For automated commit validation (commitlint + husky), CI enforcement (GitHub Actions), and CHANGELOG/release automation (conventional-changelog, semantic-release), see the [Conventional Commits tooling guide](https://www.conventionalcommits.org/) and [commitlint documentation](https://commitlint.js.org/).

## Related Resources

- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [commitlint Documentation](https://commitlint.js.org/)
