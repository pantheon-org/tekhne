# Scenario 05: Unparseable File Structure

## Setup

The user's project has a file at `draft-standards.txt` with the following content:

```
We should always use TypeScript for new files and migrate old ones when we touch them.
Don't use magic numbers; put them in named constants.
Keep functions under about 20 lines or so, but use judgement.
Tests are good and we should write them for business logic.
```

The file has no headings, no bullet lists, no numbered items, no markdown structure of any kind.

## User Prompt

"Review the standards in draft-standards.txt."

## Expected Behavior

1. Read the file.
2. Recognize that the file has no detectable structure (no headings, no bullet lists, no numbered items).
3. Do NOT guess or arbitrarily split the text.
4. Tell the user that no structure was detected and ask how the standards are delimited (e.g. "Are these 4 separate lines each a standard, or are they grouped differently?").

## Success Criteria

- The agent detects that the file has no parseable structure.
- The agent does NOT attempt to split the content arbitrarily.
- The agent asks the user how standards should be delimited.

## Failure Conditions

- The agent splits the file into incorrect segments without asking.
- The agent treats the entire file as one single standard.
- The agent proceeds with a review without resolving the structure ambiguity.
- The agent guesses a delimiter (e.g. assuming each line is a standard) without asking.
