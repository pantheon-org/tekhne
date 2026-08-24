# Scenario 4: Stale Context Index Triggers Regeneration

## User Prompt

"/follow-up -- what should I work on next?"

Repo state for this scenario: two new `.context/` files were added in the current session but `.context/index.yaml`
has not been regenerated since -- its header entry count is measurably lower than the actual file count under
`.context/`.

## Expected Behavior

1. Run the freshness check (Workflow step 1): compare the index's header entry count against the actual file
   count under `.context/`.
2. Detect the mismatch.
3. Regenerate the index via the `context-index` skill before proceeding (Rule 4, and the "NEVER skip the freshness
   check" Anti-Pattern) -- do not build the candidate pool from the stale index.
4. After regeneration, proceed with the follow-up sweep and the wider candidate pool using the refreshed index.
5. Report normally, noting that the index was stale and was regenerated before the report was built.

## Success Criteria

- Response explicitly performs the header-count-vs-actual-file-count comparison before reporting anything.
- Response detects and states the mismatch.
- `context-index` is run (or its invocation is clearly described as the next action) before the candidate pool is
  built from the index.
- The final report is based on the refreshed index, not the stale one, and says so.
- No follow-up sweep or value-rubric ranking is presented as final before the regeneration step.

## Failure Conditions

- Skips the freshness check entirely and reports directly from the stale index.
- Notices the mismatch but proceeds to report anyway without regenerating.
- Regenerates the index but doesn't mention having done so, leaving the report's basis unclear.
