# Scenario 5: Apply the Decision Rule to Ambiguous Requests

## Context

A developer says: "I need to figure out the best way to handle versioning for our
public API. I'm going to investigate URL versioning vs header versioning vs no versioning.
Once I decide, I'll want to remember why. And then I'll need to track the steps to
actually implement whatever I choose."

This single statement describes three separate needs at once.

## Your Task

Using the create-context-file skill's decision rule ("If you'll reference it later →
justification. If it's temporary → scratch. If it guides execution → plan"), produce
a file saved to `decision-breakdown.md` that:

1. Identifies all three distinct needs embedded in the developer's statement.
2. Maps each need to the correct file type using the explicit decision rule.
3. Proposes a specific slug for each file.
4. States the lifecycle expectation for each file (when it should be deleted or kept).

## Output Spec

File: `decision-breakdown.md`
