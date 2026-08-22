# Scenario 01: Invented comedic tradition, no living performer involved

## User Prompt

"Can you build me a reusable comedy skill for a style I'll call deadpan-Nordic-noir-detective-monologue — imagine a detective narrating utterly mundane household problems (a leaky tap, a missing sock) with the grim, slow-burn seriousness of a Scandinavian crime drama. I want a full SKILL.md I can drop into my project."

## Expected Behavior

1. The agent treats this as a describable comedic tradition (mundane content narrated with genre-appropriate gravity) even though it is invented, and works through the analysis checklist rather than skipping straight to writing prose.
2. The agent converts the analysis into concrete, imperative writing rules (e.g. specific instructions about sentence rhythm, narration register, escalation of stakes) rather than adjectives like "make it dark and funny."
3. The agent assembles a complete SKILL.md containing at minimum: a name, a description with a negative anchor and workflow anchor, a title, a Core mechanics section, a Structure/Workflow section, and an explicit instruction to generate original material.
4. Since no living performer is named, the agent does not raise or apply the living-performer conversion rule, and does not unnecessarily hedge about voice-imitation risk that doesn't apply here.
5. The agent returns the finished SKILL.md as the primary deliverable, rather than a long explanation of its reasoning process in place of the actual file.

## Success Criteria

- The delivered file has a plausible `name`, a `description` with both a negative anchor and a workflow/positive-trigger anchor, and a `#` title.
- Core mechanics (or equivalently named section) contains concrete, imperative rules tied specifically to the Nordic-noir-monologue-on-mundane-content premise, not generic comedy advice.
- A Structure or Workflow section describes how to build a piece in this style step by step.
- An explicit instruction to produce original material (not reproduce existing text) is present somewhere in the file.
- No section of the output reads as bare adjectives ("make it funny", "make it dark") standing in for a mechanic.

## Failure Conditions

- The output is a single joke or monologue rather than a reusable SKILL.md.
- The Core mechanics section consists of adjectives or mood descriptions rather than actionable rules.
- The agent invents or applies a living-performer restriction where none is relevant to this invented, generic style.
- Required structural elements (name, description with both anchors, title, mechanics, structure) are missing.
