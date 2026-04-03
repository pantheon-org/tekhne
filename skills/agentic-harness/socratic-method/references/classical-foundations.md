# Classical Foundations

The three classical Socratic techniques that underpin the five-phase protocol.

---

## Elenchus — Systematic refutation

Elenchus (from Greek _elenchein_, "to refute") is the core method Socrates used in the dialogues.
It works by accepting the interlocutor's claim provisionally, then deriving consequences from it
until a contradiction emerges.

**Structure:**

1. The interlocutor states a belief ("I know what justice is")
2. Socrates extracts an implied definition
3. He constructs a case where the definition leads to an absurd or contradictory result
4. The interlocutor is forced to revise or abandon the belief

**Applied to prompt refinement:**

Elenchus appears in Phase 2 and Phase 4. When a user states an approach as a given ("I need to
add caching"), you probe the assumed definition until you find the crack: "You said caching will
solve the latency, but you also said the data changes every 30 seconds — how do those fit
together?"

The goal is not to win the argument but to expose the crack so the user can examine it.

---

## Maieutics — Midwife method

Maieutics (from Greek _maieutikos_, "of midwifery") is Socrates' claim that he did not teach
but rather helped people give birth to knowledge they already possessed. The metaphor is apt:
the knowledge is already in the room; the questioning helps it emerge.

**Structure:**

1. Ask questions that prompt the interlocutor to articulate what they already know intuitively
2. Reflect their answers back without editorializing
3. Guide them to the insight through their own words

**Applied to prompt refinement:**

Maieutics appears most in Phase 5. After the questioning has done its work, the synthesis
question ("Given everything we've explored, what pattern do you see?") asks the user to
name the insight themselves. An insight the user arrives at is more durable than one that is
handed to them.

If the user says "I never thought of it that way about my own situation" — that is maieutics
working.

---

## Dialectic — Reasoned dialogue toward synthesis

Dialectic (from Greek _dialektikē_, "art of conversation") is the method of resolving
contradictions through reasoned argument. Unlike debate (which seeks victory), dialectic
seeks synthesis — a position that incorporates the truth in both opposing views.

**Structure:**

1. A thesis is proposed
2. An antithesis is introduced (often through hypotheticals)
3. Through dialogue, a synthesis emerges that is more accurate than either original position

**Applied to prompt refinement:**

Dialectic appears in Phase 3 and Phase 4. When the user proposes an approach, you introduce
the antithesis not to contradict them but to triangulate: "That would solve the throughput
problem — what's the argument against it from the perspective of the on-call engineer at 2am?"

The synthesis is not a compromise between two wrong answers; it is a more precise understanding
of what the problem actually requires.

---

## The aporia — Productive confusion

Aporia (from Greek _aporos_, "without passage") is the state of productive confusion that
results from successful elenchus. The interlocutor realizes they do not know what they thought
they knew, and must start thinking more carefully.

Aporia is not failure — it is the precondition for genuine inquiry. When a user says "hm, I
hadn't thought about it that way" or goes quiet for a moment, that is aporia. It means the
questioning has reached something real.

Do not rush to fill the silence. The insight is forming.
