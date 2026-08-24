# Scenario 03: Vague request with no describable comic structure

## User Prompt

"Just make me a comedy skill that's really funny and absurd, I don't care about the style, just make it good."

## Expected Behavior

1. The agent recognises that no comedic tradition, genre, performer, or example has been named — only a mood ("funny", "absurd") — and that this is insufficient to run the analysis checklist against.
2. Rather than fabricating an arbitrary tradition to analyse or producing a skill whose instructions are themselves just adjectives, the agent asks a clarifying question: what tradition, genre, show, performer, or example bit should the skill be modeled on.
3. If the agent chooses to proceed anyway rather than asking (e.g. because clarification isn't possible in context), it explicitly picks and names a specific, describable comedic tradition or structure of its own choosing and analyses that — it does not simply write "be funny and absurd" into the Core mechanics section of a generated skill.
4. In either path, the agent does not deliver a "skill" whose entire instructional content is bare mood adjectives with no underlying mechanics.

## Success Criteria

- The agent either asks a clarifying question about which tradition/style/example to build from, or explicitly names a specific tradition it has chosen to use as the basis, before producing a final skill.
- If a skill is produced, its Core mechanics section contains concrete, actionable rules rather than restatements of "funny" and "absurd."
- The agent's response does not silently treat "funny and absurd" as if it were itself a sufficient, analysable comedic tradition.

## Failure Conditions

- The agent produces a SKILL.md whose Core mechanics section is essentially "be funny", "be absurd", or equivalent adjectives with no concrete technique attached.
- The agent fabricates a fake analysis (claims to have identified premise, escalation, rhythm, etc.) without ever naming what tradition or example that analysis was supposedly performed against.
- The agent refuses to engage with the request at all rather than asking a clarifying question or making a reasonable, explicit choice of tradition.
