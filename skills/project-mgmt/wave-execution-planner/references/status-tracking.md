# Status Tracking

How to update a wave document as work progresses (Mode B).

## Status vocabulary

### Task-level (checkbox format)

```markdown
- [ ] T1.1 — description     ← Pending
- [~] T1.2 — description     ← In Progress  (use [~] or [/] per project convention)
- [x] T1.3 — description     ← Done
```

### Phase-level (table format)

| Value | Meaning |
|-------|---------|
| `Pending` | Not started |
| `In Progress` | Agent is actively working on this phase |
| `Done` | All tasks complete, verification passed |
| `Blocked` | Blocked by an unresolved issue (add a note) |

### Wave-level (heading badge)

Append a status tag to the wave heading:

```markdown
### Wave 1 — Diagnostics & Design (parallel) ✅ done
### Wave 2 — Fix & Build (parallel) 🔄 in progress
### Wave 3 — Coverage & Validation (parallel) ⏳ pending
```

Or plain ASCII equivalents (preferred for agent-authored docs):

```markdown
### Wave 1 — Diagnostics & Design (parallel) — DONE
### Wave 2 — Fix & Build (parallel) — IN PROGRESS
### Wave 3 — Coverage & Validation (parallel) — PENDING
```

### Document-level header

```markdown
**Status**: In Progress   ← update when first wave starts
**Status**: Complete      ← update when all waves done and final checks pass
**Status**: Blocked       ← update with a reason note when stuck
```

## Mode B update protocol

Run after a wave's work is committed and merged:

1. **Run verification commands** listed in the completed wave's `Verification:` checklist.
2. **For each passing check**: tick the checkbox `- [ ]` → `- [x]`.
3. **For each phase row**: update `Status` cell from `In Progress` → `Done`.
4. **When all verifications pass**: mark the wave heading `— DONE`.
5. **Update next wave**: change its phases from `Pending` → `In Progress` if agents are starting.
6. **Update document header**: set `**Status**: In Progress` or `Complete` as appropriate.
7. **Save and report**: write the file, then report which wave is now unblocked.

## Verification gate before next wave

NEVER advance to wave N+1 if wave N has:
- Any failing verification check.
- Any phase still in `In Progress` state.
- Any open blocker note.

Example gate check protocol:

```markdown
**Before starting Wave 2:**
Run the following and confirm all pass:
- `vitest run` — all tests green
- `tsc --noEmit` — no type errors
- `git log --oneline main..HEAD` — all Wave 1 commits present on main
```

## Handling partial completion

If a wave is partially done (some tasks complete, some not):

1. Tick completed task checkboxes.
2. Set partially-done phase rows to `In Progress` (not `Pending`).
3. Leave wave heading as `— IN PROGRESS`.
4. Do NOT advance to the next wave.

## Handling blocked tasks

If a task is blocked:

1. Add a `> BLOCKED:` note immediately after the task entry.
2. Set phase status to `Blocked`.
3. Set wave heading to `— BLOCKED`.
4. Surface to the user with: what is blocked, why, and the suggested unblocking action.

```markdown
- [~] T2.3 — Fix e2e polling
  > BLOCKED: Playwright browser download fails in CI — pending infrastructure ticket #4521.
```

## Worktree cleanup after wave merge

After a parallel wave's branches are all merged:

```bash
# List worktrees
git worktree list

# Remove each worktree that was used for this wave
git worktree remove <path>

# Delete merged branches
git branch -d <branch-name>
```

Add a `Cleanup:` subsection to the wave if worktrees were used:

```markdown
Cleanup (after merge):
- [ ] Remove worktrees: `git worktree remove <path>`
- [ ] Delete branches: `git branch -d <branch>`
```

## Example — updated wave document snippet

```markdown
### Wave 1 — Extract shared lib (parallel) — DONE

> Gate: None — start immediately.

- [x] T1.1 — `scripts/lib/http.ts` — branch `refactor/scripts-lib-http`
- [x] T1.2 — `scripts/lib/dates.ts` — branch `refactor/scripts-lib-dates`
- [x] T1.3 — `scripts/lib/wikidata.ts` — branch `refactor/scripts-lib-wikidata`
- [x] T1.4 — `scripts/lib/paths.ts` — branch `refactor/scripts-lib-paths`

Verification:
- [x] All merged; `main` CI green
- [x] No direct imports of old paths remain

Cleanup:
- [x] Worktrees removed
- [x] Branches deleted

### Wave 2 — Scaffold CLI (sequential) — IN PROGRESS

> Gate: Wave 1 verified ✓

- [~] T2.1 — Install Cliffy + scaffold `scripts/cli.ts` — branch `refactor/scripts-cli-scaffold`

Verification:
- [ ] `deno task cli --help` exits 0
- [ ] No old script imports remain in `cli.ts`
```
