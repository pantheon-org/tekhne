# Full Worked Examples

This reference walks two invented comedic traditions all the way through the pipeline in
`SKILL.md`: analysis against the full checklist, conversion into concrete rules, and
assembly into a complete `SKILL.md`. Use it when the miniature examples in the main skill file
aren't enough to see the full shape of a real generation pass — in particular, how the full
checklist's points (not just two or three) end up distributed across a finished skill's
sections.

Both traditions below are invented for this reference; they are not based on a specific real
performer or show, so the "For living performers" conversion rule does not apply to either.

## Example 1: "a deadline-obsessed project-manager narrator"

The requested tradition: comedy narrated entirely in the voice of a project manager who responds
to every event in life — personal, romantic, tragic — as if it were a workstream with a Gantt
chart, dependencies, and a status colour.

### Step 2 — Analysis against the checklist

1. **Comic premise construction.** An emotionally significant or serious event (a breakup, a
   funeral, a birth) is narrated as a project update.
2. **Character roles and power relationships.** The narrator holds no real authority over the
   events being described, but speaks with the confidence of someone who has full authority over
   a project — the mismatch between narrated authority and actual authority is the engine.
3. **Dialogue structure.** Other characters speak in normal registers; the narrator's lines (and
   internal narration) are the only ones in project-management jargon, which sharpens the
   contrast rather than blending in.
4. **Sentence length and rhythm.** Clipped, list-like sentences mimicking status updates ("Blocked
   on: Dad. Owner: TBD. Status: At risk.").
5. **Escalation pattern.** Each new development in the emotional situation gets reframed with a
   more elaborate piece of project-management apparatus — a status colour, then a RAID log entry,
   then a full retrospective.
6. **Use of repetition.** A recurring phrase ("let's take this offline") reappears at each
   emotionally inappropriate moment.
7. **Rhetorical devices.** Euphemism (calling grief "a blocker"), false neutrality (assigning a
   "risk owner" to a family conflict).
8. **Political/social targets, if relevant.** Workplace jargon culture and its colonisation of
   personal life — not a specific person or group.
9. **Typical endings and reversals.** The narrator finally uses a piece of jargon so absurdly
   inappropriate for the moment that even they seem to notice, but instead of dropping the bit,
   they escalate it one further notch and move straight to "next steps."
10. **What makes the humour work.** The gap between the narrator's total sincerity and the
    absurdity of applying project management to things that categorically cannot be managed that
    way.

### Step 4 — Converting analysis into rules

- Narrate one clearly emotional or serious event using project-management terminology
  throughout — status updates, blockers, owners, risk levels — with zero acknowledgement that
  this framing is unusual.
- Keep every other character's dialogue in a normal register; the contrast comes from the
  narrator alone using jargon, not from the whole scene adopting it.
- Write the narrator's lines short and list-like, mimicking a status report, in contrast to
  longer, more natural sentences from other characters.
- Escalate by upgrading the jargon apparatus at each beat: a status colour, then a logged risk,
  then a formal retrospective — never downgrade or repeat the same device twice in a row.
- Use "let's take this offline" (or an equivalent recurring corporate phrase) at least once at a
  moment where it is maximally inappropriate.
- End on the narrator applying the single most inappropriate piece of jargon yet, then moving
  straight to a brisk "next steps" line without any beat of self-awareness — do not let the
  narrator break the bit to acknowledge the absurdity.

### Step 5 — Assembled skill excerpt

```markdown
---
name: project-manager-narrator-esque
description: Generate an original comic scene or short piece narrated in the voice of a project manager who reframes an emotionally serious personal event as a workstream — status updates, blockers, risk logs — while staying completely sincere. Use this when a user wants a comic bit built on corporate-jargon narration clashing with an emotionally serious situation. Do NOT use this for workplace satire aimed at a real identifiable person, or for comedy where the humour comes from the narrator being deliberately callous rather than sincerely oblivious.
---

# Project-Manager-Narrator-esque Comedy

## Mindset
The narrator's total sincerity is the engine: the humour comes from the gap between complete
conviction in project-management framing and events that categorically cannot be managed that
way. If the narrator ever acknowledges the absurdity, the engine stops.

## Core mechanics
- Narrate a serious personal event entirely in project-management terminology, with total sincerity.
- Keep every other character's dialogue in a normal register; only the narrator speaks in jargon.
- Write the narrator's lines short and status-report-like.
- Escalate the jargon apparatus at each beat (status colour, then risk log, then retrospective) rather than repeating the same device.
- Deploy a recurring corporate phrase at a maximally inappropriate moment.

## Structure
1. Establish the serious event through other characters' normal dialogue.
2. Introduce the narrator's project-management reframing of it.
3. Escalate the jargon apparatus across 2-3 beats.
4. Land the recurring phrase at its most inappropriate moment.
5. Close on the narrator upgrading to the most absurd piece of jargon yet, then moving straight to "next steps" — no acknowledgement, no self-aware wink.

## Troubleshooting
- If the piece reads as just a jargon list, the narrator is describing rather than escalating — ensure each beat upgrades the apparatus instead of restating the previous one.
- If the piece feels like it's mocking the narrator, the register has slipped — the narrator must never signal self-awareness of the absurdity.

Generate original scenes and dialogue for this piece; do not reuse existing published bits built on this premise.
```

*This shows how the full checklist's points distribute across the skill's sections: points 1, 2,
8, and 10 inform the `description` and the premise; points 3-7 and 9 become specific bullets in
`Core mechanics` and steps in `Structure`; point 10 also becomes the `Mindset` statement; the
failure-mode pairs, `Troubleshooting` branches, and per-mechanic worked examples required by step
5 of the generator would fill out the remaining sections — and the step 6 style-fidelity pass
(draft a trial piece, fix the skill if it misses) is what confirms the assembled skill actually
reproduces the tradition before it is returned.*

## Example 2: "a conspiracy-board true-crime parodist"

The requested tradition: comedy built as a mock true-crime documentary in which a narrator treats
a trivial mystery (who ate the last slice of cake) with the full evidentiary apparatus of a real
criminal investigation — corkboard, red string, dramatic narration.

### Step 2 — Analysis against the checklist

1. **Comic premise construction.** A trivial, low-stakes mystery is investigated with the full
   procedural weight of a serious criminal case.
2. **Character roles and power relationships.** The narrator/investigator holds no actual
   authority (they are a housemate, a coworker, a sibling) but speaks with the gravity of a
   detective or documentary narrator.
3. **Dialogue structure.** Interview-style "talking head" asides interrupt the main narration,
   mimicking true-crime documentary editing.
4. **Sentence length and rhythm.** Long, ominous, slow-building sentences full of dramatic
   pauses, contrasted with the mundane content of what's actually being described.
5. **Escalation pattern.** Circumstantial evidence is treated as increasingly damning: a receipt,
   a security-camera-style detail ("she was seen near the fridge at approximately 9:47pm"), a
   forced confession.
6. **Use of repetition.** A repeated dramatic phrase ("but here's where it gets interesting")
   before each new, trivial piece of evidence.
7. **Rhetorical devices.** False gravitas (calling a fridge "the scene"), appeal to authority
   ("forensic analysis of the crumbs").
8. **Political/social targets, if relevant.** True-crime documentary conventions themselves —
   not a real case or real people.
9. **Typical endings and reversals.** The case is "solved" with a wildly disproportionate
   response (a tribunal, an exile from the group chat) to a nonexistent or trivial crime, or the
   twist reveals the investigator did it.
10. **What makes the humour work.** The total mismatch between the stakes of the actual event and
    the procedural, dramatic weight of the genre being parodied.

### Step 4 — Converting analysis into rules

- Investigate a trivial, low-stakes mystery using the full procedural apparatus of a real
  criminal case — evidence boards, timelines, forensic-sounding language.
- Break the main narration with short "talking head" asides in a different voice/register, in
  the style of true-crime documentary interview cuts.
- Write narration in long, ominous, dramatically-paced sentences that build tension around
  content that is objectively trivial.
- Escalate the "evidence" from circumstantial to falsely conclusive across at least three beats.
- Repeat one dramatic transition phrase before each new piece of evidence.
- Resolve with either a wildly disproportionate consequence for the "crime," or a twist that the
  investigator was responsible all along — pick whichever fits the specific mystery better.

*Note how this reaches nearly identical rule shapes to Example 1 (sincerity, escalation,
disproportion) while producing entirely different concrete content — the checklist is the same
across traditions, but nothing about the resulting rules is copy-pasted between them.*

## Model formats for checklist point 11 (format and medium constraints)

When a request doesn't name its own format, these six common comedy forms are a useful model of
how specific the format constraint in the checklist should be — each form carries a distinct
structural pattern a generated skill should pin down rather than leave implicit:

- **Stand-up bit** — premise + escalating examples or a rule of three; callbacks reward patient
  audiences.
- **Monologue** — first-person confessional; build the situation, let it spiral, end on something
  self-defeating or absurdly mundane.
- **One-liner** — maximum compression; one premise, one turn, fewest possible words, punchline
  word at the end.
- **Observational story** — "have you ever noticed..." + escalating truth + an earned absurd
  conclusion.
- **Roast** — specific to the target; true observations exaggerated to their logical extreme;
  affection makes it land, cruelty alone just stings (a nuance that also belongs in the
  targets/audience-contract checklist guidance).
- **Comic essay** — long-form observation with a consistent voice; analogies and digressions that
  pay off; a thesis that gets weirder as it's proven.

These are a floor, not a ceiling — the point of checklist point 11 is to force the analysis to
state the form's structural consequences explicitly, whichever form the tradition uses.
