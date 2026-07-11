# Plan Execute — Checklist Templates and Worked Example

Full templates and a worked example referenced from `SKILL.md`. Keep the
`SKILL.md` procedure lean; copy these when building a real `CHECKLIST.md`.

## CHECKLIST.md File Template

Create `.context/plans/<plan-slug>/CHECKLIST.md` with all tasks from all
phases, in order, using this template:

```markdown
# Execution Checklist: <Plan Title>

## Baseline
- [ ] Typecheck — <command>
- [ ] Build — <command>
- [ ] Tests — <command>
- [ ] Lint — <command>

## Phase 01 — <Phase Name>
### P01T01 — <Task Name>
- [ ] Task: <description from task file>
- [ ] Files created: <list>
- [ ] Files modified: <list>
- [ ] Files deleted: <list>
- [ ] Quick gate (typecheck) after atomic changes
  - Command:
  - Actual:
- [ ] Full gate suite at task end
  - Command:
  - Actual:
- [ ] Structural audit (if applicable)
  - Command:
  - Expected:
  - Actual:
- [ ] Status:

### P01T02 — <Task Name>
...

## Phase 02 — <Phase Name>
...

## Phase-Level Validation
### Phase 01
- [ ] All tasks in phase validated
- [ ] Phase gate suite passes
- [ ] Domain invariant holds
- [ ] Regression diff vs baseline: no new failures
- [ ] Committed as: <commit hash>
```

## Example: Real Checklist Excerpt

From Phase 01 of a standards compliance remediation:

```markdown
## Phase 01 — Extension-Free Imports
### P01T01 — Strip `/index` from consumer imports
- [x] Task: Update 26 consumer imports to remove `/index` suffix
- [x] Files modified:
  - web/src/composables/store/apply-query.ts
  - web/src/composables/store/load-data.ts
  - web/src/composables/store/set-ui-locale.ts
  - web/src/composables/store/use-data-state.ts
  - web/src/composables/store/use-query-sync.ts
  - web/src/composables/store/use-ui-state.ts
- [x] Quick gate after atomic changes
  - Command: bun run typecheck
  - Actual: vue-tsc --noEmit
    > 0 errors, 0 warnings
  - Status: VALIDATED
- [x] Structural audit
  - Command: grep -rn "from '.*/index'" web/src --include='*.ts' --include='*.vue' | grep -v 'index.ts' | grep -v 'export.*from' | wc -l
  - Expected: 0
  - Actual: 0
  - Status: VALIDATED
- [x] Full gate suite at task end
  - Command: bun run typecheck && bun run build && bun run test
  - Actual:
    Typecheck: 0 errors
    Build: dist/ generated
    Test: 24/27 suites, 190 tests pass
  - Status: VALIDATED
- [x] Status: VALIDATED
```

## Final Report Template

Write the final report in the checklist file using this template:

```markdown
# Final Report

## Summary
- Plan: <title>
- Branch: <branch name>
- Phases completed: <N>/<N>
- Tasks completed: <M>/<M>
- Commits: <list of hashes>

## Evidence
- Baseline: <link to baseline section>
- Per-phase evidence: <links>
- Final gate suite: <output>
- Global audit: <output>

## Regressions
- Pre-existing failures: <count> (unchanged / fixed / new)
- New failures introduced: <count>
- If any new failures: **PLAN IS NOT DONE**

## Divergences
- <list of any plan amendments made during execution>

## Checklist Integrity
- All items checked: YES / NO
- All items have proof: YES / NO
- If NO: **PLAN IS NOT DONE**
```
