# Scenario 01: Execute Wave 0 of an Auth Migration Plan

## User Prompt

The wave document for an auth migration lives at `.context/plans/auth-migration.md`. Execute Wave 0.

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

## Expected Behavior

1. Spawn three Agent tool calls, one for `feat/schema`, one for `feat/tokens`, and one for `feat/session`
2. Make all three Agent calls in a single response (parallel), not across separate sequential messages
3. Set worktree isolation for each Agent call (e.g. `isolation: 'worktree'` or equivalent parameter)
4. Resolve the "standard" tier to `sonnet` for the `feat/schema` agent call
5. Resolve the "standard" tier to `sonnet` for the `feat/tokens` agent call
6. Resolve the "fast" tier to `haiku` for the `feat/session` agent call
7. Include the Focus cell text verbatim in each agent prompt (e.g. "Add new auth schema columns") — not a reworded summary
8. After spawning agents, do not immediately proceed to Wave 1 — wait for agent results and then stop to ask for merge confirmation

## Success Criteria

- **all-three-agents-spawned**: Three Agent tool calls were made, one for `feat/schema`, one for `feat/tokens`, and one for `feat/session`
- **agents-spawned-in-single-message**: All three Agent calls appear in a single response (parallel), not across separate sequential messages
- **worktree-isolation-set**: Each Agent call sets isolation: 'worktree' (or equivalent parameter indicating branch isolation)
- **correct-model-feat-schema**: The Agent call for `feat/schema` uses model: 'sonnet'
- **correct-model-feat-tokens**: The Agent call for `feat/tokens` uses model: 'sonnet'
- **correct-model-feat-session**: The Agent call for `feat/session` uses model: 'haiku'
- **task-description-not-paraphrased**: Each agent prompt includes the Focus cell text verbatim, not a reworded summary
- **stops-after-agents-complete**: After spawning agents, the skill does not immediately proceed to Wave 1 — it waits for agent results and then stops to ask for merge confirmation

## Failure Conditions

- Spawns fewer than three agents, or spawns them across multiple sequential messages
- Does not set worktree isolation for agent calls
- Uses the wrong model for `feat/schema` or `feat/tokens` (should be `sonnet`, not `haiku` or `opus`)
- Uses the wrong model for `feat/session` (should be `haiku`, not `sonnet` or `opus`)
- Paraphrases or rewrites the Focus cell text instead of using it verbatim
- Immediately proceeds to Wave 1 after spawning Wave 0 agents without waiting for results and merge confirmation
