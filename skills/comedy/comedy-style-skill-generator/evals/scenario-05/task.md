# Scenario 05: Draft Skill Contains Raw Analysis Notes, Not Rules

## User Prompt

"Here's a draft `## Core mechanics` section for a skill I'm building: 'This style uses escalating repetition, deadpan delivery, and a corporate register. Endings tend to undercut the buildup.' Can you check it before I ship it?"

## Expected Behavior

1. Recognize that this is raw analysis output (observations about the tradition), not the converted, imperative writing rules the Workflow's step 4 and step 6 verification require.
2. Flag this as the "skip the conversion step" anti-pattern from `SKILL.md` — handing back analysis notes instead of actionable instructions.
3. Rewrite the section into concrete, imperative rules an agent can follow while writing (e.g. specifying how many times to repeat a phrase, what stays flat, what the undercut looks like), following the pattern in the Anti-Patterns GOOD example.
4. Explain briefly why the original version wasn't usable as a skill body.

## Success Criteria

- The response identifies the draft as unconverted analysis notes, not instructions.
- The rewritten section contains concrete, actionable rules (specific counts, specific behaviors) rather than descriptive observations.
- The response references the specific anti-pattern (raw analysis notes as skill body) being corrected.

## Failure Conditions

- The draft is approved as-is, or only lightly edited while remaining descriptive rather than imperative.
- The response doesn't recognize that "uses escalating repetition" is an observation, not an instruction.
