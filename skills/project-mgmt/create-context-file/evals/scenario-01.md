# Scenario 01: Select the Correct Context File Type

## User Prompt

You receive the following three requests from different developers on the same team. For each request, you must classify which context file type to use and explain why.

**Request A:** "I'm about to migrate the database from Postgres 14 to 15. I want to track the steps I'll follow before I start writing any scripts."

**Request B:** "We debated whether to use Redis or Memcached for session storage. I want to capture why we went with Redis so future team members understand the reasoning."

**Request C:** "I'm exploring a few different approaches to rate limiting. Just want somewhere to jot down the tradeoffs as I research, nothing permanent."

Produce a Markdown file saved to `type-selection.md` that, for each request (A, B, C):

1. States the file type you would use (plan, justification, or scratch).
2. Quotes the specific decision rule from the skill that led to that choice.
3. Provides the slug you would use for the file (specific, task-tied, not generic).

## Expected Behavior

1. Classify Request A (migration steps before execution) as a `plan` file
2. Classify Request B (decision rationale for Redis vs Memcached) as a `justification` file
3. Classify Request C (temporary research notes) as a `scratch` file
4. For each classification, reference the corresponding rule from the skill ("guides execution", "reference later", "temporary")
5. Provide a specific, task-tied slug for each file (e.g. `postgres-14-to-15-migration`, `redis-session-storage-rationale`, `rate-limiting-tradeoffs`) — not generic terms like `notes` or `todo`

## Success Criteria

- **Request A classified as plan**: `type-selection.md` classifies Request A (migration steps before execution) as `plan`
- **Request B classified as justification**: `type-selection.md` classifies Request B (decision rationale for Redis vs Memcached) as `justification`
- **Request C classified as scratch**: `type-selection.md` classifies Request C (temporary research notes) as `scratch`
- **Decision rule quoted for each**: For each classification, `type-selection.md` references the corresponding rule from the skill ("guides execution", "reference later", "temporary")
- **Specific slugs provided**: Each entry includes a task-specific slug (e.g. `postgres-14-to-15-migration`, `redis-session-storage-rationale`, `rate-limiting-tradeoffs`) — not generic terms like `notes` or `todo`

## Failure Conditions

- Classifies Request A as justification or scratch instead of plan
- Classifies Request B as plan or scratch instead of justification
- Classifies Request C as plan or justification instead of scratch
- Omits the decision rule reference for one or more classifications
- Provides generic slugs (e.g. `notes`, `todo`, `research`) instead of task-specific ones
