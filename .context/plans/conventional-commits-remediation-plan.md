---
plan_date: 2026-02-23
skill_name: conventional-commits
source_audit: .context/audits/conventional-commits-audit-2026-02-22.md
---

# Remediation Plan: conventional-commits

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 87/120 | 100/120 |
| **Grade** | C | B |
| **Priority** | High | - |
| **Effort** | Medium (M) | - |

**Focus Areas**: Progressive disclosure (D5), Anti-pattern quality (D3), Pattern recognition (D7)

## Critical Issues to Address

| Issue | Severity | Dimension | Impact |
| --- | --- | --- | --- |
| Very large SKILL.md (636 lines, 0 refs) | Critical | D5 (5/15) | Unmaintainable structure |
| Weak anti-pattern examples | High | D3 (8/15) | Commit convention violations |
| Moderate pattern recognition | Medium | D7 (8/10) | May miss some trigger phrases |

## Detailed Remediation Steps

### Phase 1: Progressive Disclosure (D5) - Priority: Critical

**Target**: Increase from 5/15 to 13/15 (+8 points)

#### Step 1.1: Create reference structure

**File**: `skills/conventional-commits/`

```
skills/conventional-commits/
├── SKILL.md (hub, ~120 lines)
├── references/
│   ├── commit-types.md
│   ├── scope-conventions.md
│   ├── breaking-changes.md
│   └── tooling-integration.md
├── templates/
│   └── commit-message.yaml
```

#### Step 1.2: Extract commit types to reference

**File**: `skills/conventional-commits/references/commit-types.md`

Move detailed type definitions:

```markdown
# Commit Types Reference

## Standard Types

| Type | Description | Example |
| --- | --- | --- |
| `feat` | New feature | `feat(auth): add OAuth2 login` |
| `fix` | Bug fix | `fix(api): handle null response` |
| `docs` | Documentation | `docs(readme): update install steps` |
| `style` | Formatting | `style: fix indentation` |
| `refactor` | Code refactoring | `refactor(core): extract validation` |
| `perf` | Performance | `perf(query): optimize index scan` |
| `test` | Tests | `test(auth): add unit tests` |
| `chore` | Maintenance | `chore(deps): update dependencies` |
| `ci` | CI/CD | `ci: add Windows build` |
| `build` | Build system | `build: upgrade webpack` |

## Extended Types

Project-specific types used in this repository:

[Continue with repository-specific types...]
```

#### Step 1.3: Extract scope conventions

**File**: `skills/conventional-commits/references/scope-conventions.md`

Move scope definitions and project-specific scopes.

#### Step 1.4: Extract breaking changes guidance

**File**: `skills/conventional-commits/references/breaking-changes.md`

Move BREAKING CHANGE format and examples.

#### Step 1.5: Extract tooling integration

**File**: `skills/conventional-commits/references/tooling-integration.md`

Move commitlint, semantic-release, and changelog generation setup.

#### Step 1.6: Rewrite SKILL.md as hub

**File**: `skills/conventional-commits/SKILL.md`

````markdown
---
name: conventional-commits
description: "[from Phase 3]"
---

# Conventional Commits

Structured commit messages for automated changelogs and semantic versioning.

## Use When

- "Write commit message"
- "Format commit for changelog"
- "Set up commitlint"
- "Configure semantic-release"
- "Breaking change commit format"

## Quick Reference

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Common Types

| Type | Use When |
| --- | --- | --- |
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation |
| `refactor` | Code restructuring |
| `test` | Adding tests |

See [Commit Types](references/commit-types.md) for complete reference.

### Examples

```bash
feat(auth): add OAuth2 login support

fix(api): handle null response in user endpoint

docs(readme): update installation steps

feat(core)!: remove deprecated API

BREAKING CHANGE: The old API is no longer supported
```

## Anti-Patterns

### NEVER: Use vague descriptions

BAD: `fix: fixed it`
GOOD: `fix(api): return 404 for missing user`

[Additional anti-patterns from Phase 2...]

## Breaking Changes

Use `!` after type/scope or `BREAKING CHANGE:` in footer.

See [Breaking Changes](references/breaking-changes.md) for details.

## Tooling

- commitlint: Validate commit format
- semantic-release: Automated releases
- standard-version: Changelog generation

See [Tooling Integration](references/tooling-integration.md) for setup.
````

---

### Phase 2: Anti-Pattern Quality (D3) - Priority: High

**Target**: Increase from 8/15 to 14/15 (+6 points)

#### Step 2.1: Add explicit anti-patterns

**File**: `skills/conventional-commits/SKILL.md`

````markdown
## Anti-Patterns

### NEVER: Use vague or generic descriptions

WHY: Changelogs become meaningless; version automation fails.

BAD:
```
fix: fixed it
feat: added stuff
refactor: cleanup
```

GOOD:
```
fix(api): return 404 for missing user instead of 500
feat(auth): add OAuth2 login with Google provider
refactor(core): extract validation into separate module
```

### NEVER: Mix multiple changes in one commit

WHY: Atomic commits enable accurate versioning and easy rollbacks.

BAD:
```
feat: add login and fix logout bug and update docs
```

GOOD:
```
feat(auth): add OAuth2 login
fix(auth): resolve session timeout issue
docs(auth): update login documentation
```

### NEVER: Omit scope for code changes

WHY: Scopes enable targeted changelogs and impact analysis.

BAD:
```
fix: handle null response
```

GOOD:
```
fix(api): handle null response in user endpoint
```

### NEVER: Use past tense in description

WHY: Conventional commits use imperative mood (like git itself).

BAD:
```
feat(auth): added OAuth2 login
```

GOOD:
```
feat(auth): add OAuth2 login
```

### NEVER: Capitalize first letter of description

WHY: Maintains consistency with git conventions.

BAD:
```
feat(auth): Add OAuth2 login
```

GOOD:
```
feat(auth): add OAuth2 login
```

### NEVER: End description with period

WHY: Git commit messages typically don't end with punctuation.

BAD:
```
feat(auth): add OAuth2 login.
```

GOOD:
```
feat(auth): add OAuth2 login
```
````

---

### Phase 3: Pattern Recognition (D7) - Priority: Medium

**Target**: Increase from 8/10 to 10/10 (+2 points)

#### Step 3.1: Expand frontmatter description

```yaml
---
name: conventional-commits
description: |
  Write structured commit messages following Conventional Commits specification.
  Use when: writing commit messages, formatting commits for changelogs,
  setting up commitlint, configuring semantic-release, handling breaking changes,
  or documenting API changes in commits.
  
  Keywords: conventional commits, commit message, commitlint, semantic-release,
  changelog, breaking change, feat, fix, docs, semver, versioning, git commit
---
```

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh conventional-commits --json
bunx markdownlint-cli2 "skills/conventional-commits/**/*.md"
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D5 Progressive Disclosure | Score >= 13/15 |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| D7 Pattern Recognition | Score >= 10/10 |
| SKILL.md line count | <= 150 lines |
| References created | >= 3 files |
| Overall Score | >= 100/120 (B) |

## Estimated Effort

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Disclosure | M | 2 hours |
| Phase 2: Anti-patterns | S | 45 min |
| Phase 3: Triggers | S | 15 min |
| **Total** | **M** | **3 hours** |

## Dependencies

- None (self-contained skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/conventional-commits/
```

## Notes

This skill has a large SKILL.md (636 lines) that requires significant extraction work. Prioritize Phase 1 to achieve maintainability before enhancing content quality.
