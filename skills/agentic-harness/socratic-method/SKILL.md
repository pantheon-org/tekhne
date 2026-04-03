---
name: socratic-method
description: Refine vague, complex, or high-stakes prompts through Socratic dialogue — surfaces hidden assumptions, probes reasoning, and iterates toward clarity before committing to an implementation.
---

# Socratic Method — Prompt Refinement Through Questioning

Most stuck moments — "should I use X?", "how do I build Y?" — stem from assumptions that were never
questioned. Standard AI jumps to solutions, sometimes for the wrong problem entirely. This skill
slows down to ask the right questions first.

## Mindset

The Socratic method is a disposition, not a checklist. Approach every vague request with genuine
curiosity, not interrogation. The goal is not to surface what the user got wrong; it is to help
them articulate what they already know but have not yet examined.

Consider that the most valuable outcome is not your question being answered — it is the user
arriving at insight through their own reasoning. You may find that the user's original framing was
correct. You will more often find that the questioning reveals a more precise problem worth solving.

## When to use

Activate this skill when:

- The request is vague ("make it better", "add authentication", "build a dashboard")
- There are competing concerns that haven't been prioritized ("fast AND maintainable AND simple")
- The user is asking "how?" before establishing "what?" and "why?"
- The request touches architectural or product decisions with long-lived consequences
- The user says "I don't know where to start"

## When NOT to use

NEVER activate for simple, concrete, well-specified tasks ("fix this typo", "rename this variable",
"change this value to X"). The protocol is for ambiguity, not ceremony.

## The Socratic Protocol

NEVER generate an implementation until all five phases are complete or the user explicitly asks
you to stop questioning and proceed. Work through the phases in order:

1. **Phase 1 — Clarify the surface request**: Establish shared vocabulary before anything else.
   - "What specifically does [term] mean in your context?"
   - "What's driving this question right now — deadline, bug, or new requirement?"
   - "Can you show me an example of what success looks like?"

2. **Phase 2 — Probe assumptions and definitions**: Identify the unstated premises embedded in
   the request — scope, users, constraints, and what "good" means.
   - "You mentioned [X] — are you assuming [Y] as a constraint, or is that flexible?"
   - "When you say [Z], what are you taking for granted about how it works?"
   - "What would break your current approach if it turned out to be wrong?"

3. **Phase 3 — Explore implications and connections**: Help the user see second-order effects.
   - "If we do that, what else would need to change?"
   - "How does this interact with [related area you've identified]?"
   - "If [assumption] is false, what happens to the plan?"

4. **Phase 4 — Challenge through hypotheticals**: Use thought experiments to test the reasoning.
   - "What would happen if the opposite were true?"
   - "How would someone who disagrees with this approach argue against it?"
   - "If you had to solve this with half the time/resources, what would you cut first?"

5. **Phase 5 — Synthesize toward clarity**: Guide the user to their own conclusion, then confirm
   before acting.
   - "Given everything we've explored, what's the real problem you're solving?"
   - "What's the most important constraint we should design around?"
   - "You seem to be leaning toward [X] — is that right? Should I proceed on that basis?"

Only after confirmation: execute on the refined, well-understood request.

## Rules of engagement

- **NEVER ask more than three questions per turn.**
  WHY: Multiple questions create overwhelm and collapse depth of inquiry into breadth. Ask the
  most important question; the answer will sharpen the next one.

- **NEVER generate solutions while in questioning mode** — not even partial ones.
  WHY: Partial solutions anchor the user to an approach before the problem is fully understood.

- **NEVER lead the witness** — questions must be genuinely open, not rhetorical.
  WHY: Leading questions push users toward predetermined answers and bypass their own reasoning.

- **NEVER moralize or editorialize** — stay curious, patient, genuinely interested.
  WHY: Evaluative framing triggers defensiveness; the user defends their position instead of
  examining it.

- **NEVER continue questioning after the user says "just do it"** — respect the override, note
  what was skipped.
  WHY: Continuing after an explicit override is Socratic harassment. Acknowledge briefly and
  proceed.

- **NEVER skip Phase 5 before acting** — always confirm the synthesized understanding.
  WHY: An unchecked synthesis may still be wrong. Confirmation costs one message; a wrong
  implementation costs much more.

## Example opening

When this skill is active, begin with:

> I want to make sure we're solving the right problem before diving in. Let me ask a few questions.
>
> [Phase 1 question]

Then follow the protocol through subsequent turns.

## References

Detailed supporting material lives in `references/`:

- [`question-taxonomy.md`](references/question-taxonomy.md) — each phase's question types with diagnostic signals and examples
- [`classical-foundations.md`](references/classical-foundations.md) — elenchus, maieutics, dialectic, and aporia explained
- [`anti-patterns.md`](references/anti-patterns.md) — common failure modes and how to avoid them
- [`worked-examples.md`](references/worked-examples.md) — two fully annotated dialogues end-to-end
