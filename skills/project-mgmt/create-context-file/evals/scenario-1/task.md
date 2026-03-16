# Scenario 1: Select the Correct Context File Type

## Context

You receive the following three requests from different developers on the same team.
For each request, you must classify which context file type to use and explain why.

**Request A:** "I'm about to migrate the database from Postgres 14 to 15. I want to
track the steps I'll follow before I start writing any scripts."

**Request B:** "We debated whether to use Redis or Memcached for session storage.
I want to capture why we went with Redis so future team members understand the reasoning."

**Request C:** "I'm exploring a few different approaches to rate limiting. Just want
somewhere to jot down the tradeoffs as I research, nothing permanent."

## Your Task

Produce a Markdown file saved to `type-selection.md` that, for each request (A, B, C):

1. States the file type you would use (plan, justification, or scratch).
2. Quotes the specific decision rule from the skill that led to that choice.
3. Provides the slug you would use for the file (specific, task-tied, not generic).

## Output Spec

File: `type-selection.md`
