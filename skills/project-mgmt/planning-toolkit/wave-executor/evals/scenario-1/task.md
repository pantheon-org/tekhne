# Task: Execute Wave 0 of an auth migration plan

The wave document for an auth migration lives at `.context/plans/auth-migration.md`.
Execute Wave 0.

## Wave document excerpt

```markdown
# Auth Migration

**Ticket / ref**: PLAT-412
**Status**: Pending
**Assignee**: platform team

## Waves & Phases

### Wave 0 — Bootstrap (parallel)

> Gate: None — start immediately.

| Phase | Focus | Tasks | Status | Model |
|-------|-------|-------|--------|-------|
| `feat/schema` | Add new auth schema columns | 2 | Pending | standard |
| `feat/tokens` | Implement token generation service | 3 | Pending | standard |
| `feat/session` | Scaffold session store interface | 2 | Pending | fast |

Verification:
- [ ] All three branches merged to `main`
- [ ] `tsc --noEmit` exits 0 on `main`
- [ ] No references to the old auth schema remain in migration files
```
