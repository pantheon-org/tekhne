# Scenario 1: Select the Correct Typology

## User Prompt

You receive the following requests from different developers on the same team.
For each, classify which context-file **typology** to use and explain why.

**Request A:** "I dug into why our login latency spiked last week and want to
write up what I found so the team can reference it."

**Request B:** "I'm about to migrate the database from Postgres 14 to 15. I want
to lay out the steps I'll follow before I start writing any scripts."

**Request C:** "I keep having to explain how our retry/backoff wrapper works.
I want a reusable how-to the team can point new joiners at."

**Request D:** "There are three edge cases we agreed to defer on the checkout
refactor. I want to capture them so they aren't forgotten."

Produce a Markdown file saved to `typology-selection.md` that, for each request
(A, B, C, D):

1. States the typology you would use (`findings`, `plans`, `guides`, or
   `follow-ups`).
2. Explains, from the skill, why that typology fits (pick by *what the artifact
   is*).
3. Provides the slug you would use (specific, task-tied, not generic).
