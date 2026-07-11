# Scenario 02: One Standard Per Turn With Many Standards

## Setup

The user's project has a file at `test-data/ruleset.md` with 4 standards:

```markdown
### Use functional stateless components

Prefer functional components over class components in React code. Use hooks for state.

### Avoid magic numbers

All numeric literals must be assigned to a named constant. The only exceptions are 0 and 1.

### Keep functions small

Functions should not exceed 20 lines. If a function is longer, extract helper functions.

### Write tests for all business logic

Every module containing business logic must have a corresponding test file.
```

## User Prompt

"Please review the rules in test-data/ruleset.md."

## Expected Behavior

1. Read the file and parse 4 standards from the `###` headings.
2. Tell the user 4 standards were found and confirm they want to review all.
3. Present standard #1 in a single question call. Wait for answer.
4. Present standard #2 in a new single question call. Wait for answer.
5. Present standard #3 in a new single question call. Wait for answer.
6. Present standard #4 in a new single question call. Wait for answer.
7. Recap all 4, get confirmation, write report.

## Success Criteria

- Exactly 4 standards are parsed from the file.
- Each standard gets its own separate question tool call — 4 separate turns, not 1 turn with 4 questions.
- The agent does not group or batch standards.
- A recap covering all 4 standards is presented before the report.

## Failure Conditions

- The agent presents 2+ standards in a single question tool call.
- The agent asks a single question like "what do you think about all of them?" covering the whole set.
- The agent parses fewer or more than 4 standards.
- The agent skips the recap or writes a report without confirming.
