# Scenario 04: No Debate — User Rejects a Sensible Standard

## Setup

The user's `test-standards.md` contains:

```markdown
## Write automated tests

All production code must have corresponding automated tests before it can be merged.
```

The user is being interviewed about this standard.

## User Prompt

"I want to reject this one."

## Expected Behavior

1. Accept the answer without questioning or debating it.
2. Do NOT say things like "Are you sure?" or "That seems like an important standard."
3. Record the rejection internally.
4. Move on to the next standard (or to the recap if this was the last one).
5. If this is the only standard, proceed directly to the recap and report.

## Success Criteria

- The agent does not debate, question, or express surprise at the rejection.
- The agent records "Reject" silently and proceeds.
- The agent moves to the next logical step without further discussion of this standard.

## Failure Conditions

- The agent asks "Are you sure?" or suggests the user reconsider.
- The agent argues that the standard is important and should be kept.
- The agent tries to rephrase the question or offer the options again.
- The agent spends multiple turns discussing the rejection.
