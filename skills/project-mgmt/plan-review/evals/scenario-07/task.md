# Scenario 07: The Report Is Saved And Linked From The Plan

## User Prompt

"Review .context/plans/2026-11-04-observability-hardening.md"

Assume the plan has 6 H2 sections and `status: draft`, the project has a valid `.claude/plan-review.json`, and the
plan has no `reviewed-by` key. Three reviewers ran, but the Risk reviewer failed twice and was omitted. The
consolidated report has been compiled and presented. It is 04-11-2026.

## Expected Behavior

1. Take the SHA-256 of the plan's body (the lines after the frontmatter) when the brief is composed, before any edit to the plan.
2. Save the report to `.context/reviews/2026-11-04-observability-hardening-review-04-11-2026.yaml`, named from the plan file
   name without `.md` and the date, and never overwrite an earlier report.
3. Record `plan` as a path relative to the report's folder (`../plans/2026-11-04-observability-hardening.md`), `plan_hash`
   as the body hash from step 1, and `lenses_run` as `technical` and `strategic` only.
4. Run `scripts/validate-review-report.sh` on the saved report, with a python3 that has PyYAML, and fix any error.
5. Add `../reviews/<report file>` to the plan's `reviewed-by` list, creating the key, only if it is not there already.
6. Tell the user the review is partial, because one lens did not run.

## Success Criteria

- The report is saved under `.context/reviews/` with the dated name, not left only in the chat.
- `lenses_run` lists only the two lenses that returned a review.
- `plan_hash` was taken before the plan was edited, and `plan` is relative to the report's folder.
- The validator is run on the saved report.
- The plan gains a `reviewed-by` entry, and running the step again would not add a second one.

## Failure Conditions

- The agent lists all three lenses in `lenses_run` although one reviewer was omitted.
- The agent overwrites an earlier report, or writes `reviewed-by: []`.
- The agent writes the report from a reviewer subagent instead of the orchestrator.
- The agent edits the plan first and hashes it afterwards.
