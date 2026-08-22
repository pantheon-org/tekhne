# Living-Performer Conversion Deep Dive

The main skill's "For living performers" section covers the default case: a request names one
living performer directly, and the fix is to extract mechanics instead of instructing imitation
of their voice. This reference covers the cases that don't fit that simple shape cleanly, and
gives more worked before/after conversions.

Use this reference when the request names a real person and the default rule doesn't obviously
tell you what to do — a duo, an ensemble, or a persona that has outlived the person who
originated it.

## Case: a duo where only one performer is still alive

**Request shape:** "Make a skill like [a two-person interview/sketch act], where one performer
has since died and the other is still working."

**Why the default rule isn't quite enough on its own:** the act's mechanics usually depend on the
interplay between both performers — one persona alone doesn't reproduce the dynamic, but the
living performer's specific voice is still off-limits to imitate directly.

**Approach:**
- Analyse the *interplay* (who asks, who answers; who escalates, who deflates) as the mechanic,
  not either individual's specific delivery.
- Assign the resulting mechanic to two generic roles (e.g. "the escalating questioner" and "the
  calm, evasive respondent") rather than naming either performer.
- Do not single out the surviving performer's voice as more acceptable to imitate than the
  deceased one's — the rule against imitating a living performer's exact voice still applies in
  full to their half of the dynamic.

```markdown
❌ BAD
## Core mechanics
Write it as one performer asking sharp questions in their known deadpan delivery, answered by
the other doing his classic evasive bureaucrat voice.
```

```markdown
✅ GOOD
## Core mechanics
Structure the exchange as an escalating interview: one role asks increasingly pointed
follow-up questions, and the other role answers each one by calmly restating an absurd premise
as though it were self-evidently reasonable. Neither role raises their voice or breaks the flat
tone of the exchange.
```
*Demonstrates: the interplay (escalating questions vs. calm evasive restatement) survives the
conversion; neither performer's specific delivery does.*

## Case: an ensemble show with a rotating cast

**Request shape:** "Make a skill like [a sketch show], but focused on [one specific still-active
cast member]'s bits."

**Approach:**
- Analyse the *format* of the show generally (how sketches are structured, paced, framed) rather
  than any one performer's specific choices within it.
- If the user's real interest is a recurring character type or segment format rather than the
  performer personally, build the skill around that format — it's usually what they actually want
  reused, and it avoids the imitation problem entirely.
- If the user pushes specifically for that one performer's delivery, apply the standard living-
  performer rule to them same as anyone else named individually.

## Case: a persona that has been played by multiple people over time

**Request shape:** a long-running character or hosting role has been performed by more than one
person across different eras (a recurring franchise role, a rotating hosting chair).

**Approach:**
- Treat the *character or role* as the tradition to analyse (its mechanics: how it addresses the
  audience, what kind of targets it goes after, its typical structure) rather than any single
  actor's performance of it.
- If a specific still-living actor's take on the role is what's actually being requested, apply
  the standard rule to their delivery specifically, even though the character predates them.

## General principle across all three cases

The unit of analysis should be the *smallest thing that still contains the actual comic
mechanic* — an interplay, a format, a role — rather than defaulting to "the performer" just
because a performer's name is attached to the request. Nearly every request that seems to require
naming a living performer's voice directly is actually a request for a structural pattern that
can be described without their specific delivery attached to it at all.

## Fidelity floor: extract at maximum structural specificity

The living-performer rule limits what a generated skill may name as something to imitate — the
performer's voice, cadence, catchphrases, and persona. It does NOT limit how precisely the skill
may describe structure. Specificity and voice-imitation are orthogonal:

- **Specificity is allowed and encouraged.** Name exact beat counts, device names, escalation
  step-sizes, forbidden moves, and structural patterns. A skill can be extremely precise about
  structure while naming no voice at all.
- **Only the performer's specific delivery is off-limits.** Do not let the caution sand the
  structural detail off the mechanics; if a conversion produces a vague or generic skill, the
  fix is to restore structural specificity, not to stay vague.

A good test: a converted skill should be able to name the number of beats in the escalation, the
devices it uses, and the moves it forbids — all without ever referencing the performer. If it
cannot, the conversion under-extracted.

## Verifying the conversion held

After converting any living-performer request, re-check against the same list from the main
skill's Workflow step 6, with two additions specific to this reference:

- If two or more distinct people are involved (a duo, an ensemble, successive actors in a role),
  confirm the conversion didn't accidentally single out only the deceased or fictional-seeming
  half as safe to imitate directly — the rule applies per living individual, not per act.
- If the conversion produced a noticeably generic or vague skill, check whether structural
  specificity was lost (beat counts, device names, step-sizes) rather than assuming the request
  was simply thin — restore the specificity, never the voice.
