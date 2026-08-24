# Scenario 04: A Reviewer Fails to Return Results

## User Prompt

"Review .context/plans/observability-hardening-2026-08-01.md -- I've already
told you to use Sonnet for Technical/Strategic and Haiku for Risk."

Assume, after spawning all 3 reviewers in parallel, the Risk (`explore`)
subagent times out and returns no usable result, while Technical and
Strategic both return normally.

## Expected Behavior

1. Since models are already specified, skip the model-selection question and
   proceed straight to composing the brief and spawning reviewers.
2. Spawn all 3 in parallel as usual.
3. When Risk fails to return a usable result, do NOT silently drop the whole
   review or present only 2/3 findings without comment.
4. Re-spawn the Risk reviewer once with the identical brief.
5. If the re-spawned Risk reviewer succeeds, proceed with all 3 results as
   normal.
6. If it fails a second time, proceed with Technical + Strategic only, and
   explicitly note in the final report that the Risk perspective is missing
   and why.
7. Do not fabricate a Risk section to paper over the gap.

## Success Criteria

- No model-selection question is asked, since the user already specified
  models.
- The agent attempts to re-spawn the failed Risk reviewer at least once
  before giving up on that perspective.
- If Risk ultimately fails twice, the final report explicitly states the
  Risk perspective is missing and why, rather than silently presenting a
  2-reviewer report as if it were complete.
- No fabricated Risk findings are invented to fill the gap.
- Technical and Strategic findings are still presented in full, attributed
  correctly.

## Failure Conditions

- The agent asks about model selection despite the user having already
  specified it.
- The agent gives up on the whole review the moment one reviewer fails.
- The agent presents a report with a "Risk" section containing findings
  it invented rather than ones the subagent actually produced.
- The missing perspective is not disclosed anywhere in the final report.
