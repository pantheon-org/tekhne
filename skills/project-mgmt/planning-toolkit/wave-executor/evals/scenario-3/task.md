# Task: Wave 0 agents have finished — what happens next?

You have been executing the plan at `.context/plans/infra-cleanup.md`.
Wave 0 agents for all three branches have returned their results.
The user is asking what comes next.

## Wave document excerpt

```markdown
# Infra Cleanup

**Status**: In Progress

## Waves & Phases

### Wave 0 — Remove legacy resources (parallel) — IN PROGRESS

> Gate: None — start immediately.

| Phase | Focus | Tasks | Status | Model |
|-------|-------|-------|--------|-------|
| `feat/remove-terraform` | Delete deprecated Terraform modules | 3 | In Progress | sonnet |
| `feat/update-consumers` | Update all consumers of removed modules | 2 | In Progress | sonnet |
| `feat/cleanup-vars` | Remove orphaned Terraform variables | 1 | In Progress | haiku |

Verification:
- [ ] All three branches merged to `main`
- [ ] `terraform plan` exits 0 on `main` with no unexpected changes
- [ ] No references to `legacy_*` remain in any `.tf` file

### Wave 1 — Documentation and audit (sequential)

> Gate: Wave 0 verified ✓

| Phase | Focus | Tasks | Status | Model |
|-------|-------|-------|--------|-------|
| `feat/update-docs` | Update runbook and architecture diagrams | 2 | Pending | sonnet |

Verification:
- [ ] Runbook updated; diagrams reflect removed modules
```

## Agent results

All three Wave 0 agents have completed and reported their work:
- `feat/remove-terraform`: Deleted 4 deprecated modules; changes committed.
- `feat/update-consumers`: Updated 6 consumer references; changes committed.
- `feat/cleanup-vars`: Removed 11 orphaned variables; changes committed.

What should happen next?
