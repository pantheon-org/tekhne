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

A less obvious implication: **scope consistency drives changelog quality more than type correctness.** A perfectly typed `feat` with an ad-hoc scope like `feat(getUserById):` produces a one-line changelog group with no neighbours. Establish a fixed scope vocabulary early — see [Scope Naming and Commit Strategy](references/scope-and-strategy.md).

## When to Use

Apply when the user asks for help writing commit messages, wants to format staged changes as a commit, needs a CHANGELOG entry, or mentions conventional commits, semantic versioning, or version bumps.

When not to use: skip this skill if the project explicitly avoids Conventional Commits (e.g. uses Angular-style without the spec, or enforces a custom pattern via a project-specific hook). Do not override confirmed project conventions.

## Procedure

1. **Inspect the change** — read the diff or the user's description. Identify whether it adds new behaviour, fixes existing behaviour, or neither.
2. **Choose the type** — use the Type Reference table. Decision rules for common ambiguities:
   - `refactor` vs `feat`: does the user gain new capability? If yes → `feat`
   - `fix` vs `refactor`: does the change correct wrong behaviour? If yes → `fix`
   - `perf` vs `refactor`: is there a measurable latency/throughput improvement? If yes → `perf`
   - `chore` vs anything else: if another type fits, use it — `chore` is the last resort
   - Multiple types needed? Split into multiple commits (see [Atomic Commits](references/scope-and-strategy.md))
3. **Pick the scope** — use the module, package, or layer name (e.g. `auth`, `checkout`, `api`). Omit scope when the change is truly cross-cutting. Use a fixed vocabulary if the project defines one. See [scope naming](references/scope-and-strategy.md) for conventions.
4. **Draft the header** — imperative mood, lowercase, ≤ 72 characters. Verify with: "If applied, this commit will `<header>`."
5. **Add a body if needed** — only when the *why* is not obvious. Explain the reasoning, not the implementation. Wrap at 72 characters.
6. **Add footers** — `Closes: #N` for issue references; `BREAKING CHANGE: <description>` whenever the public API surface changes. Always pair `!` with a `BREAKING CHANGE:` footer.
7. **Verify before committing** — run `git commit --dry-run` or your local commitlint hook to catch formatting violations before they enter history.

**Agent output format:** Present the commit message in a fenced code block with no extra commentary, ready to paste:

```
feat(auth): add OAuth2 login support

Allows users to sign in with Google and GitHub. Replaces the
username/password flow for external accounts.

Closes: #412
```

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
- **BAD**: `fix(auth): fixed token expiration bug` — **GOOD**: `fix(auth): prevent token expiration on refresh`

### NEVER use `chore` as a catch-all

- **WHY**: `chore` is for maintenance tasks not covered by other types (e.g., bumping lock files). Using it for features, refactors, or docs hides semantic meaning from changelog generators.
- **BAD**: `chore: update auth module`, `chore: fix login bug` — **GOOD**: `refactor(auth): extract token refresh logic`, `fix(auth): resolve null pointer on logout`

### NEVER exceed 72 characters in the header

- **WHY**: Git log, GitHub PR titles, and most CI tools truncate at 72 characters. Longer headers break tooling and lose critical context.
- **BAD**: `feat(checkout): add new payment processing integration with Stripe for recurring subscriptions` — **GOOD**: `feat(checkout): add Stripe recurring subscription support`

### NEVER omit `BREAKING CHANGE:` footer when the API surface changes

- **WHY**: The `!` suffix alone is not sufficient for all tooling. `BREAKING CHANGE:` in the footer carries the migration description that changelog generators parse for release notes.
- **BAD**: `feat!: remove /v1 endpoints` (no footer) — **GOOD**: `feat!: remove /v1 endpoints` + footer `BREAKING CHANGE: /v1/* routes deleted; migrate to /api/v2/*`

### NEVER put implementation details in the header

- **WHY**: The header is the commit summary visible in `git log --oneline`. Implementation details belong in the body.
- **BAD**: `fix(db): change SQL query from SELECT * to SELECT id,name in getUserById` — **GOOD**: `fix(db): reduce getUserById query to required columns` (details in body)

### NEVER bundle unrelated changes in one commit

- **WHY**: A commit that spans multiple concerns can't be cherry-picked or reverted cleanly. It also produces ambiguous changelog entries and forces reviewers to untangle intent from implementation.
- **BAD**: `fix(auth): prevent token expiration and update README and bump deps` — **GOOD**: Three commits: `fix(auth): prevent token expiration`, `docs(auth): document token refresh flow`, `chore(deps): bump jsonwebtoken to 9.0.2`

## Quick Validation

Validate a message ad-hoc:

```bash
echo "feat(auth): add OAuth2 login" | npx commitlint
```

Scan recent history:

```bash
git log --oneline -10
git log --format="%s" HEAD~5..
```

For full commitlint setup, CHANGELOG generation, and semantic-release, see [Team Tooling](references/tooling.md).

## References

- [Scope Naming and Commit Strategy](references/scope-and-strategy.md) — scope vocabulary, atomic commit patterns, revert format
- [Team Tooling](references/tooling.md) — commitlint setup, CHANGELOG generation, semantic-release, history verification
- [Conventional Commits Specification](https://www.conventionalcommits.org/) — canonical spec defining types, scopes, breaking changes, and CHANGELOG generation
- [Semantic Versioning](https://semver.org/) — versioning contract that Conventional Commits maps to (MAJOR/MINOR/PATCH)
- [commitlint Documentation](https://commitlint.js.org/) — automated commit message validation for CI and pre-commit hooks
