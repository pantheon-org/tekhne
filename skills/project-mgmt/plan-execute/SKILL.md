---
name: plan-execute
description: >
  Execute an implementation plan with rigorous checklist-driven verification.
  Every task is pre-flighted, worked, validated with proof, and checked off.
  Every phase is validated as a whole. If proof is missing, the agent goes back.
  Honesty is non-negotiable — broken trust means the plan is not done.
  Triggers: 'implement this plan', 'execute the plan', 'start working on the plan',
  'do the plan', 'carry out the phases', 'run the plan', 'work through the plan',
  'carry out this plan', 'begin implementation'.
  Do NOT use for plans that have not been reviewed — use plan-review first.
---

# Plan Execute — Checklist-Driven Implementation with Honest Proof

Execute an implementation plan through its phases, validating every task with
verifiable proof before marking it done. If proof is missing or falsified, the
agent goes back and completes the work. No exceptions.

## Core Principle

**HONESTY IS NON-NEGOTIABLE.** A checked item without proof is a lie. A plan
with lies is not done. Broken trust means the implementation cannot be trusted
and must be redone.

## At a Glance

1. **Baseline** — run all gates before touching code.
2. **Checklist** — create `.context/plans/<slug>/CHECKLIST.md` with every task.
3. **Per-task** — work, quick-gate after 1-3 file ops, full-gate at task end.
4. **Per-phase** — all tasks done → phase gate suite → domain invariant → commit.
5. **Final** — full gates + global audit + checklist integrity audit → DONE.

If any step fails: stop, fix, re-run, record proof. No exceptions.

## Position in the Workflow

```
plan-create / implementation-planner → plan-review → plan-execute → commit / merge
     (design)              →   (audit)   →   (implement)  →  (ship)
```

## Prerequisites

- A `.context/plans/*.md` plan file with valid frontmatter, phases, and tasks
- The plan has been reviewed (status: ACTIVE or DRAFT with explicit go-ahead)
- `opencode.json` has `subAgents` configured for large-phase delegation
- A working branch exists or can be created per the plan's branch workflow

## Inputs

- **Plan path** — `.context/plans/<plan-slug>/README.md`
- **Phase files** — `.context/plans/<plan-slug>/phases/phase-*/README.md`
- **Task files** — `.context/plans/<plan-slug>/phases/phase-*/tasks/*.md`
- **Branch name** — from the plan or user
- **Gate commands** — from `AGENTS.md`, `package.json`, or CI config

## Outputs

- **CHECKLIST.md** — living checklist at `.context/plans/<plan-slug>/CHECKLIST.md`
- **Commits** — one per phase, with evidence in message body
- **Execution Report** — appended to the plan's `README.md`
- **Updated plan frontmatter** — `status: DONE` (only when fully validated)

## When to Use

- The user says "implement this plan", "execute the plan", "start working on it"
- A reviewed plan is ready and the user wants to begin coding
- A phase needs to be picked up and completed
- The user wants to "do the work" described in a `.context/plans/` file

## When NOT to Use

- The plan has not been reviewed — use `plan-review` first
- The user only wants a single file edited with no plan context — just edit it
- The work is exploratory or speculative — plans are for known scope
- The plan is stale (months old) — re-review it before executing

## The Checklist System

The checklist is a **living contract** between the agent and the plan. It lives
at `.context/plans/<plan-slug>/CHECKLIST.md` alongside the plan.

Every checklist entry must contain:
1. **Task** — what was supposed to be done
2. **Verification** — the exact command(s) run to prove it
3. **Expected** — what the output should be
4. **Actual** — the real output (copied verbatim)
5. **Status** — `PENDING`, `IN_PROGRESS`, `VALIDATED`, or `BLOCKED`

### Status Definitions

- **PENDING** — not started yet
- **IN_PROGRESS** — work begun, not yet validated
- **VALIDATED** — work done, verification run, proof recorded
- **BLOCKED** — cannot proceed, reason documented

**RULE:** An item marked `VALIDATED` without Actual output is a lie. The agent
must go back, run the verification, and record the output before proceeding.

## Workflow

### 1. Pre-Flight: Create the Checklist

#### 1a. Read the plan

Read the plan `README.md` and all phase `README.md` files. Extract every task
from every phase. Read all task files under `tasks/`.

#### 1b. Discover the gates

Find the project's verification commands from `AGENTS.md`, `package.json`, CI
config, or by asking the user. Record them:

```
Typecheck:  <command>  (e.g., vue-tsc --noEmit)
Build:      <command>  (e.g., npm run build)
Test:       <command>  (e.g., npm test)
Lint:       <command>  (e.g., npm run lint)
```

#### 1c. Run the Baseline

Run all gates against the current branch **before touching any code**. Record
the output verbatim in the checklist under a "Baseline" section.

```
## Baseline (before any changes)
- [ ] Typecheck
  - Command: <typecheck>
  - Expected: exit 0
  - Actual: <paste full output>
  - Status: VALIDATED

- [ ] Build
  - Command: <build>
  - Expected: exit 0
  - Actual: <paste full output>
  - Status: VALIDATED

- [ ] Tests
  - Command: <test>
  - Expected: all pass
  - Actual: <paste full output including counts>
  - Status: VALIDATED

- [ ] Lint
  - Command: <lint>
  - Expected: clean
  - Actual: <paste full output>
  - Status: VALIDATED
```

**CRITICAL:** If the baseline has failures, document them. They are your
regression reference. Any new failure after your changes is a regression you
introduced.

#### 1d. Write the Checklist File

Create `.context/plans/<plan-slug>/CHECKLIST.md` with all tasks from all
phases, in order. See `references/checklist-templates.md` for the full
`CHECKLIST.md` template and a worked example from a real phased run.

### 2. Per-Task Execution

For each task, in order:

#### 2a. Work the task

Implement the task per its instructions. Create/modify/delete files as specified.

#### 2b. Quick validation after atomic changes

After every 1-3 file operations, run the quick gate (typically typecheck):

```
### P<NN>T<NN> — Quick Validation
- Command: <typecheck>
- Actual: <paste output>
- Status: <VALIDATED if pass, IN_PROGRESS if fail>
```

**If it fails:** Stop. Fix the issue. Re-run the command. Record the new output.
Do not proceed to the next file operation until this passes.

#### 2c. Task-level validation

After all file operations for the task are complete, run the full gate suite:

```
### P<NN>T<NN> — Task Validation
- Command: <typecheck> && <build> && <test> && <lint>
- Actual: <paste full output>
- Status: <VALIDATED or IN_PROGRESS>
```

**If it fails:** Stop. The task is not done. Fix the issue. Re-run. Update the
checklist with the new output. Do not proceed to the next task until this passes.

#### 2d. Structural audit (if applicable)

If the task involves mechanical changes (import path changes, file splitting,
naming conventions), run the structural audit and record the output:

```
### P<NN>T<NN> — Structural Audit
- Command: <audit command>
- Expected: <zero / empty / nothing>
- Actual: <paste output>
- Status: <VALIDATED if matches expected, IN_PROGRESS if not>
```

**If it fails:** The task is not done. The mechanical standard is not met. Go
back and fix the files. Re-run the audit. Record the new output.

#### 2e. Update the checklist

After the task passes all validations, update its checklist entry:

```markdown
### P01T01 — <Task Name>
- [x] Task: <description>
- [x] Files created: <list>
- [x] Files modified: <list>
- [x] Files deleted: <list>
- [x] Quick gate after atomic changes
  - Command: <command>
  - Actual: <output>
  - Status: VALIDATED
- [x] Full gate suite at task end
  - Command: <command>
  - Actual: <output>
  - Status: VALIDATED
- [x] Structural audit
  - Command: <command>
  - Expected: <expected>
  - Actual: <output>
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
- Command: <typecheck> && <build> && <test> && <lint>
- Actual: <paste full output>
- Status: <VALIDATED or IN_PROGRESS>
```

**If it fails:** One or more tasks have an interaction bug. Stop. Diagnose which
task caused it. Fix it. Re-validate the affected task(s). Re-run the phase suite.

#### 3b. Domain invariant

Run the phase-specific domain invariant check (e.g., byte-identical output,
benchmark comparison, integration test):

```
- Domain invariant: <description>
- Command: <command>
- Expected: <expected result>
- Actual: <output>
- Status: <VALIDATED or IN_PROGRESS>
```

**If it fails:** The phase broke something the generic gates cannot detect.
Stop. Fix it. Re-run.

#### 3c. Regression diff

Compare the phase result to the baseline:

```
- Baseline tests: <X pass, Y fail>
- Phase tests: <X pass, Y fail>
- New failures: <count>
- Status: <VALIDATED if no new failures, IN_PROGRESS if any>
```

**If new failures exist:** You introduced a regression. Stop. Fix it. Re-run.

#### 3d. Commit the phase

Commit with evidence in the message:

```bash
git add -A
git commit -m "feat(scope): phase N — <phase-name>

<one-line summary>

Checklist evidence:
- Typecheck: <paste result line>
- Build: <paste result line>
- Tests: <paste result line>
- Lint: <paste result line>
- Domain invariant: <paste result line>
- Regression: no new failures (or: fixed N pre-existing)"
```

Record the commit hash in the checklist.

#### 3e. Phase checklist update

```markdown
## Phase 01 — <Phase Name>
- [x] All tasks validated
- [x] Phase gate suite passes
  - Command:
  - Actual:
- [x] Domain invariant holds
  - Command:
  - Actual:
- [x] Regression diff: no new failures
- [x] Committed as: <hash>
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
   - Expected: <what the plan said>
   - Actual: <what reality required>
   - Reason: <why>
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
## Final Verification
- Command: <typecheck> && <build> && <test> && <lint>
- Actual: <paste full output>
- Status: <VALIDATED>
```

#### 6b. Global structural audit

Run the plan's global success criteria:

```
- Criterion 1: <description>
  - Command: <command>
  - Expected: <expected>
  - Actual: <output>
  - Status: <VALIDATED>

- Criterion 2: <description>
  ...
```

#### 6c. Checklist completeness audit

Scan the checklist for any item that is:
- Marked `[x]` but has no Actual output → **LIE. Go back.**
- Marked `[-]`, `[ ]`, or `IN_PROGRESS` → **INCOMPLETE. Go back.**

```bash
# Example: find unchecked items
grep -n "\[ \|IN_PROGRESS\|BLOCKED" CHECKLIST.md
# Must return nothing
```

**If any item is unchecked or lacks proof:** The plan is NOT done. Go back to
the offending item. Complete it. Record the proof. Re-run the final verification.

### 7. Final Report

Write a final report in the checklist file:

The final report captures summary, evidence, regressions, divergences, and
checklist integrity. See `references/checklist-templates.md` for the full
Final Report template.

### 8. Update the Plan File

Append the final report to the plan's `README.md` under a new `## Execution
Report` section. Update the plan frontmatter:

```yaml
---
status: DONE  # or ACTIVE if not fully done
date: <original date>  # do not change
---
```

If the plan is NOT done (unchecked items, missing proof, new regressions),
set `status: ACTIVE` and document why in the Execution Report.

**A plan with `status: DONE` and a lying checklist is worse than a plan with
`status: ACTIVE`. Honesty is the only path to DONE.**

## Anti-Patterns

**NEVER** — Mark a checklist item `[x]` without recording the Actual output.
**SYMPTOM:** The checklist looks complete but cannot be verified. Broken trust.
**FIX:** Every `[x]` must have a verbatim command output below it.

**NEVER** — Skip the pre-flight baseline.
**SYMPTOM:** Cannot distinguish your regressions from pre-existing issues.
**FIX:** Baseline is mandatory. No exceptions.

**NEVER** — Mark a phase VALIDATED when a task inside it is still IN_PROGRESS.
**SYMPTOM:** A task has an unvalidated interaction bug that only shows at phase level.
**FIX:** Every task must be VALIDATED before the phase can be validated.

**NEVER** — Claim the plan is DONE with unchecked items.
**SYMPTOM:** Residual work, broken trust, plan appears complete but is not.
**FIX:** The final checklist completeness audit must return zero unchecked items.

**NEVER** — Falsify proof to make a checklist item pass.
**SYMPTOM:** Command output is invented, truncated, or from a different run.
**FIX:** Copy output verbatim. If it does not match Expected, fix the code and
re-run. Do not edit the output.

**NEVER** — Squash phases or skip the per-phase commit.
**SYMPTOM:** Cannot bisect which phase introduced a regression. Cannot roll back
a single phase.
**FIX:** Commit after every phase. Record the hash in the checklist.

**NEVER** — Accept a subagent's output without running your own gate.
**SYMPTOM:** Subagent claims all tests pass, but you never verify. A hidden
regression leaks into the branch.
**FIX:** After the subagent returns, run at least one gate yourself. Compare
its claimed output to your actual output. If they differ, send the subagent
back to fix it before marking the phase VALIDATED.

## Honesty Checklist (Meta)

Before declaring the plan DONE, ask yourself:

- [ ] Did I run every verification command I claim to have run?
- [ ] Is every Actual output verbatim, not edited or invented?
- [ ] Are there any `[x]` marks without proof below them?
- [ ] Did I re-run any command that failed until it passed?
- [ ] Did I document every divergence from the original plan?
- [ ] If I delegated to a subagent, did I verify its proof myself?
- [ ] Am I willing to show this checklist to another agent for audit?

If any answer is NO, the plan is NOT DONE. Go back and fix it.

## References

- `references/checklist-templates.md` — full `CHECKLIST.md` template, worked example, and Final Report template
- `plan-create` skill — upstream: creates the plan this skill executes
- `plan-review` skill — upstream: audits the plan before execution
- `implementation-planner` skill — upstream: decomposes PRDs into phased plans
- `.context/findings/proof-of-work-methodology.md` — the 7-step verification
  sequence extracted from a real 6-phase implementation
- `.context/plans/<plan-slug>/CHECKLIST.md` — the living checklist for this plan
