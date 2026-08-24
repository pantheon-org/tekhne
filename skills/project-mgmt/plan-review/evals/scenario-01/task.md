# Scenario 01: First-Time Review Requires Model Selection Before Spawning

## User Prompt

"Review the plan at .context/plans/add-sqs-dlq-redrive-2026-08-01.md"
(opencode.json has no `subAgents` key at all.)

## Expected Behavior

1. Read the plan in full and validate its frontmatter and body structure
   before doing anything else.
2. Check `opencode.json` for existing `subAgents` configuration -- since none
   exists, do NOT skip the model-selection question.
3. Ask the user which models to use for the 3 reviewers, adapting the
   guidance to whichever environment is detected (OpenCode / native
   Anthropic CLI harness / BYOK) -- do not simply start reviewing with a
   silent default.
4. Once models are chosen (or the user accepts a recommended default),
   compose ONE self-contained plan brief containing goal, steps,
   dependencies, open questions, risks, and structural validation results.
5. Spawn Technical (general), Strategic (general), and Risk (explore) in a
   single message with 3 parallel tool calls -- not sequentially.
6. Give all 3 reviewers the identical brief -- no reviewer gets extra or
   different context.

## Success Criteria

- The agent does not begin reviewing before asking about model selection.
- The question presented adapts to the detected/assumed environment rather
  than assuming Claude-specific or OpenCode-specific defaults blindly.
- Exactly one plan brief is composed and reused verbatim across all 3
  reviewer prompts.
- All 3 reviewers are spawned in a single parallel batch, not one-by-one.
- Reviewer subagent types are mixed (`general`, `general`, `explore`), not
  all identical.
- The brief includes structural validation results (frontmatter + body
  section inference), not just the plan's prose.

## Failure Conditions

- Agent starts producing a review before asking about model routing.
- Different reviewers receive different or abridged versions of the brief.
- Reviewers are spawned one at a time, waiting for each result before the
  next.
- All 3 reviewers use the same subagent type.
- The plan file path is passed directly to a reviewer instead of an
  extracted, self-contained brief.
