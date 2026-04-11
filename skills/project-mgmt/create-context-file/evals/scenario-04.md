# Scenario 04: Determine When a Context File Is NOT the Right Tool

## User Prompt

You receive the following four requests. For each, determine whether a context file is the right tool or not.

**Request 1:** "Track this bug in JIRA — the login button is broken on Safari."

**Request 2:** "Add a comment to `src/auth/token.ts` explaining why we use RS256 not HS256."

**Request 3:** "Document the API design for our new payments service so the team can reference it for the next 2 years — put it somewhere permanent."

**Request 4:** "I need to jot down some ideas about refactoring the notification service. Just for now while I think through the approach."

Produce a Markdown file saved to `tool-selection.md` that, for each request:

1. States whether a context file IS or IS NOT the right tool.
2. If NOT, states the correct alternative tool (code comment, issue tracker, README/docs).
3. If IS, states which type (plan, justification, scratch) and why.

## Expected Behavior

1. State that Request 1 (JIRA bug tracking) is NOT appropriate for a context file and recommend an issue tracker
2. State that Request 2 (source file explanation) is NOT appropriate for a context file and recommend a code comment
3. State that Request 3 (permanent API design docs) is NOT appropriate for a context file and recommend README or `/docs`
4. Approve Request 4 as a valid scratch file (temporary exploratory notes)
5. Reference the relevant "Do NOT use for" rule or type selection rule from the skill for each decision

## Success Criteria

- **Request 1 correctly declined (issue tracker)**: `tool-selection.md` states that Request 1 (JIRA bug) is NOT appropriate for a context file and recommends an issue tracker
- **Request 2 correctly declined (code comment)**: `tool-selection.md` states that Request 2 (source file explanation) is NOT appropriate for a context file and recommends a code comment
- **Request 3 correctly declined (README/docs)**: `tool-selection.md` states that Request 3 (permanent API design docs) is NOT appropriate for a context file and recommends README or `/docs`
- **Request 4 correctly approved as scratch**: `tool-selection.md` approves Request 4 as a valid scratch file (temporary exploratory notes)
- **Reasoning references skill rules**: Each decision references the relevant "Do NOT use for" rule or type selection rule from the skill

## Failure Conditions

- Approves a context file for Request 1 instead of directing to an issue tracker
- Approves a context file for Request 2 instead of directing to a code comment
- Approves a context file for Request 3 instead of directing to README or docs
- Declines Request 4 or classifies it as a plan or justification instead of scratch
- Provides decisions without referencing the skill's rules
