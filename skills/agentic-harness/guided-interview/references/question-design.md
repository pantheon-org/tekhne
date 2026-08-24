# Question Design — Building Options That Are Actually Options

Concrete options only help if they're genuinely distinct answers to the decision at hand. This
reference covers how to design them.

## The 3-4 rule

- **Fewer than 3** usually means the decision has been forced into a false binary. If you can only
  think of two options, ask "what would a third, meaningfully different, approach look like?"
  before presenting the question.
- **More than 4** overloads working memory and defeats the point of narrowing choices — split the
  question. A five-way decision is usually two decisions: one broad (which category), one narrow
  (which variant within that category).

## What makes an option "concrete"

A weak option is a bare label: `"Option A"`, `"Fast"`, `"Simple"`. A concrete option names the
actual choice and states its main implication in one short clause:

| Weak | Concrete |
|------|----------|
| "Cache in memory" | "In-process cache — fastest, but lost on every restart and not shared across instances" |
| "Cache externally" | "Shared cache (Redis) — survives restarts and scales across instances, adds a network hop" |
| "No caching" | "Skip caching for now — simplest to ship, revisit if latency becomes a problem" |

Each option should let the user predict the consequence of picking it without needing to ask a
follow-up question.

## Mutual exclusivity

Options should not overlap. If two options could both be true at once, they're not alternatives —
merge them into one option and ask a separate question about the other axis.

## The free-text path always stays open

Curated options exist to make answering fast, not to constrain the answer space to what you
predicted. Some interview tools (e.g. `AskUserQuestion`) add a free-text/"Other" path
automatically; when running an interview in plain chat instead, say so explicitly: "or describe
something else entirely."

Treat a free-text answer as a signal that your option set missed something — the next question
should incorporate what was actually said, not re-offer a similar curated set.

## Deciding when to stop asking

A useful check before drafting the next question: "could I write the recap right now without
guessing?" If yes, stop. Interviews that keep going past sufficiency read as ceremony, not rigor.
