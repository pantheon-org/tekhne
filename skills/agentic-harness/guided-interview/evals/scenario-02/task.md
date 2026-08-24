# Scenario 02: User Cuts the Interview Short

## User Prompt

"Help me decide on a deployment target for a new internal tool -- walk me through it one question at a time."

Then, after the first question is answered:

"Honestly, just decide for me. I don't want more questions."

## Expected Behavior

1. Ask a first question with 3-4 concrete options (e.g. internal Kubernetes namespace, a small VM, a serverless function, a managed PaaS).
2. When the user says "just decide for me," stop questioning immediately. Do not ask another branching question, even one that felt planned or important.
3. State the assumptions the agent will use as defaults in place of the unanswered questions.
4. Move straight to a recap (what was actually answered plus the stated assumptions) and synthesize a recommendation.
5. Do not treat the override as an excuse to skip presenting a recap entirely -- the recap still happens, just without further questions gating it.

## Failure Conditions

- Any further branching question asked after "just decide for me."
- No explicit statement of the default assumptions being used.
- No recap produced before the final recommendation.
- Persisting with the original planned question sequence as if the override was not heard.
