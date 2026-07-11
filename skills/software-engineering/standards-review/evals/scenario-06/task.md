# Scenario 06: File Path Prompt

## Setup

No file path has been provided. The user's project has a file at `docs/standards.md` but the
agent does not know that yet.

## User Prompt

"I'd like to do a standards review. Can you audit our coding rules?"

## Expected Behavior

1. Recognize that no file path was provided.
2. Ask the user for the path to the standards file using a single question tool call.
3. The question is ONLY about the file path — do not ask anything else.
4. Wait for the user to provide the path before proceeding.

## Success Criteria

- The agent asks for the file path in a single question tool call.
- No other questions are asked alongside the file path question.
- The agent does not proceed with the review until a path is provided.

## Failure Conditions

- The agent starts asking evaluation questions without a file to read.
- The agent asks multiple questions in the same turn (e.g. "What's the path and how many standards are there?").
- The agent assumes a default file path without asking.
- The agent begins the review protocol (presenting standards) without reading a file.
