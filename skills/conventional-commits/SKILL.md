---
title: Writing Conventional Commit Messages
description: Skill for creating structured, semantic commit messages following the Conventional Commits specification
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

# Skill: Writing Conventional Commit Messages

This skill guides you through creating structured commit messages that enable automated tooling, clear communication,
and semantic versioning.

## When to Use This Skill

Use this skill when you need to:

- Create commits that integrate with automated tools (CHANGELOG, versioning)
- Communicate changes clearly to team members
- Enable semantic versioning automation
- Maintain a searchable, organized commit history
- Trigger CI/CD workflows based on commit type

## Prerequisites

- Basic understanding of Git
- Familiarity with your project's branching strategy
- Understanding of semantic versioning (MAJOR.MINOR.PATCH)

## Commit Message Structure

Every conventional commit follows this format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Step-by-Step Guide

### Step 1: Choose the Correct Commit Type

Before writing your commit, identify what kind of change you're making:

#### Decision Tree for Type Selection

**Ask yourself these questions in order:**

1. **Does this add new functionality?**
   - YES → Use `feat:`
   - NO → Continue

2. **Does this fix a bug?**
   - YES → Use `fix:`
   - NO → Continue

3. **Does this change build configuration or dependencies?**
   - YES → Use `build:`
   - NO → Continue

4. **Does this change CI/CD configuration?**
   - YES → Use `ci:`
   - NO → Continue

5. **Does this only change documentation?**
   - YES → Use `docs:`
   - NO → Continue

6. **Does this only change code formatting (no functional change)?**
   - YES → Use `style:`
   - NO → Continue

7. **Does this improve code structure without changing behavior?**
   - YES → Use `refactor:`
   - NO → Continue

8. **Does this improve performance?**
   - YES → Use `perf:`
   - NO → Continue

9. **Does this only add or modify tests?**
   - YES → Use `test:`
   - NO → Continue

10. **Is this maintenance work or configuration?**
    - YES → Use `chore:`
    - NO → Continue

11. **Are you reverting a previous commit?**
    - YES → Use `revert:`

#### Type Reference Table

| Type        | SemVer Impact         | Use When                | Example                        |
| ----------- | --------------------- | ----------------------- | ------------------------------ |
| `feat:`     | MINOR (1.0.0 → 1.1.0) | Adding new feature      | `feat: add user export`        |
| `fix:`      | PATCH (1.0.0 → 1.0.1) | Fixing a bug            | `fix: resolve crash on logout` |
| `build:`    | None                  | Build system changes    | `build: upgrade webpack to v5` |
| `ci:`       | None                  | CI/CD changes           | `ci: add security scanning`    |
| `docs:`     | None                  | Documentation only      | `docs: update API guide`       |
| `style:`    | None                  | Code formatting         | `style: apply prettier`        |
| `refactor:` | None                  | Code restructuring      | `refactor: extract helper`     |
| `perf:`     | None                  | Performance improvement | `perf: add caching layer`      |
| `test:`     | None                  | Tests only              | `test: add login tests`        |
| `chore:`    | None                  | Maintenance             | `chore: update .gitignore`     |
| `revert:`   | Varies                | Revert previous commit  | `revert: feat: add feature X`  |

**Special Case - Breaking Changes:** Any type with `!` or `BREAKING CHANGE:` footer triggers MAJOR bump (1.0.0 → 2.0.0)

### Step 2: Add a Scope (Optional but Recommended)

Scopes provide context about what part of the codebase changed.

**Format:** `<type>(<scope>):`

**Choosing a Scope:**

- **By module:** `feat(parser):`, `fix(ui):`
- **By feature:** `feat(auth):`, `fix(checkout):`
- **By component:** `feat(Button):`, `fix(Modal):`
- **By service:** `feat(api):`, `fix(database):`

**Examples:**

```
feat(auth): add OAuth2 support
fix(database): resolve connection leak
docs(api): update authentication guide
refactor(ui-components): extract Button component
```

**When to skip scope:**

- Changes affect multiple areas equally
- Changes are project-wide (e.g., `docs: update README`)
- Scope would be too generic to add value

### Step 3: Write the Description

The description is a one-line summary of the change.

**Rules:**

- Keep under 72 characters (GitHub UI limitation)
- Use imperative mood: "add" not "added" or "adds"
- Start with lowercase
- No trailing period
- Describe _what_ changed, not _why_ (save why for body)

**Good Examples:**

```
feat: add user authentication module
fix: resolve memory leak in parser
docs: update installation instructions
refactor: simplify promise chain handling
```

**Poor Examples:**

```
❌ feat: added user authentication         (past tense)
❌ fix: fixes stuff                        (vague)
❌ Update documentation                   (capitalized, no type)
❌ refactor: Code cleanup and stuff.      (vague, trailing period)
```

### Step 4: Add a Body (When Needed)

Use the body to explain _why_ the change was made and provide context.

**When to add a body:**

- The change is complex or non-obvious
- You need to explain design decisions
- There were alternatives you considered
- The change has important impacts to document

**Format:**

- Separate from description with one blank line
- Wrap lines at 72 characters
- Use multiple paragraphs if needed

**What to include:**

- Problem being solved
- Why this approach was chosen
- Alternatives considered and rejected
- Impact on users or systems
- Testing performed

**Example:**

```
fix(auth): prevent concurrent session exploits

Sessions were vulnerable to race conditions when multiple
login attempts occurred simultaneously. This implements
request deduplication and session lock mechanism.

Alternative considered: Exponential backoff was considered
but would degrade UX for legitimate users with poor networks.

Impact: Users with flaky networks may see brief delays but
the security vulnerability is eliminated.

Tested with: jmeter load test (1000 concurrent requests)
```

### Step 5: Add Footers (When Applicable)

Footers link to issues, note co-authors, or indicate breaking changes.

**Format:** `Token: value` (one per line)

**Common footers:**

- `Closes: #123` or `Fixes: #456` - Links to issue being closed
- `Related-To: #789` - Links to related issue
- `Co-Authored-By: Name <email>` - Credits co-author
- `Reviewed-By: Name` - Notes reviewer
- `BREAKING CHANGE: description` - Indicates breaking change

**Example:**

```
feat(billing): add annual payment option

Allow customers to pay annually with 10% discount.

Closes: #145
Related-To: #132
Co-Authored-By: Alice <alice@company.com>
```

### Step 6: Indicate Breaking Changes (If Applicable)

Breaking changes require action from users or downstream systems.

**Three ways to indicate breaking changes:**

#### Method 1: Exclamation Mark (Quick)

Add `!` before the colon:

```
feat!: remove deprecated login endpoint
fix(api)!: change response format
```

#### Method 2: Footer (Detailed)

Add `BREAKING CHANGE:` footer (uppercase required):

```
feat: redesign authentication flow

BREAKING CHANGE: Old /auth/login endpoint removed.
Use new /auth/v2/login instead.
```

#### Method 3: Both (Recommended for clarity)

```
feat!: redesign user profile API

BREAKING CHANGE: User profile endpoint now returns nested
object structure instead of flat keys.

Migration guide: see docs/migration-v2.md
```

**When is something a breaking change?**

- API contract changes (endpoints, response formats, parameters)
- Configuration format changes
- Behavior changes that break existing usage
- Dependency updates with breaking changes
- Removal of features or endpoints

### Step 7: Review Your Commit

Before finalizing, check:

**Checklist:**

- [ ] Type is correct and follows specification
- [ ] Scope (if used) is clear and consistent with project conventions
- [ ] Description is under 72 characters
- [ ] Description uses imperative mood and lowercase
- [ ] Description has no trailing period
- [ ] Body (if used) explains _why_, not _what_
- [ ] Body lines wrap at 72 characters
- [ ] Footers use correct format (`Token: value`)
- [ ] Breaking changes are indicated with `!` or `BREAKING CHANGE:`
- [ ] Issue references are included (if applicable)

## Common Patterns and Examples

### Pattern 1: Simple Feature Addition

```
feat(search): add filter by date range

Users can now filter search results by custom date ranges.
```

### Pattern 2: Bug Fix with Context

```
fix(checkout): prevent duplicate order submissions

Race condition occurred when users double-clicked submit
button during network latency. Now implement debouncing
with 2-second window.

Fixes: #234
```

### Pattern 3: Breaking Change with Migration

```
refactor!: consolidate API endpoints under /api/v2

BREAKING CHANGE: All endpoints moved from /endpoints/* to /api/v2/*

Migration:
- /endpoints/users → /api/v2/users
- /endpoints/auth → /api/v2/auth

See docs/migration-v2.md for full migration guide.

Co-Authored-By: Bob <bob@company.com>
```

### Pattern 4: Performance Improvement

```
perf(search): implement query result caching

Add Redis cache layer for search queries. Cache TTL set
to 5 minutes. Reduces average query time from 500ms to 50ms.

Benchmark: 1000 queries/sec sustained load
- Before: 500ms avg, 2s p99
- After: 50ms avg, 200ms p99
```

### Pattern 5: Dependency Update

```
build: upgrade React from 17.0 to 18.2

Update to React 18 for improved concurrent rendering.
All components tested and compatible with new version.

Related-To: #456 (TypeScript types update)
```

## Type-Specific Guidelines

### `feat:` - Features

**Use for:**

- New functionality
- New API endpoints
- New user-facing capabilities
- New configuration options

**Don't use for:**

- Bug fixes (use `fix:`)
- Code reorganization (use `refactor:`)
- Performance improvements (use `perf:`)

**Example:**

```
feat(notifications): add email notification system

Implement email notifications for:
- Order confirmations
- Password resets
- Account activity alerts

Uses SendGrid API for delivery.

Closes: #89
```

### `fix:` - Bug Fixes

**Use for:**

- Correcting broken functionality
- Resolving errors or exceptions
- Fixing incorrect behavior
- Patching security vulnerabilities

**Don't use for:**

- Improvements to working code (use `refactor:` or `perf:`)
- Adding functionality (use `feat:`)

**Example:**

```
fix(auth): resolve JWT token expiration issue

Tokens were expiring 1 hour early due to timezone
conversion error. Now correctly handle UTC timestamps.

Fixes: #123
```

### `docs:` - Documentation

**Use for:**

- README updates
- API documentation
- Code comments
- Inline documentation
- Migration guides

**Don't use for:**

- Code changes (even if they affect documentation)

**Example:**

```
docs(api): add authentication examples

Add code examples for:
- OAuth2 flow
- JWT token refresh
- API key authentication

Closes: #67
```

### `refactor:` - Refactoring

**Use for:**

- Code reorganization
- Extracting functions/classes
- Renaming for clarity
- Simplifying logic

**Must maintain:**

- Same functionality
- Same API contracts
- Same behavior

**Example:**

```
refactor(user-service): extract validation logic

Move user input validation to separate validator class.
No functional changes, improved testability.
```

### `test:` - Tests

**Use for:**

- Adding missing tests
- Updating existing tests
- Fixing broken tests
- Improving test coverage

**Don't include:**

- Production code changes (separate commit)

**Example:**

```
test(payment): add integration tests for refunds

Add comprehensive test suite covering:
- Successful refunds
- Partial refunds
- Failed refund scenarios
- Webhook handling

Coverage: 85% → 94%
```

## Troubleshooting

### Issue: Not sure if it's `feat` or `fix`

**Solution:** Ask: "Did this functionality work before?"

- If NO (new functionality) → `feat:`
- If YES (broken functionality) → `fix:`

### Issue: Not sure if it's `refactor` or `perf`

**Solution:** Ask: "Is performance measurably improved?"

- If YES (measurable improvement) → `perf:`
- If NO (same performance, better code) → `refactor:`

### Issue: Commit touches multiple areas

**Solution:**

- **Preferred:** Split into multiple commits, one per area
- **Alternative:** Use most significant area as scope
- **Last resort:** Omit scope if equally weighted

### Issue: Everything feels like `chore`

**Solution:** Avoid overusing `chore`. Ask:

- Does it add features? → `feat:`
- Does it fix bugs? → `fix:`
- Does it change docs? → `docs:`
- Does it change build? → `build:`
- Does it change CI? → `ci:`

Use `chore:` only for maintenance tasks that don't fit above categories.

### Issue: Breaking change in a fix

**Solution:** Use `fix!:` with `BREAKING CHANGE:` footer:

```
fix(api)!: correct response format for user endpoint

BREAKING CHANGE: Response now returns 'userId' instead
of 'user_id' to match API conventions.
```

## Team Integration

### Setting Up Automated Validation

**Git Hook (.git/hooks/commit-msg):**

```bash
#!/bin/bash
pattern="^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\(.+\))?!?: .+"

if ! grep -qE "$pattern" "$1"; then
  echo "❌ Commit message does not follow Conventional Commits"
  echo "Expected: <type>[scope]: <description>"
  echo "Example: feat(auth): add OAuth2 integration"
  exit 1
fi
```

**CI Validation (GitHub Actions):**

```yaml
name: Validate Commits
on: [pull_request]
jobs:
  commitlint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
        with:
          fetch-depth: 0
      - uses: commitlint-bot/commitlint-action@v2.0.0
```

### Tooling Recommendations

- **commitlint** - Validate commit messages
- **husky** - Manage Git hooks
- **commitizen** - Interactive commit creation
- **conventional-changelog** - Generate CHANGELOGs
- **semantic-release** - Automated versioning and releases

## Quick Reference Card

```
Format: <type>[scope]: <description>

Types:
  feat:     New feature (MINOR bump)
  fix:      Bug fix (PATCH bump)
  docs:     Documentation only
  style:    Code formatting only
  refactor: Code change, same behavior
  perf:     Performance improvement
  test:     Test changes
  build:    Build system changes
  ci:       CI/CD changes
  chore:    Maintenance
  revert:   Revert previous commit

Breaking Changes:
  type!:    Add ! for breaking change
  footer:   BREAKING CHANGE: description

Rules:
  - Imperative mood: "add" not "added"
  - Lowercase description
  - No trailing period
  - Max 72 characters for description
  - Wrap body at 72 characters
  - Blank line between description and body
  - Footers format: Token: value

Examples:
  feat: add user export
  fix(auth): resolve token expiration
  docs: update API guide
  feat!: remove deprecated endpoint
```

## Related Resources

- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [commitlint Documentation](https://commitlint.js.org/)
