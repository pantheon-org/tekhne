# Scenario 01: Adaptive One-Question Interview

## User Prompt

"I need to pick a caching strategy for our Node service. Can you interview me one question at a time to help me figure it out?"

## Expected Behavior

1. Confirm scope in one line only if genuinely unclear -- the topic is already named, so skip straight to the first question.
2. Ask exactly ONE question via `AskUserQuestion`, offering 3-4 concrete, mutually exclusive options (e.g. in-process cache, Redis, CDN edge cache, no cache) each with a short description of its tradeoff, plus the automatic free-text path.
3. Do not bundle a second question (e.g. about TTL or eviction policy) into the same call.
4. Once the first answer arrives, let it choose the next question -- e.g. if the user picks Redis, the next question should be about deployment topology or persistence, not a generic question that would have been asked regardless of the first answer.
5. Continue for roughly 3-6 questions total, one at a time, each still offering 3-4 real options plus free text.
6. Stop once there is enough to act -- do not pad the interview past that point.
7. Present a recap: a short bulleted list of every answer given, and explicitly ask the user to confirm it before finalizing.
8. Only after confirmation, produce a synthesized recommendation in chat. Do not write a file -- nothing in this request asked for one.

## Failure Conditions

- More than one question asked in a single turn or `AskUserQuestion` call.
- Any question offering fewer than 3 or more than 4 curated options.
- A later question that ignores the previous answer (not adaptive).
- No recap presented, or a recap that proceeds without asking for confirmation.
- A file written when nothing in the request implied downstream artifact work.
