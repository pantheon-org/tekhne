# Scenario 03: Wave 0 Agents Have Finished — What Happens Next?

## User Prompt

You have been executing the plan at `.context/plans/infra-cleanup.md`. Wave 0 agents for all three branches have returned their results. The user is asking what comes next.

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

## Expected Behavior

1. Do NOT spawn a Wave 1 Agent call in the response — explicitly stop before doing so
2. Ask the user to merge the three Wave 0 branches (`feat/remove-terraform`, `feat/update-consumers`, `feat/cleanup-vars`) to main before proceeding
3. Include the Wave 0 Verification checklist items verbatim so the user knows what to run before confirming
4. Explicitly state it will wait for user confirmation (e.g. "reply verified when done") before starting Wave 1
5. List all three branch names in the merge request

## Success Criteria

- **does-not-spawn-wave-1-immediately**: The agent does NOT spawn a Wave 1 Agent call in its response — it explicitly stops before doing so
- **asks-user-to-merge-branches**: The response asks the user to merge the three Wave 0 branches to main before proceeding
- **verification-checklist-provided**: The response includes the Wave 0 Verification checklist items verbatim so the user knows what to run before confirming
- **waits-for-user-confirmation**: The response explicitly states it will wait for user confirmation before starting Wave 1
- **lists-all-three-branches**: All three branch names (`feat/remove-terraform`, `feat/update-consumers`, `feat/cleanup-vars`) are listed in the merge request

## Failure Conditions

- Spawns a Wave 1 Agent call without waiting for merge confirmation
- Does not ask the user to merge the Wave 0 branches
- Does not include the Verification checklist items in the response
- Does not explicitly state it will wait for user confirmation
- Omits one or more branch names from the merge request
