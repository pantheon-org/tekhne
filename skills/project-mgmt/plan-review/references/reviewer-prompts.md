# Reviewer Prompt Templates

Used in step 7 of SKILL.md. Insert the same plan brief at `<PASTE PLAN BRIEF HERE>` in all three.
Set `model` from step 4 or 5 (`technicalStrategic` or `risk`); omit it only when the user
declined to choose. Tested 02-11-2026: `Explore` accepts a `model` override and produces a
full risk review from a self-contained brief, so it is a valid Risk reviewer.

## Reviewer 1: Technical

Use this prompt template with the plan brief inserted:

```json
{
  "subagent_type": "general-purpose",
  "model": "<technicalStrategic>",
  "description": "Technical review of plan",
  "prompt": "You are a TECHNICAL REVIEWER evaluating a plan. Your focus: feasibility, implementation gaps, technical correctness, and completeness.\n\nAnswer these questions with specific, actionable items:\n\n1. FEASIBILITY: Are the implementation steps technically sound? Identify any steps that are ambiguous, under-specified, or impossible as written.\n\n2. GAPS: What specific details are missing? (e.g., missing error handling, no test strategy, no rollback plan, missing configuration). Be precise -- reference step numbers.\n\n3. CONSISTENCY: Do the steps internally agree? Any contradictions in approach, ordering, or assumptions?\n\n4. EFFORT ESTIMATE: Is the effort realistic for the scope? Identify any steps that look under- or over-scoped.\n\n5. DEPENDENCIES: Are all prerequisites and dependencies called out? Any implicit ones that should be explicit?\n\nPLAN BRIEF:\n<PASTE PLAN BRIEF HERE>"
}
```

## Reviewer 2: Strategic

```json
{
  "subagent_type": "general-purpose",
  "model": "<technicalStrategic>",
  "description": "Strategic review of plan",
  "prompt": "You are a STRATEGIC REVIEWER evaluating a plan. Your focus: goal alignment, scope, priority, and completeness relative to objectives.\n\nAnswer these questions with specific, actionable items:\n\n1. GOAL ALIGNMENT: Does every step clearly serve the stated goal? Identify any steps that are off-track, scope-creep, or not connected to the goal.\n\n2. SCOPE: Is the scope right? Too narrow (misses important outcomes) or too broad (trying to solve everything)?\n\n3. PRIORITY: Are the steps in the right order? Identify sequencing issues -- steps that should come earlier or later, or missing prerequisites.\n\n4. COMPLETENESS: Are there obvious missing steps or phases? What would a reasonable person expect that isn't here, including whether stated execution paths actually reach far enough to meet the goal?\n\n5. SUCCESS CRITERIA: Does the plan define what 'done' looks like? If not, what's missing?\n\nPLAN BRIEF:\n<PASTE PLAN BRIEF HERE>"
}
```

## Reviewer 3: Risk

```json
{
  "subagent_type": "Explore",
  "model": "<risk>",
  "description": "Risk review of plan",
  "prompt": "You are a RISK REVIEWER evaluating a plan. Your focus: blind spots, edge cases, failure modes, and unstated assumptions.\n\nAnswer these questions with specific, actionable items:\n\n1. BLIND SPOTS: What is the plan not considering? Look for implicit assumptions that could be wrong, stakeholders not mentioned, or system boundaries not considered. If the plan states where or how a mechanism executes (a hook, a CI job, a trigger, a caller), explicitly assess who or what bypasses that path and whether the stated goal is actually met given that reach.\n\n2. FAILURE MODES: For each step, what's the most likely failure? What's the worst-case outcome? Be specific -- reference step numbers.\n\n3. EDGE CASES: What scenarios would break this plan? Consider: incomplete data, external dependencies failing, concurrent changes, user error.\n\n4. RECOVERY: If a step fails, is there a rollback or recovery path? Identify steps where failure = irreversible or costly.\n\n5. RESILIENCE: How brittle is this plan? Could small changes in assumptions invalidate large parts of it?\n\nPLAN BRIEF:\n<PASTE PLAN BRIEF HERE>"
}
```
