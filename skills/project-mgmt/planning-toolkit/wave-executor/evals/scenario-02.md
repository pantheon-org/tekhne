# Scenario 02: Execute a Wave with Mixed Model Tiers

## User Prompt

The wave document at `.context/plans/data-pipeline.md` has a single wave with phases spanning all three model tiers, plus one phase with no Model value specified.

Execute Wave 1.

## Wave document excerpt

```markdown
# Data Pipeline Overhaul

**Status**: Pending

## Waves & Phases

### Wave 1 — Analysis and cleanup (parallel)

> Gate: None — start immediately.

| Phase | Focus | Tasks | Status | Model |
|-------|-------|-------|--------|-------|
| `feat/delete-scripts` | Delete deprecated sync scripts | 1 | Pending | fast |
| `feat/cross-analysis` | Write cross-cutting analysis from 8 source files | 1 | Pending | smart |
| `feat/update-docs` | Update API reference tables | 2 | Pending | standard |
| `feat/index-rebuild` | Run index rebuild and verify output | 1 | Pending | |

Verification:
- [ ] All four branches merged to `main`
- [ ] No references to deprecated scripts remain
```

## Expected Behavior

1. Resolve "fast" tier to `haiku` for the `feat/delete-scripts` Agent call
2. Resolve "smart" tier to `opus` for the `feat/cross-analysis` Agent call
3. Resolve "standard" tier to `sonnet` for the `feat/update-docs` Agent call
4. Apply the default "standard" → `sonnet` for the `feat/index-rebuild` Agent call (blank Model cell)
5. Make all four Agent calls in a single response (parallel spawning)
6. Do not use a model tier that differs from what is specified in the wave document for any agent

## Success Criteria

- **haiku-for-delete-scripts**: The Agent call for `feat/delete-scripts` uses model: 'haiku'
- **opus-for-cross-analysis**: The Agent call for `feat/cross-analysis` uses model: 'opus'
- **sonnet-for-update-docs**: The Agent call for `feat/update-docs` uses model: 'sonnet'
- **sonnet-default-for-index-rebuild**: The Agent call for `feat/index-rebuild` uses model: 'sonnet' (the default when Model cell is blank)
- **all-four-agents-spawned-in-parallel**: All four Agent calls appear in a single response (parallel spawning)
- **no-model-override-or-substitution**: No agent is given a model tier that differs from what is specified in the wave document

## Failure Conditions

- Uses the wrong model for `feat/delete-scripts` (should be `haiku`, not `sonnet` or `opus`)
- Uses the wrong model for `feat/cross-analysis` (should be `opus`, not `sonnet` or `haiku`)
- Uses the wrong model for `feat/update-docs` (should be `sonnet`, not `haiku` or `opus`)
- Uses a non-default model for `feat/index-rebuild` when the cell is blank (default should be `sonnet`)
- Spawns agents across multiple sequential messages instead of in a single parallel response
- Overrides or substitutes a model tier specified in the wave document
