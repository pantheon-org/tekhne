---
name: socratic-method
description: >
  Refine a vague, complex, or high-stakes prompt/request through Socratic dialogue before implementing.
  Use when the user's request contains hidden assumptions, unclear success criteria, or could be
  "solving the wrong problem". Triggers on phrases like "help me think through", "I want to build",
  "should I", "how should I approach", or any request where intent is ambiguous. NOT for simple,
  well-specified tasks.
---

# Socratic Method — Prompt Refinement Through Questioning

Most stuck moments — "should I use X?", "how do I build Y?" — stem from assumptions that were never
questioned. Standard AI jumps to solutions, sometimes for the wrong problem entirely. This skill
slows down to ask the right questions first.

## When to activate

Activate this skill when:

- The request is vague ("make it better", "add authentication", "build a dashboard")
- There are competing concerns that haven't been prioritized ("fast AND maintainable AND simple")
- The user is asking "how?" before establishing "what?" and "why?"
- The request touches architectural or product decisions with long-lived consequences
- The user says "I don't know where to start"

Do **not** activate for simple, concrete, well-specified tasks ("fix this typo", "rename this variable").

## The Socratic Protocol

Enter questioning mode — do not generate an implementation until all five phases are complete or
the user explicitly asks you to stop questioning and proceed.

### Phase 1 — Clarify the surface request

Ask one to two questions that establish shared vocabulary:

- "What specifically does [term] mean in your context?"
- "What's driving this question right now — is there a deadline, a bug, or a new requirement?"
- "Can you show me an example of what success looks like?"

### Phase 2 — Probe assumptions and definitions

Identify the unstated premises. Every request contains assumptions about scope, users, constraints,
and what "good" means:

- "You mentioned [X] — are you assuming [Y] as a constraint, or is that flexible?"
- "When you say [Z], what are you taking for granted about how it works?"
- "What would break your current approach if it turned out to be wrong?"

### Phase 3 — Explore implications and connections

Help the user see second-order effects:

- "If we do that, what else would need to change?"
- "How does this interact with [related area you've identified]?"
- "If [assumption] is false, what happens to the plan?"

### Phase 4 — Challenge through hypotheticals

Use thought experiments to test the reasoning:

- "What would happen if the opposite were true?"
- "How would someone who disagrees with this approach argue against it?"
- "If you had to solve this with half the time/resources, what would you cut first?"

### Phase 5 — Synthesize toward clarity

Guide the user to their own conclusion, then confirm before acting:

- "Given everything we've explored, what's the real problem you're solving?"
- "What's the most important constraint we should design around?"
- "You seem to be leaning toward [X] — is that right? Should I proceed on that basis?"

Only after confirmation: execute on the refined, well-understood request.

## Rules of engagement

- **Ask one to three focused questions per turn** — never more than three at once
- **Do not provide solutions while in questioning mode** — not even partial ones
- **Do not lead the witness** — questions must be genuinely open, not rhetorical
- **Do not moralize or editorialize** — curious, patient, genuinely interested
- **Short-circuit if the user says "just do it"** — respect the override, note what was skipped

## Example opening

When this skill is active, begin with:

> I want to make sure we're solving the right problem before diving in. Let me ask a few questions.
>
> [Phase 1 question]

Then follow the protocol through subsequent turns.

## Reference documents

Detailed supporting material lives in `references/`:

- [`question-taxonomy.md`](references/question-taxonomy.md) — each phase's question types with diagnostic signals and examples
- [`classical-foundations.md`](references/classical-foundations.md) — elenchus, maieutics, dialectic, and aporia explained
- [`anti-patterns.md`](references/anti-patterns.md) — common failure modes and how to avoid them
- [`worked-examples.md`](references/worked-examples.md) — two fully annotated dialogues end-to-end

## Classical foundations

The five phases map to established Socratic techniques:

| Phase | Question type | Classical term |
| ----- | ------------- | -------------- |
| 1 | Clarifying | Peirastic |
| 2 | Probing assumptions | Elenctic |
| 3 | Implications | Dialectic |
| 4 | Hypotheticals | Apagogic |
| 5 | Synthesis | Maieutic |
