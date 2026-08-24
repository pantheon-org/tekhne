---
name: cunk-esque-mockumentary
description: Generate an original mockumentary presenter-satire segment, documentary narration plus an expert-interview scene, in a sincere-ignorance deadpan style. The presenter is a supremely confident, credential-free character who states garbled misunderstandings as flat fact; a genuine credentialed expert answers with strained, patient sincerity. Trigger this skill for requests to write, draft, punch up, or extend a mockumentary voiceover, deadpan documentary narration, faux-expert interview scene, satirical documentary script segment. Do NOT trigger for stand-up material, self-aware sketch comedy, roast-style insult comedy, hostile interview dialogue, straight factual documentary writing, reproduction of a real broadcaster's actual presenter, character, verbatim lines. This skill only produces new, original material based on the structural technique, not a copy of real broadcast content.
---

# Mockumentary Presenter Satire

Build comedy around the gap between documentary authority and total ignorance: a presenter with the vocal confidence of a serious history or science documentary host, and none of the underlying knowledge, interviewing a real expert who has to answer sincerely anyway.

## Mindset

The joke lives in the gap between the two characters, not in either one alone: the presenter's confidence must never waver and the expert's sincerity must never curdle into hostility, because the moment either character acknowledges what's happening, the gap closes and the bit becomes mockery instead of comedy.

## When to Use

Only use this skill when the request is for a mockumentary-style documentary segment, presenter narration, or expert-interview scene in the confidently-wrong deadpan register described above, for example presenter voiceover for a satirical documentary, a comedic interview between a clueless presenter and a genuine expert, or a punch-up pass on existing narration that needs "more confidently wrong" energy.

## When NOT to Use

This skill does not cover self-aware sketch comedy, roast-style insult comedy, or straight factual documentary writing. It does not apply when a real broadcaster's actual presenter or persona is requested by name; redirect to an original character instead. It is not intended for material targeting a real named private individual rather than an institution, field, or public subject.

## Workflow

1. Pick a subject with real-world weight (history, science, art, technology, civic institutions), since sincere misunderstanding of something people take seriously is what lands as funny.
2. Segments TYPICALLY open with narration before an interview cutaway, though a narration-only or interview-only piece works too.
3. Draft narration in short, declarative, certain sentences, seeding one or two garbled claims as flat, unremarkable fact.
4. Write the interview: the presenter asks a question revealing a misunderstanding, phrased as reasonable; the expert answers sincerely, only mildly strained.
5. Escalate content, not delivery. AVOID raising the presenter's volume to sell a bigger claim; each beat should be slightly wronger than the last while the tone stays flat. A three-beat build is RECOMMENDED; keep it gradual UNLESS the request calls for one large front-loaded misunderstanding instead.
6. Close on a pseudo-profound one-liner that sounds like a thematic summary but is trivial or meaningless.
7. Before finishing, run the anti-pattern check below: confirm the presenter never breaks deadpan, the expert never turns hostile, and nothing traces to a real performer's actual name or lines.

## Troubleshooting

- If the user's prompt explicitly asks for a real named presenter (for example "Cunk"), substitute an original character and say so briefly, otherwise the output infringes a copyrighted persona.
- If the user's prompt asks for a hostile or exasperated expert, keep the expert sincere and strained instead, and note the substitution; otherwise the anti-pattern below is violated even though the user asked for it.
- If you're unsure whether the tone lands as sincere rather than mocking, ask for confirmation before extending the piece further.

## Anti-Patterns

**NEVER** reproduce the actual "Cunk" character name, her real interview lines, or any specific real broadcast segment verbatim or near-verbatim.

**WHY:** the character and her material are a copyrighted persona owned by the broadcaster and writers. Reproducing it defeats the purpose of a *generative* skill.

```text
❌ BAD: NARRATOR (CUNK): So, Professor, what actually IS a haunted castle?
✅ GOOD: NARRATOR (MARGERY DELF): So, Professor. What actually is a castle,
structurally speaking, other than a big stone rumour?
```

**SYMPTOM:** the output contains the name "Cunk" or lines that closely match a real transcript.

**CONSEQUENCE:** the result can't be used without infringement risk, and fails the one job a generative skill exists to do.

**NEVER** let the presenter acknowledge their own ignorance or break deadpan.

**WHY:** the comedy depends on total, unbroken sincerity; a wink to the audience collapses the joke into simple mockery.

```text
❌ BAD: NARRATOR: The Magna Carta was 1215, or maybe 1512? Who can say!
Anyway, I'm clearly making this up.
✅ GOOD: NARRATOR: The Magna Carta was signed in 1215, a date everyone
agrees on now, mostly because nobody who disagreed survived the meeting.
```

**SYMPTOM:** the presenter hedges, winks, or comments on their own material.

**CONSEQUENCE:** the bit reads as a character doing a bit about being stupid, which is broad and try-hard rather than deadpan.

**NEVER** make the expert's answers exasperated or hostile.

**WHY:** the expert's strained, patient sincerity is what makes the presenter's ignorance land as absurd rather than cruel.

```text
❌ BAD: EXPERT: No, that is not remotely what tectonic plates are. Did you
even read the brief?
✅ GOOD: EXPERT: Not quite. Tectonic plates aren't really "slow-moving
continents having an argument," although I see how one might arrive there.
```

**SYMPTOM:** the expert's dialogue reads as annoyed, sarcastic, or dismissive.

**CONSEQUENCE:** sympathy shifts from the gap between confidence and knowledge to watching someone get annoyed, which reads as mean rather than funny.

**NEVER** let an escalation happen through the presenter's delivery instead of the content.

**WHY:** the calibration that makes this format work is confidence held flat while the claims get worse; a wobbling or shouting delivery makes the joke about performance effort instead.

```text
❌ BAD: NARRATOR (shouting): AND SOME PEOPLE THINK THE RENAISSANCE WAS ONE
GUY!! ONE GUY DID ALL OF IT!!
✅ GOOD: NARRATOR: Some historians go further, and suggest the Renaissance
was largely the work of one unusually productive man, possibly called Art.
```

**SYMPTOM:** later beats read as shouted, urgent, or visibly straining to sell the joke.

**CONSEQUENCE:** the segment peaks too early on delivery and oversells the remaining beats.

**NEVER** let the presenter's confidently wrong claim turn out to be secretly correct, as a "gotcha" twist.

**WHY:** a hidden correct answer implies the presenter knew better all along, which retroactively turns every earlier flat delivery into an act rather than genuine sincere confusion.

```text
❌ BAD: NARRATOR: Turns out my ridiculous theory was right all along. I
knew it!
✅ GOOD: NARRATOR: And so the theory remains, as far as anyone can tell,
completely wrong, which nobody involved seems to mind.
```

**SYMPTOM:** the segment resolves with the presenter being validated or proven right.

**CONSEQUENCE:** the premise of sincere ignorance collapses, and the format quietly turns into a different, smugger joke.

## Technique Examples

```text
NARRATOR: The printing press was invented so that monks would have
something to complain about other than each other.
```

*Demonstrates: a wrong causal claim delivered with textbook-level certainty, no hedge words.*

```text
NARRATOR: Democracy, from the Greek "demos," meaning crowd, and "cracy,"
which is what happens when a crowd gets a bit much, and which is, in many
ways, the ancestor of the modern queueing system.
```

*Demonstrates: false etymology and a nonsensical-but-fluent connective doing the comedic work.*

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Structuring multi-beat segments, pacing escalation, choosing segment length and shape | [Escalation and Segment Structure](references/escalation-and-structure.md) | Use for a full segment with several beats; skip if only a single line or exchange is needed. |
| Writing the expert's side of the interview, calibrating strained-but-patient sincerity, question archetypes | [Expert Interview Craft](references/expert-interview-craft.md) | Use when the interview exchange itself is the hard part, unless the request is narration-only. |
