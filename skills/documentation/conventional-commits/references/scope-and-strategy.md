# Scope Naming and Commit Strategy

## Scope Naming Conventions

Scopes appear in changelogs grouped by name. Inconsistent scopes fragment the changelog into one-entry groups that defeat the purpose.

**Choose scope at the module/package level, not the file level:**

| Codebase structure | Good scope | Bad scope |
|--------------------|-----------|-----------|
| `src/auth/` | `auth` | `authService`, `tokenHelper` |
| `packages/ui/` | `ui` | `button`, `modal` |
| `apps/api/` | `api` | `getUserById`, `routes` |

**Cross-cutting changes:** omit scope entirely rather than inventing a catch-all like `core` or `global`. The absence of scope is itself meaningful.

**Team alignment:** define a fixed scope list in `commitlint.config.mjs` to enforce consistency:

```js
export default {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'scope-enum': [2, 'always', ['auth', 'checkout', 'api', 'ui', 'db', 'ci']],
  },
};
```

## Multi-Commit Strategy (Atomic Commits)

A commit should be the smallest unit of meaningful, independently revertable change. Apply these rules when splitting work:

**One concern, one commit.** If a feature requires a schema migration and an API change, commit them separately — the migration can be rolled back without touching the API.

**Refactor before feature.** When a feature requires restructuring existing code, split into:
1. `refactor(<scope>): <what you cleaned up>` (no behaviour change)
2. `feat(<scope>): <new capability>`

This makes code review easier and keeps `refactor` commits revert-safe.

**Fix + test together.** A bug fix commit may include the regression test — the test is not a separate concern, it is proof of the fix.

**Never bundle unrelated changes to "save commits."** One `fix` + one `docs` update = two commits. The changelog grouping depends on it.

## Revert Commits

```
revert: feat(auth): add OAuth2 login support

This reverts commit a1b2c3d4.

Reason: OAuth2 provider contract changed; re-implementing in #456.
```

Rules:
- Header must reference the original commit header verbatim after `revert:`
- Include the original SHA in the body
- Explain why — changelog consumers need this context
