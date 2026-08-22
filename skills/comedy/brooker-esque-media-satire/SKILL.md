---
name: brooker-esque-media-satire
description: Generate original, misanthropic media/technology satire in the Charlie-Brooker tradition — a cynical column, monologue, short story built from one extended grotesque metaphor, self-loathing narration, hyperbole-and-bathos, corporate-jargon translation, plus a near-future premise that escalates one believable step at a time toward a bleak, deflating ending. Use for a satirical takedown of a TV show, app, gadget, news format, or social-media trend, plus dystopian near-future short stories about a plausible technology/social mechanic. Do NOT use for mockumentary/character-voice sketches, deadpan two-hander political interviews, observational stand-up; also not for reproducing a real writer's actual column, script, episode — this skill only generates new material inspired by the technique, never a copy.
---

# Misanthropic Media & Technology Satire

Build comedy around a weary, contemptuous narrator dismantling media, technology, or culture through one sustained, escalating metaphor rather than a string of unrelated jokes: the output is new writing, never a reproduction of anyone else's material.

## Mindset

Keep the escalation obeying its own machine-like rules: a nightmare that follows believably from a small premise unsettles a reader; breaking that rule once turns the piece silly instead.

## When to Use

- Use this for a cynical column or monologue about a TV format, app, gadget, or news story.
- Use this for a Black-Mirror-style short story pushing one small, current mechanic into a nightmare.
- Use this to mock corporate jargon by quoting it and translating it plainly.

## When NOT to Use

- Skip this for mockumentary-style sketches built on character voices rather than narration.
- Skip this for deadpan interviews or observational stand-up — anything not a narrator dismantling a subject through metaphor.
- Do not use this to quote, closely paraphrase, or "tidy up" an existing real column, script, or episode.
- Keep hopeful or triumphant endings out of scope for this skill.

## Prerequisites

Before starting, have a mundane subject (a TV format, gadget, app, news story, or social trend) and a target form: rant/column/monologue, or escalating short story.

## Workflow

1. Define the subject — a mundane, recognisable format, gadget, app, or trend.
2. Set the form: follow the rant branch below for a column or monologue; otherwise follow the story branch for a short story.
3. Build the rant around one grotesque metaphor: extend it across elaborate sentences, then puncture with a short, blunt clause.
4. Build the story around one escalating cause-and-effect chain instead, one plausible consequence at a time.
5. Include self-directed contempt alongside contempt for the subject, weighted however suits the piece.
6. Add a jargon-mockery beat if the subject carries corporate language: quote it dryly, translate what it means.
7. Return to the centre near the end, worse, rather than introducing something new.
8. Keep the ending flat: close on a bleak note, without triumph.
9. Run a final check against the anti-patterns below to verify the draft before calling it done.

## Core Mechanics

PREFER a single scaling metaphor system (an illness, a production line, a court case) over switching images mid-piece. AVOID a second unrelated joke once the first metaphor lands, UNLESS the piece is deliberately fragmented.

```text
The app doesn't send notifications, it triages you: every ping is a small
paramedic arriving to say which part of your life is bleeding out fastest.

→ produces: one image (triage) followed mechanically, not switched mid-thought.
```

Follow the trivial-annoyance-first pattern for hyperbole and bathos together: describe it in apocalyptic terms, then undercut it with how small the problem really is.

```text
The buffering wheel spun for what felt like a geological era — and then
the video played, and it was a man reviewing a kettle.

→ creates: an apocalyptic build-up punctured by a small, flat payoff.
```

Test the escalation for logical jumps before finishing a story.

```bash
make check-draft FILE=draft.md
```

That command is illustrative only — this skill ships no such script.

## Anti-Patterns

**NEVER** reproduce Charlie Brooker's actual columns, Screenwipe/Newswipe scripts, Black Mirror episodes, or specific lines verbatim or near-verbatim.

**WHY:** it is copyrighted material, and reproducing it defeats a *generative* skill — the user gets a copy, not new satire.

**SYMPTOM:** a draft sentence could be traced back to a real broadcast column or episode with only a few words changed.

```text
❌ BAD — lightly reworded lift of a known line about a real talent show,
changing only a few nouns.

✅ GOOD — an entirely new grotesque metaphor for an invented streaming
competition, built from scratch.

→ produces: material that teaches the technique instead of copying a joke.
```

**CONSEQUENCE:** the output is a copyright liability disguised as a deliverable.

**NEVER** let the escalation break plausible cause-and-effect logic.

**WHY:** random surrealism reads as silly; a believable step reads as unsettling — that believability is the engine of the form.

**SYMPTOM:** a step in the chain needs a new capability or actor that nothing earlier set up.

```text
❌ BAD — "The fridge started ordering groceries. Then, for no reason, it
declared war on a neighbouring appliance."

✅ GOOD — "The fridge started ordering groceries. Then it began
substituting cheaper brands to hit a margin target in the terms of service."

→ produces: a step the reader can imagine a real company actually taking.
```

**CONSEQUENCE:** the piece stops being unsettling and becomes a shrug.

**NEVER** end on a triumphant or hopeful note.

**WHY:** the bleak ending — the narrator saw it coming and nothing was done — is the punchline; a hopeful ending undercuts the tone.

**SYMPTOM:** the final beat has a character learning something, fixing the situation, or feeling relief.

```text
❌ BAD — "She deleted the app, felt the sun on her face, and realised
she'd been free all along."

✅ GOOD — "She deleted the app. It reinstalled itself during a routine
update nobody remembers agreeing to."

→ produces: a last beat with no fix and no relief, which is the joke.
```

**CONSEQUENCE:** a hopeful last beat reads as a tonal U-turn, and the escalation stops paying off.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Extended-metaphor mechanics, narrator voice, jargon-translation technique in depth | [Extended Metaphor & Narrator Voice](references/extended-metaphor-and-voice.md) | Load only when writing a rant/column/monologue and the metaphor is drifting or self-contempt feels bolted on; skip it unless that form is in play |
| Structuring a full near-future escalation chain and landing a bleak close | [Dystopian Escalation Structure](references/dystopian-escalation-structure.md) | Load only when writing the short-story form and the escalation stalls, jumps illogically, or the ending drifts hopeful; not needed for a short rant |
