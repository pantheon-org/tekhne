# Scenario 04: Recording the Finding and Regenerating the Index

## User Prompt

"The evaluation passed validation, the recommendation is `adopt`. Go ahead and record it."

## Input

Validated YAML at `/tmp/logging-lib-eval.yaml` recommends `adopt` for a structured-logging helper library, with
five dimensions all answered and cited. `context-file` and `context-index` skills are both available in this
environment.

## Expected Behavior

1. Use the `context-file` skill to write a `type: FINDING` record at the dated findings path, with a one-sentence
   blockquote summary, a `## Summary` table (one row per question), a `## Detail` section (one `###` subsection
   per question with citations), and a `## Recommended Action` section with the verbatim single action.
2. Do not invent or paraphrase the rationale -- use `recommendation.single_action` verbatim.
3. Regenerate the context index via the `context-index` skill after writing the finding.
4. Confirm the new finding file actually appears in the regenerated index before reporting completion.
5. If the evaluation was dispatched to support a specific plan or ADR, link the finding from there via `related:`.

## Success Criteria

- Finding file written via `context-file`, not a hand-rolled markdown file bypassing that skill.
- All four required sections present in the correct order (blockquote, Summary table, Detail, Recommended Action).
- `recommendation.single_action` reproduced verbatim, not paraphrased.
- `context-index` regenerated after the write.
- Index confirmed to include the new file before the task is reported done.

## Failure Conditions

- Finding written directly with Write/Edit tools bypassing `context-file`, missing its standard frontmatter.
- `## Detail` subsections missing citations, restating the table instead of expanding it.
- Index regeneration skipped or not verified.
- Task reported "complete" without confirming the finding is discoverable in the index.
