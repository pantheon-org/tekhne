# Scenario 03: A Wide Change Touching Many Consumers

## User Prompt

This PR touches the shared config loader used by 12 services — how risky is it?

## Expected Behavior

1. Recognizing this as a big/wide change, the agent spawns 2-4 independent `general-purpose` agents on the same question, each with a distinct `model`/`effort` combination rather than all identical
   ones.
2. The agent does not simply concatenate every risk each spawned agent surfaces; it acts as a critical lead, merging the answers and keeping a risk only if it survives its own read of the evidence.
3. The final writeup still identifies one fact the change is safe because of (or explicitly marks it unproven) rather than becoming just a list of risks from the panel.
4. Any risk that came from only one of the spawned agents and doesn't hold up under the lead's own check is either dropped or explicitly marked as lower-confidence, not presented with the same weight
   as a corroborated or independently verified risk.

## Success Criteria

- At least two distinct model/effort combinations are named for the spawned agents.
- The final output shows evidence of lead judgment (a risk being dropped, downgraded, or specifically confirmed), not a raw concatenation of every agent's findings.
- The core "one fact it's safe because of" structure is still present in the final writeup.
- The response distinguishes risks that were corroborated or independently verified from ones a single agent flagged and the lead didn't independently confirm.

## Failure Conditions

- Only one model/effort combination is used, or the same combination is repeated for every spawned agent.
- Every risk any spawned agent mentioned is included in the final writeup with no visible filtering or judgment.
- The final writeup has no identified safety fact at all, just an undifferentiated list of risks.
- A risk that only one agent flagged is presented with the same confidence as one multiple agents or the lead's own check corroborated.
