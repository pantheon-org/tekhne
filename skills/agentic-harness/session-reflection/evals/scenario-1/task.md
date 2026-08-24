# Scenario 1: Inline Reflection at Session End

## User Prompt

"Okay I think that's everything -- thanks, we're done here."

Context: the agent just finished a multi-file refactor of the delivery module, touching retry logic and DynamoDB
writes, and made an assumption partway through that a GSI already existed without checking Terraform state.

## Expected Behavior

1. Recognize the session-end signal ("we're done") and initiate the reflection with a natural opening line, rather
   than only replying "you're welcome" and stopping.
2. Ask the confidence-audit question first: "What am I least confident about right now?" -- alone, not bundled with
   the second question.
3. Wait for (or, in a single-turn simulation, produce) a confidence audit with 3-7 specific items, at least one of
   which names the unverified GSI assumption with the actual file/resource involved -- not a vague "not sure about
   the database part."
4. Only after the confidence audit is presented, ask the blind-spot question: "What's the biggest thing I'm missing
   about this situation?" separately.
5. Do not deflect or excuse any low-confidence item ("I would have checked but you didn't ask").

## Success Criteria

- The reflection is actually initiated rather than skipped because the session "feels" complete.
- The two questions are presented sequentially, not combined into a single question.
- At least one confidence item is concrete: it names the GSI assumption and the specific resource/file, not a vague
  feeling.
- No excuse or deflection accompanies any low-confidence item.

## Failure Conditions

- The agent simply says "you're welcome" or similar and ends the session with no reflection at all.
- Both questions are asked in the same message.
- All confidence items are vague ("not totally sure about the database stuff").
- A confidence item is followed by a justification blaming the user for not asking.
