---
name: guided-interview
description: Conduct a structured, one-question-at-a-time interview to explore a specific topic or decision — each question offers 3-4 concrete, mutually exclusive options plus a free-text path, and later questions adapt to prior answers rather than following a fixed script. Ends with a recap the user confirms before finalizing, and only then produces a synthesized answer or (when the interview clearly feeds downstream work) a written brief. NOT for open-ended ambiguity-surfacing (use socratic-method) or single-fact lookups.
---

# Guided Interview — One Question, Concrete Options, Adaptive

Interviews fail two ways: dumping a wall of questions on the user at once, or asking open
questions that make the user do the work of drafting an answer from scratch. This skill fixes
both — ask one question, make each option a genuine, distinct answer the user can just pick, and
let every answer reshape what gets asked next.

## Mindset

Every question should be answerable in one short reply, not a paragraph. The interview is not a
checklist to complete — it is a chain of decisions where each answer changes the shape of the
next question. Treat "I don't know" or "other" as real signal, not a failure to plan for.

## Prerequisites

Applicable only when ALL of the following hold:

- The user asked to be interviewed, walked through a decision, or to "help me figure out" a
  specific, nameable topic.
- The topic has enough shape to generate concrete, meaningfully different options — not
  "tell me about your life" or anything too diffuse to enumerate.
- Interrupting the current task to interview is not worse than proceeding directly (the user
  isn't mid-emergency or explicitly time-constrained).

If any prerequisite is absent, skip this skill. For open-ended ambiguity where the problem itself
is unclear, use `socratic-method` instead — that skill probes assumptions with open questions;
this one narrows a known topic down to a decision via concrete choices.

## When to use

BY DEFAULT, reach for this skill whenever a topic has a real, enumerable decision space. PREFER a
direct answer instead when the ask is mechanical or the user is visibly time-constrained.
TYPICALLY, the whole interview resolves in 3-6 questions — consider that a rough budget, not a
hard cap.

- The user explicitly asks to be interviewed, "asked one question at a time", or offered options
- Requirements gathering for a plan, ticket, design doc, or brief where several discrete
  decisions must be made before work can start
- Exploring a decision space where the user has unstated preferences that surface faster through
  concrete choices than through open questions

## When NOT to use

- Purely factual questions with one right answer
- Requests that are already unambiguous with clear acceptance criteria
- Open-ended, assumption-surfacing work where the problem itself is undefined — use
  `socratic-method`
- The user has explicitly said "just decide for me" or "no more questions"

## The Interview Protocol

1. **Confirm scope in one line.** If the topic is already clear from the user's request, skip
   straight to the first question — do not ask a throat-clearing "what topic?" question.
2. **Ask exactly ONE question per turn.** Use the `AskUserQuestion` tool for a single question at
   a time. Never bundle multiple questions into one call just because the tool nominally supports
   up to four — batching is the default for quick clarifications, but an interview is explicitly
   the case where that default is wrong.
3. **Every question offers 3-4 concrete, mutually exclusive options**, each with a short
   description of what picking it implies or its main tradeoff. Never rely on a bare label alone.
   The free-text/"Other" path must always remain open — `AskUserQuestion` adds it automatically;
   if asking in plain chat instead, explicitly invite a written answer.
4. **Let the previous answer choose the next question.** Before drafting it, ask: does this
   answer make a planned question moot? Does it open a branch that needs a question you hadn't
   planned? Work adaptively — do not march through a fixed list irrespective of what's answered.
   AVOID re-offering a similar curated set right after a free-text answer; build the next set from
   what the user actually said instead.
5. **Stop as soon as you have enough to act.** Most interviews resolve in 3-6 questions. Check:
   "could I write the recap right now without guessing?" If yes, stop asking.
6. **Recap and confirm before finalizing.** Summarize every answer in a short bulleted list and
   ask for the user's confirmation, correcting anything they flag. Do not proceed past this point
   on an unconfirmed recap, UNLESS the user has explicitly waived it (e.g. "skip the recap, just
   tell me").
7. **Only after confirmation, produce the output.** Default to a synthesized answer or
   recommendation in chat. Write a file only when the interview is clearly feeding downstream
   work the user already asked for (e.g. a plan via `plan-create`, a brief, a ticket) — if it's
   unclear whether a file is wanted, ask as part of the recap rather than assuming.

## Rules of engagement

These are the anti-patterns that turn a crisp interview into an interrogation. The most persistent
pitfall — the one worth watching for even after the protocol feels automatic — is treating the
curated option list as exhaustive instead of a starting point.

### NEVER ask more than one question per turn

Even as a single `AskUserQuestion` call bundling several.

WHY: Multiple simultaneous questions force the user to hold several decisions in mind at once and
break the adaptive chain — the second question can no longer benefit from the first answer.

BAD: One `AskUserQuestion` call with four unrelated questions about scope, timeline, audience, and format.
GOOD: One question about scope; once answered, decide what to ask next based on that answer.

### NEVER offer fewer than 3 or more than 4 options

On a question that has a real decision space.

WHY: Two options force a false binary when more genuine paths exist; five or more overloads the
user's working memory. If a decision space needs more than 4 branches, split it into two
sequential questions instead.

BAD: "Fast or slow?" when three genuinely different approaches exist.
GOOD: Three or four options, each a distinct, real answer — plus the automatic free-text path.

### NEVER hide or omit the free-text path

WHY: Concrete options speed up the common case, but the user's actual answer may not be listed.
Forcing a pick from a closed set produces false signal.

BAD: Presenting only the 3-4 curated options as if they were exhaustive.
GOOD: Curated options for speed, with "Other" always available and genuinely considered.

### NEVER keep asking after the user signals they're done

A clear "here's my answer, let's move on" or "just decide" signal ends the questioning.

WHY: Persisting past an explicit override wastes the user's time and reads as not listening.

BAD: Asking another branching question after the user says "just pick the sensible default."
GOOD: Acknowledge, synthesize with reasonable defaults, and move to the recap.

### NEVER let the recap silently drop, merge, or reinterpret an answer

WHY: The recap is the user's only checkpoint before the interview's output is produced — if it
misrepresents an answer, the final output will be built on the wrong premise.

BAD: Summarizing "you wanted the simple option" when the user actually picked a specific named one.
GOOD: Recap each answer close to verbatim, then ask "did I get all of that right?"

### NEVER produce a written file the user did not ask for

Unless it's clearly needed by downstream work.

WHY: Defaulting to files nobody asked for creates clutter and implies more ceremony than the
interview warranted.

BAD: Writing a brief document after a quick chat-scoped decision with no follow-on task.
GOOD: Default to a chat answer; write a file only when the interview is visibly feeding a plan,
ticket, or other artifact the user already asked for — or ask if it's unclear.

## Troubleshooting

| Situation | Response |
|-----------|----------|
| User picks "Other" repeatedly | Your curated options aren't matching their mental model. The RECOMMENDED fix is only when a second free-text answer confirms the pattern: stop curating and ask an open question instead |
| User wants to skip a question | Skip it, note the gap in the recap, and adapt later questions to not depend on it |
| A new answer contradicts an earlier one | Surface the contradiction plainly and ask which one holds, rather than silently picking one |
| Topic turns out broader than expected mid-interview | Pause, state the new scope in one line, and confirm before continuing with adapted questions |
| User says "just decide for me" | Stop questioning, state the assumptions you'll use as defaults, and move straight to the recap |
| You can't generate 3 genuinely distinct options | The question may be too granular or premature — merge it into a broader question or defer it |

## Example opening

When this skill is active, begin with:

> I'll ask a few questions one at a time, each with a few options to pick from (or write your own
> answer). Let's start.
>
> [First question, via `AskUserQuestion`, 3-4 options]

## Quick diagnostic

Run this check before opening the interview, to confirm the topic actually has a decision space:

```bash
# Returns "interview" or "answer-directly" based on whether the request names a real choice
# skip if the prompt already names one specific, unambiguous fact to look up
echo "$USER_PROMPT" | head -c 200
```

→ Expected output: a short excerpt you can eyeball for a decision word ("choose", "which",
"strategy", "approach"). If it reads as a decision, open the interview. If it reads as a single
fact, answer directly instead.

## Verification

- Was exactly one question asked per turn, with no bundled multi-question calls?
- Did every question offer 3-4 genuinely distinct options plus an open free-text path?
- Do later questions show evidence of branching from earlier answers, rather than working through
  a fixed list regardless of what was said?
- Was a recap presented and explicitly confirmed before any output was finalized?
- If a file was produced, was it because the interview was clearly feeding work the user already
  requested — not produced by default?
- If a file was produced, verify the output file exists and matches the confirmed recap before
  reporting the interview complete.

## References

Detailed supporting material lives in `references/`:

- [`question-design.md`](references/question-design.md) — the 3-4 rule, what makes an option
  concrete, and when to stop asking
- [`worked-example.md`](references/worked-example.md) — a fully annotated interview from opening
  question through recap and confirmation
