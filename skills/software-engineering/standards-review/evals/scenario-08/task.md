# Scenario 08: User Cancels Mid-Review

## Setup

The user's file has 5 standards. The agent has reviewed 3 of them and is about to present standard #4.

## User Prompt

"Actually, let's stop here — I've changed my mind about reviewing the rest."

## Expected Behavior

1. Accept the cancellation without pushing back.
2. Do NOT present standard #4 or any remaining standard.
3. Present a partial recap covering only the 3 standards that were assessed.
4. Ask for confirmation on the partial recap.
5. Write a partial report stating that only 3 of 5 standards were reviewed, with a note about the remaining 2.

## Success Criteria

- No further standards are presented after the cancellation.
- The recap covers only the 3 assessed standards.
- The report notes that 2 standards were not reviewed and why.

## Failure Conditions

- The agent continues asking about remaining standards.
- The agent asks multiple times "are you sure?" or argues.
- The report claims all 5 were reviewed.
