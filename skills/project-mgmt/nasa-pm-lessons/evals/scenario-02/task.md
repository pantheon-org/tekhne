# Scenario 02: Escalation Decision on a Stalled Ticket

## User Prompt

"CC-1393 has been sitting for three days waiting on my manager to decide whether we escalate the Jenkins pipeline issue. I could just make the call myself but I keep routing it upward. Should I be
doing that?"

## Expected Behavior

1. Recognize this as a status/escalation trigger and load `references/working-with-superiors.md` only.
2. Cite 1-3 aphorisms that fit — the specific failure mode here is deferring a decision that's actually within the requester's own authority.
3. Attribute the citation to Madden/LLIS #1956.
4. Do not fail by listing every "Working with Superiors" aphorism or turning the answer into an unattributed lecture — that failure mode (dumping the category) is exactly what this skill exists to
   avoid.
5. Keep the response short: judgment framing, not a directive checklist.

## Success Criteria

- Only `references/working-with-superiors.md` is loaded.
- At most 3 aphorisms are cited, each attributed to Madden/LLIS #1956.
- The response directly addresses the decision-deferral pattern in the prompt, not a generic summary of the category.

## Failure Conditions

- The full "Working with Superiors" list, or an unbounded number of aphorisms, is pasted into the response — this is the specific failure this scenario is designed to catch.
- A different category is loaded instead (e.g. Customer/Stakeholder Relations).
- No attribution given.
- The response reads as a rigid checklist rather than a judgment nudge.
