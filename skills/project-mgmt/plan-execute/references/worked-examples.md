# Worked Execution-Checklist Examples

Full worked examples of execution checklists (extension-free imports, validation, and a generic phase). The core workflow is in `SKILL.md`; load this for a complete example to copy.

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

### 2. Per-Task Execution

For each task, in order:

#### 2a. Work the task

Implement the task per its instructions. Create/modify/delete files as specified.

#### 2b. Quick validation after atomic changes

After every 1-3 file operations, run the quick gate (typically typecheck):

```
### P{NN}T{NN} — Quick Validation
- Command: {typecheck}
- Actual: {paste output}
- Status: {VALIDATED if pass, IN_PROGRESS if fail}
```

**If it fails:** Stop. Fix the issue. Re-run the command. Record the new output.
Do not proceed to the next file operation until this passes.

#### 2c. Task-level validation

After all file operations for the task are complete, run the full gate suite:

```
### P{NN}T{NN} — Task Validation
- Command: {typecheck} && {build} && {test} && {lint}
- Actual: {paste full output}
- Status: {VALIDATED or IN_PROGRESS}
```

**If it fails:** Stop. The task is not done. Fix the issue. Re-run. Update the
checklist with the new output. Do not proceed to the next task until this passes.

#### 2d. Structural audit (if applicable)

If the task involves mechanical changes (import path changes, file splitting,
naming conventions), run the structural audit and record the output:

```
### P{NN}T{NN} — Structural Audit
- Command: {audit command}
- Expected: {zero / empty / nothing}
- Actual: {paste output}
- Status: {VALIDATED if matches expected, IN_PROGRESS if not}
```

**If it fails:** The task is not done. The mechanical standard is not met. Go
back and fix the files. Re-run the audit. Record the new output.

#### 2e. Update the checklist

After the task passes all validations, update its checklist entry:

```markdown
### P01T01 — {Task Name}
- [x] Task: {description}
- [x] Files created: {list}
- [x] Files modified: {list}
- [x] Files deleted: {list}
- [x] Quick gate after atomic changes
  - Command: {command}
  - Actual: {output}
  - Status: VALIDATED
- [x] Full gate suite at task end
  - Command: {command}
  - Actual: {output}
  - Status: VALIDATED
- [x] Structural audit
  - Command: {command}
  - Expected: {expected}
  - Actual: {output}
  - Status: VALIDATED
- [x] Status: VALIDATED
```

**RULE:** You may only mark an item with `[x]` after recording the Actual output.
An `[x]` without Actual output is a lie.

### 3. Per-Phase Validation

After all tasks in a phase are validated, validate the phase as a whole:

#### 3a. Phase gate suite

Run the full gate suite for the entire phase:

```
## Phase 01 — Validation
- Command: {typecheck} && {build} && {test} && {lint}
- Actual: {paste full output}
- Status: {VALIDATED or IN_PROGRESS}
```

**If it fails:** One or more tasks have an interaction bug. Stop. Diagnose which
task caused it. Fix it. Re-validate the affected task(s). Re-run the phase suite.

#### 3b. Domain invariant

Run the phase-specific domain invariant check (e.g., byte-identical output,
benchmark comparison, integration test):

```
- Domain invariant: {description}
- Command: {command}
- Expected: {expected result}
- Actual: {output}
- Status: {VALIDATED or IN_PROGRESS}
```

**If it fails:** The phase broke something the generic gates cannot detect.
Stop. Fix it. Re-run.

#### 3c. Regression diff

Compare the phase result to the baseline:

```
- Baseline tests: {X pass, Y fail}
- Phase tests: {X pass, Y fail}
- New failures: {count}
- Status: {VALIDATED if no new failures, IN_PROGRESS if any}
```

**If new failures exist:** You introduced a regression. Stop. Fix it. Re-run.

#### 3d. Commit the phase

Commit with evidence in the message:

```bash
git add -A
git commit -m "feat(scope): phase N — {phase-name}

{one-line summary}

Checklist evidence:
- Typecheck: {paste result line}
- Build: {paste result line}
- Tests: {paste result line}
- Lint: {paste result line}
- Domain invariant: {paste result line}
- Regression: no new failures (or: fixed N pre-existing)"
```

Record the commit hash in the checklist.

#### 3e. Phase checklist update

```markdown
## Phase 01 — {Phase Name}
- [x] All tasks validated
- [x] Phase gate suite passes
  - Command:
  - Actual:
- [x] Domain invariant holds
  - Command:
  - Actual:
- [x] Regression diff: no new failures
- [x] Committed as: {hash}
- [x] Status: VALIDATED
```

### 4. Between-Phase Rules

- **If Phase N+1 depends on Phase N:** Phase N must be VALIDATED before starting
  Phase N+1. No exceptions.
- **If a phase is independent:** It may be delegated to a subagent in parallel,
  but the subagent must produce its own checklist evidence.

### 5. Plan Divergence Handling

If reality diverges from the plan during implementation:

1. **Stop.** Do not continue.
2. **Document the divergence** in the checklist under a "Divergence" section:
   ```markdown
   ## Divergence — Phase 02, Task 03
   - Expected: {what the plan said}
   - Actual: {what reality required}
   - Reason: {why}
   ```
3. **Amend the plan** using `plan-create` amendment pattern.
4. **Re-review** if the amendment is significant (new tasks, changed gates,
   altered scope).
5. **Update the checklist** with the amended tasks.
6. **Continue** from where you stopped.

**Never** implement something not in the plan (or its amendment) without updating
the checklist and plan first.

### 6. Final Verification

After all phases are validated and committed:

#### 6a. Full gate suite

Run all gates one final time on the complete branch:

```
