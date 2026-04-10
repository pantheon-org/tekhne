# Task: Execute a wave with mixed model tiers

The wave document at `.context/plans/data-pipeline.md` has a single wave with phases
spanning all three model tiers, plus one phase with no Model value specified.

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
