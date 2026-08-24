# Scenario 01: Draft a New Plan From a Rough Idea

## User Prompt

"Create a plan for adding an SQS dead-letter-queue redrive script to the retry
sweep. It touches the DLQ Terraform resource, the retry Lambda, and the
runbook. Should take a few sessions. It unblocks recovering failed deliveries
without a manual S3 replay."

## Expected Behavior

1. Gather goal, scope, phases, tasks, dependencies, and risks from the prompt
   before drafting -- do not skip straight to writing the file.
2. Grade `effort` against the rubric (three files/one subsystem -> `M`, not a
   guess) and `value` (unblocks a previously-manual recovery path -> `HIGH`),
   rather than picking arbitrary values.
3. Tag `themes` primary-first from the closed vocabulary (`DELIVERY` first,
   since it changes delivery recovery behaviour; `INFRA` second, since it
   touches Terraform).
4. Open the plan with valid YAML frontmatter: `title`, `type: PLAN`,
   `status: DRAFT`, `date`, `effort`, `value`, `themes`.
5. Structure the work into 2-5 phases, each with 2-5 concrete tasks, not a
   flat list of steps.
6. Mark any tasks that can run in parallel as an explicit wave.
7. Include a `## Goal`, `## Phases`, and `## Open Questions` section.
8. Run the frontmatter validation script on the created file and report the
   result before declaring the plan done.
9. Offer to run `plan-review` on the new plan as the next step.

## Success Criteria

- Frontmatter present and valid: `title`, `type: PLAN`, `status: DRAFT`,
  `date`, `effort: M`, `value: HIGH`, `themes` starting with `DELIVERY`.
- 2-5 phases, each with 2-5 tasks and a stated exit criterion.
- At least one task explicitly marked as parallelisable (or an explicit
  statement that none are), not silently left ambiguous.
- `## Goal`, `## Phases`, `## Open Questions` sections present.
- `effort`/`value` justified by the rubric's leverage/reversibility framing,
  not asserted without reasoning.
- Validation script run (or its command shown) before the plan is presented
  as complete.
- `plan-review` offered as a next step.

## Failure Conditions

- Plan created without YAML frontmatter, or with a missing required field.
- `effort` or `value` picked without any stated justification.
- A single flat list of 10+ steps with no phase grouping.
- No exit criterion stated for any phase.
- Validation step skipped entirely.
- `status` set to `ACTIVE` instead of the required default `DRAFT`.
