# Scenario 05: Apply the Decision Rule to Ambiguous Requests

## User Prompt

A developer says: "I need to figure out the best way to handle versioning for our public API. I'm going to investigate URL versioning vs header versioning vs no versioning. Once I decide, I'll want to remember why. And then I'll need to track the steps to actually implement whatever I choose."

This single statement describes three separate needs at once.

Using the create-context-file skill's decision rule ("If you'll reference it later → justification. If it's temporary → scratch. If it guides execution → plan"), produce a file saved to `decision-breakdown.md` that:

1. Identifies all three distinct needs embedded in the developer's statement.
2. Maps each need to the correct file type using the explicit decision rule.
3. Proposes a specific slug for each file.
4. States the lifecycle expectation for each file (when it should be deleted or kept).

## Expected Behavior

1. Identify three separate needs: investigation/research, decision rationale, and implementation steps
2. Map the investigation phase (temporary research) to a scratch file
3. Map the "remember why" need to a justification file with "keep indefinitely" lifecycle
4. Map the implementation tracking need to a plan file with "delete after task complete" lifecycle
5. Provide specific, task-tied slugs for each of the three files (not generic terms like `notes` or `todo`)
6. Include explicit lifecycle statements for each file (when to delete or keep)

## Success Criteria

- **Three distinct needs identified**: `decision-breakdown.md` identifies three separate needs: investigation/research, decision rationale, and implementation steps
- **Research mapped to scratch**: `decision-breakdown.md` maps the investigation phase (temporary research) to a scratch file
- **Decision rationale mapped to justification**: `decision-breakdown.md` maps the "remember why" need to a justification file with "keep indefinitely" lifecycle
- **Implementation steps mapped to plan**: `decision-breakdown.md` maps the implementation tracking need to a plan file with "delete after task complete" lifecycle
- **Specific slugs and lifecycle expectations provided**: Each of the three files has a specific task-tied slug (not generic) and an explicit lifecycle statement (when to delete or keep)

## Failure Conditions

- Identifies fewer than three distinct needs, collapsing multiple needs into one file
- Maps the investigation phase to justification or plan instead of scratch
- Maps the decision rationale to scratch or plan instead of justification
- Maps the implementation steps to scratch or justification instead of plan
- Provides generic slugs (e.g. `notes`, `research`, `todo`) instead of task-specific ones
- Omits lifecycle expectations for one or more files
