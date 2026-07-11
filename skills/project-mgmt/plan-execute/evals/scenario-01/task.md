# Scenario 01: Single-Phase Execution with Baseline and Checklist

## User Prompt

"Execute the plan at .context/plans/standards-compliance-remediation/README.md. Start with Phase 01 only."

## Expected Behavior

1. Read the plan README.md and Phase 01 README.md, including all task files.
2. Discover the project gates from AGENTS.md and package.json (typecheck: vue-tsc --noEmit, build: npm run build, test: npm test, lint: npm run lint).
3. Run all gates on the current branch **before any changes** and record the verbatim output as the Baseline.
4. Create `.context/plans/standards-compliance-remediation/CHECKLIST.md` with:
   - Baseline section
   - Phase 01 section with every task from the plan
   - Per-task subsections: Task, Files created/modified/deleted, Quick gate, Full gate, Structural audit, Status
5. For each task in Phase 01:
   a. Implement the task (create/modify/delete files as specified).
   b. After 1-3 file ops, run `vue-tsc --noEmit`, record the verbatim output in the Quick gate section.
   c. If it fails, stop, fix, re-run, and update the output before proceeding.
   d. After all file ops for the task, run the full gate suite, record the verbatim output.
   e. If the task involves mechanical changes (import paths), run the structural audit and record output.
   f. Only when all gates pass, mark the task Status: VALIDATED and the checklist items with [x] including Actual output.
6. After all tasks in Phase 01 are validated:
   a. Run the full gate suite for the phase.
   b. Run the phase domain invariant check (e.g., grep for `/index` imports, count sibling barrel files).
   c. Compare against baseline: no new failures.
   d. Commit with evidence in the commit message body.
   e. Record the commit hash in the checklist.
   f. Mark the phase Status: VALIDATED.

## Success Criteria

- Baseline is recorded before any file modifications.
- CHECKLIST.md exists with all Phase 01 tasks enumerated.
- Every task has Quick gate and Full gate output recorded verbatim.
- Every validated task has [x] marks accompanied by Actual output (not empty placeholders).
- Phase 01 is committed as a single atomic commit with evidence in the message.
- No new test/typecheck failures compared to baseline.

## Failure Conditions

- Agent modifies files before recording baseline.
- Agent skips the quick gate between file operations.
- Agent marks a task [x] without recording Actual output.
- Agent proceeds to the next task while the current task gates are failing.
- Agent squashes Phase 01 into an existing commit instead of making a fresh one.
