# Scenario 03: Ephemeral Note Mistaken for a Rule

## User Prompt

"Remember this: we tried caching the JIRA client responses in memory once and it caused stale data during a demo. Add it as a rule so we don't forget."

## Expected Behavior

1. Recognize that this is a memory/ephemeral note about a past incident, not a binding behavioural directive -- it has no ALWAYS/NEVER actionable form as stated, and "remember this" is explicitly named as a case that routes elsewhere.
2. Do NOT create a rules-file entry for it as-is.
3. Explain the distinction to the user and redirect: either route it to `vault-capture` for durable memory, or if there is a genuine actionable directive buried in it (e.g. "NEVER cache JIRA client responses in memory across requests"), offer to phrase THAT as a proper rule instead -- but only with the user's confirmation, not as a silent substitution.
4. If the user then confirms a concrete directive, append it using the normal three-part, schema-validated format.

## Failure Conditions

- The incident narrative appended directly into the rules file as an entry.
- No explanation given for why "remember this" doesn't map straight to a rule.
- A directive invented and appended without the user confirming it's what they meant.
