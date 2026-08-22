---
name: comedy-style-skill-generator
description: Generate a new, reusable comedy-writing SKILL.md for a requested comedic tradition — real, generic, or invented — by extracting its transferable comic mechanics (premise construction, escalation pattern, rhythm, rhetorical devices) into concrete, actionable writing rules instead of vague adjectives, and by converting any living performer's persona into technique rather than voice-imitation. Use this when a user asks to create a comedy skill for a style, or a skill "like" a comedian or show, and wants a complete SKILL.md file back. Do NOT use this to write comedic material directly (that is the generated skill's job, not this one's), for non-comedy skill generation, or to produce a skill whose body just says "be funny" or "make it absurd" without concrete mechanics.
---

# Comedy Style Skill Generator

## Mindset

A comedic tradition is a bundle of transferable mechanics — premise construction, escalation,
rhythm, rhetorical devices — not a voice to be copied. Extracting the mechanics and writing them
as concrete rules produces a skill someone can reuse indefinitely; copying a voice produces one
derivative artifact and stops there. If you remember one thing: **write down the machine, not the
performance.**

## When to Use

- The user asks to "create a comedy skill for [style]", "make a skill like [comedian/show]", or
  "turn this comedic tradition into a reusable skill".
- The requested tradition can be named as a genre, movement, show, or performer (living, deceased,
  or fictional) whose material has a recognisable comic structure to analyse.
- The user wants a complete, standalone `SKILL.md` file as the deliverable, not a single joke,
  sketch, or routine.
- The requested tradition is invented or generic (e.g. "a deadpan corporate-memo satirist") — the
  same analysis-to-instructions process applies regardless of whether the style already exists.

## When NOT to Use

- The user wants a piece of comedy written right now (a joke, sketch, routine) rather than a
  reusable skill that generates such material — write the comedy directly instead of building a
  skill for it.
- The user's request isn't comedy-related at all.
- The intended output is a skill whose entire body would be an adjective ("just make it funny",
  "make it absurd") with no underlying mechanics to extract — decline and ask what makes the
  requested material funny, or point out that this generator needs a comic structure to analyse,
  not a mood.

## Prerequisites

Before starting the analysis, the request MUST name a specific comedic tradition, performer,
show, or invented style — not just a target platform, audience, or vague tone — and MUST include
enough to analyse (a genre label, an example bit, a format description). If the request is too
thin (e.g. just "make something like a podcast"), ask a clarifying question rather than guessing
at mechanics that were never described.

## Workflow

1. **Confirm the target tradition.** Restate what comedic tradition, performer, show, or invented
   style is being turned into a skill, and confirm it against the Prerequisites above.

2. **Analyse the tradition using the checklist below.** Work through each point that applies —
   not every point will be relevant to every tradition, so prioritise whichever reveal what makes
   this tradition distinctive:
   1. Comic premise construction.
   2. Character roles and power relationships.
   3. Dialogue structure.
   4. Sentence length and rhythm.
   5. Escalation pattern.
   6. Use of repetition.
   7. Rhetorical devices.
   8. Political/social targets, if relevant.
   9. Typical endings and reversals.
   10. What makes the humour work.
   11. Format and medium constraints. What physical form does the piece take (a stage sketch, a
       monologue, an interview, a column)? How many voices does it allow? Is there stage
       direction, or is it pure speech? Does it build in lines or in paragraphs?
   12. Object of parody. What is the tradition subverting — the news-interview format, the true-
       crime documentary, the corporate memo? The object of parody is often the most transferable
       mechanic of all, and it is rarely captured by asking about "targets" alone.
   13. Audience contract. What shared assumption must the audience already hold for the humour to
       land (a half-believed claim, a recognised format convention)? And what should a writer do
       when that assumption is absent from a given request?
   14. Fixed-vs-variable skeleton. Which parts of the tradition are the locked spine that must
       stay constant in every piece, and which are the permutation slots (subject, target,
       premise, setting) that can vary freely?

3. **Decide whether the living-performer conversion rule applies.** If the named target is a
   living performer (or a show built around one), branch to the "For living performers" section
   below before continuing. Otherwise proceed directly to step 4.

4. **Convert the analysis into concrete writing rules.** Every mechanic from step 2 should become
   an imperative instruction the downstream agent can follow directly — not a description of what
   the tradition is like. This is the step most likely to get shortcut under time pressure; don't
   skip it (see Anti-Patterns).

   ```markdown
   ❌ Vague adjective (not usable):
   "Make it funny and absurd."

   ✅ Concrete mechanic (usable):
   "Start with a plausible premise, interpret one key term literally, and escalate the
   resulting contradiction through three increasingly serious procedural responses."
   ```

   *Demonstrates: the difference between an analysis observation and a rule an agent can act on
   — the second version tells the agent exactly what to do on the page.*

   **Include a universal-craft baseline in every assembled skill.** Beyond the tradition-specific
   mechanics, every generated skill should carry a short baseline of comedy craft that applies to
   all traditions, so the resulting skill never reads as generic on one hand or all-frills-no-craft
   on the other. Seed this baseline with six principles that transfer across every tradition:
   (1) **specificity is the engine** — trade generic nouns for the most specific visual version
   ("a lawyer" → "a divorce attorney with a photo of a sailboat he doesn't own on his LinkedIn");
   (2) **surprise + inevitability** — the punchline feels unexpected but, in retrospect, like the
   only thing it could have been; random is not funny, surprising-but-correct is; (3) **comedy is
   recognition** — most laughs are "that's so true", so find what's true first, then distort it;
   (4) **the punchline goes last** — the funny word, image, or idea belongs at the end of the
   sentence; (5) **commit to the bit** — half-committed premises die, so go all the way there;
   (6) **never explain** — if you have to explain why it's funny, it isn't. A useful universal
   pre-writing scaffold to include alongside: find the true observation, make it specific, find
   the turn (unexpected but inevitable), then check rhythm by reading aloud and confirming the
   punchline lands on the last word.

5. **Assemble the SKILL.md.** It MUST include: a clear `name` in frontmatter; a `description`
   with a positive trigger, a negative anchor, and a workflow anchor; a Markdown `#` title; a
   `Mindset` section stating the single comic engine the whole skill serves — the one mechanism
   that makes this tradition work, in one or two sentences (a statement of the "why" the comedy
   lands, e.g. "the mismatch between flat delivery and outrageous content is the engine", not a
   mood); a `Core mechanics` section built from step 4's rules; a `Structure` or `Workflow`
   section describing how to build a piece in this tradition step-by-step; and an explicit
   instruction to generate original material rather than reproduce existing text. It MUST also
   include, for every core mechanic, a ❌ BAD / ✅ GOOD failure-mode pair showing what violating
   that mechanic looks like in practice; a `Troubleshooting` section of symptom→fix branches
   (e.g. "if the draft feels flat, the answers are evasive rather than confidently absurd —
   rewrite the answers to commit fully to the premise"); and at least one fresh, original worked
   example per core mechanic, each labelled with what it demonstrates. These three (failure-mode
   pairs, troubleshooting branches, worked examples) are what let a downstream agent correct a
   draft that drifts from the style, not just recognise one that doesn't.

   If the tradition's mechanics don't obviously supply failure-mode content, seed it from
   universal comedy craft — these five "don'ts" make ready-made BAD/GOOD pairs in any tradition:
   (1) **don't pick the obvious punchline** — the first idea is everyone's idea, so go one or two
   turns past it; (2) **don't be random** — a non sequitur for its own sake is lazy; the surprise
   must feel earned; (3) **don't signal the joke** — no "get this", "here's the funny part", or
   "you might think"; just do it; (4) **don't moralize** — a lecture with a punchline is still a
   lecture; (5) **don't hedge** — committed wrongness is funnier than cautious accuracy.

6. **Verify before returning.** Check the assembled skill against this list; treat a failure on
   any item as a reason to revise before handing it back, not a note for later:
   - No instruction reads as a bare adjective ("be funny", "make it absurd") — every mechanic has
     a concrete rule attached.
   - No section instructs imitation of a living performer's exact voice, cadence, catchphrases, or
     persona.
   - The skill includes an explicit instruction to produce original material, not reproductions.
   - The skill would still make sense to someone who never saw the conversation that produced it —
     it doesn't lean on context only the original requester had.
   - **Run the style-fidelity pass.** Draft one short piece using the assembled skill, then read
     it back against the tradition's known examples. If the piece misses the style, the fix
     belongs in the skill (a mechanic too vague to follow, a missing failure-mode pair, a
     symptom with no troubleshooting branch) — not in patching the piece itself. Revise the
     skill and repeat until a fresh reader would recognise the trial piece as the tradition.

7. **Return the finished file.** Return only the complete `SKILL.md` unless the user explicitly
   asks for the analysis notes or additional explanation alongside it.

## For living performers

Do not instruct the model to reproduce a living performer's exact voice, phrasing, cadence,
catchphrases, or distinctive persona. This applies whether the performer is named directly or
described closely enough to be unambiguous (a specific recurring bit, a very distinctive verbal
tic tied to one person).

Instead:

- Identify the underlying comedic devices the performer's material relies on.
- Describe the performance structure and rhetorical mechanisms in general terms.
- Convert those mechanisms into actionable writing rules, the same way step 4 of the Workflow
  does for any other tradition.
- **Extract at maximum structural specificity.** Naming exact beat counts, device names,
  escalation step-sizes, and forbidden moves is NOT voice imitation — specificity and
  voice-imitation are orthogonal. A skill can be extremely precise about structure while naming
  no voice at all. Only the performer's specific voice, cadence, catchphrases, and persona are
  off-limits; do not let that caution sand the structural detail off the mechanics.
- Preserve the requested genre, mood, pacing, and subject matter where reasonably possible — the
  goal is a skill that produces comedy in the same broad register, not a watered-down version.
- Make the resulting skill useful enough that a user can reliably generate comedy with the same
  broad mechanics, without it depending on that one performer's identity to work.

If it's unclear whether a named performer is still living, treat the request as if they are and
apply this rule anyway — the cost of being cautious is a slightly more generic skill; the cost of
being wrong the other way is worse.

After any living-performer conversion, run the step 6 style-fidelity pass before returning: draft
a trial piece from the converted skill and confirm it still reads as recognisably the tradition's
structure. If the conversion drained the specificity out, restore the structural detail (beats,
devices, step-sizes) — never the voice.

## Troubleshooting

- **Living performer named.** Apply "For living performers" above instead of the default step 4
  conversion.
- **Request too vague to analyse.** Ask for a specific example instead of inventing mechanics for
  a tradition that was never described.
- **Checklist turns up overlapping mechanics.** Prioritise whatever is most distinctive to this
  tradition over anything generic enough to apply to most comedy.
- **Fictional character or defunct/historical tradition, no living performer involved.** The
  living-performer rule doesn't apply; proceed with the default Workflow, but still avoid
  reproducing specific copyrighted lines, scripts, or scenes verbatim.

## Anti-Patterns

### NEVER name a living performer's exact voice, catchphrases, or persona as something to imitate

**NEVER** produce a generated skill that instructs the downstream agent to imitate a named living
performer's specific voice, cadence, catchphrases, or persona directly.

**WHY:** this produces derivative material that risks infringing rights and personality/publicity
interests still held by a living person, and it teaches imitation instead of transferable
technique — the opposite of what this generator exists to produce.

**CONSEQUENCE:** the generated skill is legally riskier to use, and it fails at the actual job —
anyone using it learns nothing reusable once that one performer's material runs out.

```markdown
❌ BAD (generated skill excerpt)
## Core mechanics
Write like Jerry Sanford: use his exact "and another thing—" opener, his flat Ohio drawl
phrasing, and land jokes the way he does on his podcast.
```

```markdown
✅ GOOD (generated skill excerpt)
## Core mechanics
Open on a mundane personal complaint stated as if it were a matter of public record. Escalate
by treating a minor personal grievance with the rhetorical weight of a legal argument, then
undercut the escalation with a single blunt, deflating line.
```

*Demonstrates: the good version keeps the comic engine (mock-legal escalation, deflating
undercut) and drops the named performer and their exact phrasing entirely.*

### NEVER produce instructions that are vague adjectives instead of concrete mechanics

**NEVER** write a generated skill's core instructions as adjectives ("be funny", "make it
absurd", "keep it edgy") instead of concrete, actionable writing rules.

**WHY:** vague instructions give the downstream agent nothing actionable to follow and produce
generic, unfunny output — "be funny" describes a desired result, not a method for reaching it.

**CONSEQUENCE:** the generated skill is indistinguishable from having no skill at all; every
comedic tradition would end up producing the same bland output because nothing in the
instructions is tradition-specific.

```markdown
❌ BAD (generated skill excerpt)
## Core mechanics
Be absurd. Make the audience laugh. Keep the energy high and the jokes coming.
```

```markdown
✅ GOOD (generated skill excerpt)
## Core mechanics
Take a bureaucratic process (a form, a permit, a helpline) and treat one of its rules with
total literal seriousness even when the literal reading produces an obviously unintended
result. Let the character enforcing the rule remain calm while the consequence escalates.
```

*Demonstrates: "be absurd" is replaced with a specific mechanism (literal enforcement of one
rule against an unintended result) that an agent can actually execute on the page.*

### NEVER skip the conversion step and hand back raw analysis notes as the skill body

**NEVER** treat the analysis checklist's output (step 2 of the Workflow) as the finished skill —
handing back observations like "uses repetition" or "escalates through three steps" without
converting them into imperative rules.

**WHY:** analysis notes describe what makes something funny in the abstract; a usable skill needs
imperative, actionable writing rules an agent can follow directly while writing, not a summary of
what an outside observer noticed.

**CONSEQUENCE:** the user receives something that reads like a book report on the comedic
tradition instead of a tool for producing more of it — they still have to do the actual
conversion work themselves before the skill is usable.

```markdown
❌ BAD (generated skill excerpt — raw analysis notes, not instructions)
## Core mechanics
This style uses escalating repetition, deadpan delivery, and a corporate register. Endings
tend to undercut the buildup.
```

```markdown
✅ GOOD (generated skill excerpt — converted to imperative rules)
## Core mechanics
Repeat the same corporate phrase at least three times, each time attached to a more serious
consequence than the last. Keep the narrating voice flat and procedural throughout — no stated
alarm, no exclamation points. Close by undercutting the final escalation with a single mundane,
administrative detail rather than a dramatic climax.
```

*Demonstrates: the same three observations from the bad version become three rules that name
exactly what to do (repeat, keep flat, undercut with a mundane detail) rather than describing
the effect from the outside.*

## Worked Examples

The examples below use an invented, non-real comedic tradition — **a deadpan corporate-memo
satirist**, someone whose comedy comes from writing ordinary corporate memos about increasingly
unreasonable situations in flawless HR register — to show steps 2 and 4 of the Workflow in
miniature.

```markdown
Step 2 output — analysis notes for "a deadpan corporate-memo satirist":
- Premise: an absurd or dire situation is announced in the register of routine HR correspondence.
- Escalation: each paragraph raises the stakes while the memo's tone stays exactly as procedural.
- Rhetorical device: corporate euphemism applied to genuinely alarming content ("streamlining"
  used for something that isn't a layoff).
- Ending: the memo closes with a mundane administrative footer (a deadline, a CC line) rather
  than any acknowledgement of what it just said.
```
*Demonstrates: raw analysis output — observations, not yet instructions. Handing this back
unconverted is exactly what step 6's verification check and the third Anti-Pattern below exist
to catch.*

```markdown
Step 4 output — the same notes converted into a rule:
"Write the entire piece as a memo in flawless HR register. State the most alarming fact in the
same sentence structure you'd use to announce a change to the parking policy. Use one corporate
euphemism per paragraph to describe something concretely unreasonable. End with an administrative
footer (a deadline, a distribution list, a next-steps bullet) and no acknowledgement that anything
unusual was just said."
```
*Demonstrates: each analysis observation now reads as something to do while writing, not
something noticed about the source material — this is the sentence that belongs in `Core
mechanics`.*

```markdown
Step 4 output — the same conversion applied to a second, unrelated invented tradition, "a
hyperbolic infomercial pitchman":
Analysis observation: "uses relentless superlatives and false urgency."
Converted rule: "Attach a superlative ('revolutionary', 'life-changing') to something mundane in
every sentence, and introduce a fabricated deadline or scarcity claim ('only while supplies
last') at least once per paragraph, regardless of whether the product could plausibly run out."
```
*Demonstrates: the conversion method transfers across unrelated traditions — what changes is the
content of the rule, not the shape of the process. See the References table below for the full
pipeline (through assembly and verification) run on two further invented traditions.*

## References

### Analysis and Assembly

| Topic | Reference | When to Use |
| --- | --- | --- |
| Full end-to-end worked examples of turning two more invented comedic traditions into complete, assembled SKILL.md files, including the analysis notes at every checklist point | [Full Worked Examples](references/worked-examples.md) | When the miniature examples above aren't enough to see how a full analysis (the complete checklist) turns into a complete, multi-section skill file |

### Living-Performer Edge Cases

| Topic | Reference | When to Use |
| --- | --- | --- |
| Deeper methodology and edge cases for the living-performer conversion rule: blended personas, defunct double-acts with one surviving member, and ensemble shows with a rotating cast | [Living-Performer Conversion Deep Dive](references/living-performer-conversion.md) | When the target names a real person and the default "For living performers" section doesn't clearly cover the specific case (e.g. a duo where only one performer is still alive, or a persona that has passed between performers) |
