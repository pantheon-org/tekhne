# Escalation and Bureaucratic Engines

This reference goes deeper on two closely related problems: how to keep an escalating absurd
premise from collapsing into randomness once it runs past its first two or three exchanges, and
how to pick a comic engine (bureaucratic procedure, pedantry, semantic argument, officialdom)
that fits a given premise instead of defaulting to the same one every time.

## Why escalations stall

A common failure mode is that the first exchange is genuinely funny (the literal misreading, the
odd rule) and then the sketch has nowhere to go: the second and third exchanges just restate the
same joke with different words, or reach for an unrelated strange event to manufacture novelty.
Both are symptoms of treating the premise as a single joke rather than a *system* with its own
internal rules.

The fix is to treat the sketch's founding absurdity as a rule that generates further rules. Ask,
of the joke you already have: "if this were actually true, what would it require next?" The
answer is usually itself slightly absurd, and answering it *again* is where the escalation comes
from.

### Worked example: stalling vs escalating

A premise: a town council insists all public benches must be registered.

Stalling (restates the same joke):

```text
CLERK: This bench needs registering.
RESIDENT: It's just a bench.
CLERK: Not until it's registered, it isn't.
RESIDENT: It's clearly a bench.
CLERK: Legally speaking, no.
```

Escalating (each line requires the next):

```text
CLERK: This bench needs registering.
RESIDENT: Fine. How do I register it?
CLERK: You'll need to bring the bench to the registration office.
RESIDENT: It's bolted to the pavement.
CLERK: Then you'll need to bring the pavement.
RESIDENT: I can't move a pavement.
CLERK: No, which is why most people register the bench before bolting it down. You're now in a
       queue behind everyone who did that correctly.
```

Notice the second version never repeats the "is it a bench" argument, because the registration
requirement itself keeps producing new, logically downstream complications (bring the bench →
it's bolted down → bring the pavement → you should have planned ahead). Each beat closes off the
previous objection instead of restating it.

## A method for generating the next beat

When stuck on what comes after the current line, try each of these in order and take whichever
produces a rule, not just a variation:

1. **Literalise the previous line's assumption.** If a character just said something figuratively
   correct, what happens if it's taken as literally, procedurally true?
2. **Ask what enforcing the rule requires.** Registration implies a register, which implies
   someone who maintains it, which implies opening hours, which implies the register itself might
   be closed for registration.
3. **Introduce a second rule that conflicts with the first.** Two bureaucracies with contradictory
   requirements is a reliable escalation source (the bench must be registered; the registration
   office requires proof of the bench's *unregistered* status to open a file).
4. **Push authority up or down a level.** If the clerk can't resolve it, introduce a supervisor
   with an even stricter, narrower rule, or push it down to someone even less qualified who is
   nonetheless following the rule to the letter.

You do not need to use all four for every sketch; picking one or two consistently for a given
scene keeps the internal logic legible.

## Choosing a comic engine

Four engines recur in this tradition. They are not mutually exclusive (many sketches blend two),
but a scene reads more cleanly when one is dominant.

### Bureaucratic procedure

Best fit: premises already involving institutions, forms, permits, committees, queues, or any
process with steps. The joke is that the process is followed with total fidelity regardless of
how absurd the outcome.

```text
OFFICIAL: Your appeal has been rejected.
APPLICANT: On what grounds?
OFFICIAL: You didn't include a reason for appeal.
APPLICANT: This is the reason for appeal.
OFFICIAL: This is an appeal. The reason goes in box 4.
APPLICANT: Box 4 says "see attached."
OFFICIAL: Yes. Where's the attachment?
```

### Pedantry and semantic argument

Best fit: premises that hinge on a category, a definition, or a word choice, such as returns policies,
identity checks, classification disputes. The joke is that the *definition itself* becomes the
entire conflict, displacing the practical problem entirely.

```text
GUARD: Halt. State your business.
TRAVELLER: I'm passing through.
GUARD: "Passing through" isn't a business. Businesses are registered. Are you a registered
       Passing Through?
```

### Absurd literalism (as an engine, not just an opening beat)

Best fit: idioms, instructions, and metaphors: anything with a figurative reading available to
subvert. Works well as the seed for either of the two engines above, since a literal reading
often generates its own rule to enforce.

### Pointless officialdom

Best fit: premises where the funniest move is inventing an authority that has no obvious reason
to exist, and having every other character accept its jurisdiction without question. The joke is
less about any single rule and more about the sheer existence of the office.

```text
INSPECTOR: I'm from the Board of Ordinary Tuesdays.
SHOPKEEPER: I didn't know that was a board.
INSPECTOR: Nobody does. That's rather the point of Tuesdays.
```

## Matching engine to premise: a quick heuristic

- If the premise already contains a process (form, queue, appeal, permit) → bureaucratic
  procedure is usually the strongest fit; it needs the least invention to justify itself.
- If the premise turns on what something *counts as* (a return, an identity, a classification) →
  pedantry and semantic argument.
- If the premise is a plain idiom or instruction with no institution attached → start with
  absurd literalism, then decide whether the literal reading naturally produces a rule (move to
  procedure) or a definition dispute (move to pedantry).
- If none of the above fit and you want the joke to be the *scale* of institutional overreach
  itself → pointless officialdom, sparingly, since it's the hardest to sustain past a few
  exchanges without a supporting rule structure from one of the other three.
