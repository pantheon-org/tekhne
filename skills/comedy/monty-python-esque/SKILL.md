---
name: monty-python-esque
description: Generate an original British surrealist comedy sketch, script, or comic bit in the Monty Python tradition, built on absurd literalism, escalating internally-consistent illogic, bureaucratic pedantry, semantic argument, deadpan seriousness amid impossible premises, sudden non sequiturs, abrupt anticlimactic endings. Use this when a user asks for a new sketch, script, dialogue bit, or scene built on a mundane premise pushed to logical extremes. Do NOT use this to reproduce, quote, paraphrase, reword any actual Monty Python sketch, character, catchphrase (e.g. Dead Parrot, Spanish Inquisition, Ministry of Silly Walks, Argument Clinic); this is a generative skill for fresh original material, not a lookup, quotation, transcription tool. Not for slapstick, observational stand-up, roast comedy, American-style sitcom writing, where the humour depends on delivery, timing, rather than an escalating internally-consistent premise.
---

# Monty Python-esque Comedy

## Mindset

The comedy in this tradition is not randomness dressed up as "weird": it is absolute internal
consistency applied to an impossible premise. A sketch is funny because the characters treat the
absurd with the same procedural seriousness they'd give a tax return, and the escalation follows
rules the audience can half-predict a beat before the logic breaks something. If you remember one
thing: escalate the logic, don't escalate the randomness.

## When to Use

Only use this skill when the request is for new, original material built on an ordinary premise
pushed to absurd extremes, not for retrieving or lightly disguising existing content.

- The user asks for an original sketch, script, or short comic bit ("write me a sketch about...",
  "give me a scene where...", "a bit involving...").
- The premise is ordinary (a council meeting, a customer complaint, a job interview, a queue), and
  the user wants it pushed to absurd extremes through dialogue.
- The user wants dry, escalating, deadpan-toned comedy rather than punchline-driven one-liners.
- The user references Monty Python's style or tradition as a tone descriptor, not as a request to
  retrieve or adapt specific existing material.

## When NOT to Use

- The user asks for the actual text of an existing Monty Python sketch, or a "rewrite"/"parody" of
  one that keeps its characters, structure, or catchphrases recognisable. If the user wants to
  reuse recognisable characters or catchphrases, do not attempt a lightly disguised version;
  offer new, premise-driven material inspired by the same mechanism instead (see Anti-Patterns).
- The user wants slapstick, physical-comedy description, observational stand-up material, roast
  jokes, or punchline-per-line sitcom banter: those rely on delivery and timing, not on escalating
  premise logic, and this skill's techniques will not fit the brief.
- The user wants comedy about a real, named, identifiable person in a way that could read as
  mockery of them specifically rather than of the absurd situation.

## Workflow

1. Use an ordinary, recognisable premise: a planning committee, a returns desk, a job interview,
   a bus queue, a parish newsletter.
2. Apply absurd literal precision: take a phrase or rule in the premise and interpret it with
   total, humourless literalness rather than its intended meaning.
3. Keep deadpan tone from the first line; consider giving at least one character genuine
   bureaucratic calm no matter how strange things get.
4. Set a comic engine for the escalation: bureaucratic procedure, pedantry over definitions,
   semantic argument, and pointless officialdom all work, and you might blend two. **AVOID**
   switching engines mid-scene unless the premise clearly calls for it.
5. Follow one escalating rule through consistent internal logic; ensure each new absurdity
   follows from the last one by the sketch's own rules, not at random. You may mix pompous,
   formal register with the trivial subject matter to sharpen the contrast.
6. Add a non sequitur or reversal once the escalation has built momentum, if it sharpens the
   joke; it's optional, not required for every sketch.
7. Avoid narrating the joke. **PREFER** letting the absurdity speak for itself over announcing
   "this is ridiculous" directly.
8. Close on a dry anticlimax, an absurd non-conclusion, or a sudden cut. The ending must not
   explain or resolve the escalation.
9. If the escalation fails to build past the opening exchange, stop and revisit the premise, or
   fall back to a different comic engine (see References). If the sketch touches a real,
   identifiable person or a sensitive topic, ask for confirmation before sharing it further.

## Anti-Patterns

### NEVER reproduce actual Monty Python sketch content

**NEVER** reproduce actual Monty Python sketch dialogue, scene structure, or character names
(named sketches or characters associated with the troupe) verbatim or near-verbatim, even if the
user asks for a "version of" or "sketch like" a specific one.

**WHY:** it is copyrighted material, and reproducing it defeats the point of a generative skill:
the user gets a copy of something that already exists, not new comedy.

**CONSEQUENCE:** the output is not usable by the user (copyright exposure), and the request that
prompted the skill goes unfulfilled.

```text
❌ BAD
CUSTOMER: This parrot is dead.
SHOPKEEPER: No no, it's resting.
```

```text
✅ GOOD
CUSTOMER: This carpet is definitely diagonal.
FITTER: It's not diagonal, madam. It's had a long journey and it's tired.
```
*Same comic shape (denial in the face of the obvious), zero reused names, lines, or premise.*

### NEVER label something "surreal" without escalating through consistent internal logic

**NEVER** just declare a scene "surreal" or "random" and drop in unrelated strange events without
each one following from the sketch's own established rules.

**WHY:** randomness without internal logic reads as confusing, not funny: the comedy comes from
an absurd premise followed through with total consistency, not from novelty for its own sake.

**CONSEQUENCE:** the audience can't track why anything is happening, so nothing lands as a joke;
the scene reads as noise rather than escalation.

```text
❌ BAD
CLERK: You'll need form 7B.
[A giraffe walks in wearing a hat. CLERK ignores it. A phone rings underwater.]
CLERK: Anyway, sign here.
```

```text
✅ GOOD
CLERK: You'll need form 7B.
APPLICANT: I filled in form 7B.
CLERK: Then you'll need form 7B, in triplicate, to certify that you filled in form 7B.
APPLICANT: Isn't that the same form?
CLERK: Only once it's been countersigned by someone who has never seen form 7B.
```
*Every escalation is a direct, logical (if maddening) consequence of the rule before it.*

### NEVER let a character break character to signal "this is a joke"

**NEVER** have a character wink at the audience, comment on the absurdity, or otherwise step out
of the scene's own deadpan reality to flag that a joke is happening.

**WHY:** it kills the deadpan effect that makes the escalation land: the comedy depends on the
characters being the only people in the room who don't find this strange.

**CONSEQUENCE:** the joke is pre-explained rather than discovered, so the escalation you built in
step 5 of the Workflow loses its payoff.

```text
❌ BAD
INSPECTOR: Your licence is missing a licence for the licence. [to camera] Bit much, isn't it?
```

```text
✅ GOOD
INSPECTOR: Your licence is missing a licence for the licence.
OWNER: I have that licence right here.
INSPECTOR: Yes, but is it licensed?
```
Demonstrates: the inspector stays entirely serious; the audience supplies the "bit much."

### NEVER resolve the escalation with a tidy explanation or moral

**NEVER** wrap up the escalating absurdity with a neat explanation, an apology, or a stated moral
("...and that's why you should always read the small print").

**WHY:** the tradition's endings work through abruptness or dry anticlimax, not closure: a tidy
resolution retroactively makes the absurdity feel like it needed justifying.

**CONSEQUENCE:** the ending undercuts the escalation instead of capping it, and the sketch reads
as a fable rather than a bit.

```text
❌ BAD
CLERK: ...and so, after six more forms, the applicant finally understood the importance of
paperwork, and left a wiser man.
```

```text
✅ GOOD
CLERK: You'll need to reapply for the right to reapply.
APPLICANT: That's insane.
CLERK: Sadly, "insane" requires its own form. Next!
```
*Cuts off on the escalation itself rather than resolving or explaining it.*

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Extending an escalation past 3-4 exchanges without it collapsing into randomness, and choosing a bureaucratic/pedantic comic engine | [Escalation and Bureaucratic Engines](references/escalation-and-bureaucratic-engines.md) | When a sketch's escalation is running out of steam after the opening exchanges, or when picking a comic engine, unless the premise already implies one obviously |
| Landing deadpan tone across multiple characters and choosing an ending (non sequitur, reversal, or anticlimax) | [Deadpan Delivery and Abrupt Endings](references/deadpan-delivery-and-abrupt-endings.md) | When a draft sketch's ending feels flat or unresolved in the wrong way, or when a character's dialogue reads as "trying to be funny" instead of deadpan |
