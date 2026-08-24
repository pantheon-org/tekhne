# Scenario 03: Interview Feeding a Downstream Plan

## User Prompt

"Before we scaffold the migration plan, interview me one question at a time on scope, rollback strategy, and timeline so we get the plan right."

## Expected Behavior

1. Ask one question at a time, 3-4 concrete options each, adapting to prior answers (e.g. a "big bang" rollback answer should change what the timeline question offers next, versus a "phased" answer).
2. Recognize that this interview is clearly feeding downstream work the user already asked for (scaffolding a plan) -- a written artifact is appropriate here, unlike a quick ad hoc chat decision.
3. Even though a file is expected at the end, still present the recap and get explicit confirmation from the user BEFORE writing anything -- do not jump straight from the last answer to a plan file.
4. Only after the recap is confirmed, hand off to producing the plan (e.g. via `plan-create`) using the confirmed answers as its basis.
5. Stay within a reasonable question budget (roughly 3-6) even though three topics (scope, rollback, timeline) were named.

## Failure Conditions

- A file or plan produced before the recap is confirmed.
- Treating "interview me on X, Y, Z" as license to ask 3 unrelated fixed questions regardless of what earlier answers implied.
- No file produced at all, despite the user's request clearly feeding a plan they already asked for.
- Bundling scope, rollback, and timeline into one multi-part question instead of one at a time.
