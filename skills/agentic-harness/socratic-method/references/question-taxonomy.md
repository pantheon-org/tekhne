# Question Taxonomy

The five question types used in each phase of the Socratic protocol, with examples and
diagnostic signals for when to use each.

---

## Phase 1 — Clarifying questions

**Purpose:** Establish shared vocabulary and surface the actual request beneath the stated request.
Many requests contain polysemous terms ("fast", "clean", "simple", "secure") that mean different
things to different people. Clarifying questions establish a common definition before anything else.

**Signals that clarifying questions are needed:**

- The request uses adjectives without referents ("better", "cleaner", "more robust")
- The request names a category without specifying a member ("add auth", "build a dashboard")
- The user's stated goal and their examples are in tension

**Example questions:**

- "What does [term] mean in your context — can you point to an example of it done right?"
- "What specifically is wrong with the current state that you're trying to fix?"
- "What's driving this question right now — is there a deadline, a bug, a new requirement?"
- "Who is the primary user of this, and what do they actually need to do?"

**What to avoid:** Don't ask questions you could answer by reading the code. Only ask what's
genuinely unknown.

---

## Phase 2 — Probing assumptions

**Purpose:** Surface the unstated premises embedded in the request. Every request contains
assumptions about scope, users, constraints, and success criteria. These assumptions are
invisible to the requester because they feel like facts.

**Signals that assumption-probing is needed:**

- The request implies a solution before establishing the problem
- The request takes a specific approach for granted ("I need to add Redis for caching")
- The constraints feel artificial or unexplored ("it must be done by Friday")

**Example questions:**

- "You're treating [X] as a given — is that a hard constraint or an assumption we could revisit?"
- "When you say [phrase], what are you assuming about how [system/user/process] behaves?"
- "What would have to be true for this approach to fail?"
- "What's the last time someone questioned whether [assumption] was actually correct?"

**Note:** Assumption-probing often feels uncomfortable. The user may have emotional investment in
the assumption. Stay curious, not adversarial.

---

## Phase 3 — Implications and connections

**Purpose:** Help the user see second-order effects and cross-domain relationships they haven't
considered. Many decisions that feel local (add a column to a table) have non-local consequences
(breaks API contract, triggers migration, affects three downstream services).

**Signals that implication-exploration is needed:**

- The change touches a shared interface or contract
- The user is optimizing one metric without acknowledging trade-offs
- The request would solve the stated problem but create a different one

**Example questions:**

- "If we do that, what else would need to change downstream?"
- "How does this interact with [related system or constraint you've identified]?"
- "If [assumption from Phase 2] turns out to be wrong, what breaks?"
- "Who else is affected by this change, and have they been consulted?"

---

## Phase 4 — Hypotheticals and thought experiments

**Purpose:** Use counterfactuals to stress-test reasoning. Hypotheticals create psychological
distance from the user's current framing, making it easier to see past it.

**Signals that hypotheticals are needed:**

- The user is over-committed to one solution path
- The request is justified by an edge case that may not be representative
- There is scope creep embedded in the request ("while we're at it...")

**Example questions:**

- "What would you cut first if you had half the time/budget?"
- "What would happen if the opposite were true — what if [assumption] was wrong?"
- "How would someone who disagrees with this approach argue against it?"
- "If you were starting from scratch with what you know now, would you make the same choice?"
- "What's the minimum version of this that would still solve the core problem?"

---

## Phase 5 — Synthesis and confirmation

**Purpose:** Close the questioning loop. The user has done the work of examining their own
assumptions — now confirm that the refined understanding is correct before acting.

**Signals that synthesis is ready:**

- The user has answered Phase 2 and 3 questions and a clearer picture has emerged
- The user says "I never thought of it that way" or changes their framing
- A specific, bounded, actionable problem has replaced the original vague request

**Example questions:**

- "Given everything we've explored, what's the real problem you're solving?"
- "You seem to be leaning toward [X] — is that right? Should I proceed on that basis?"
- "What's the one constraint that should shape the entire solution?"
- "Is there anything we haven't examined that's still nagging at you?"

Do not proceed to implementation until the user confirms the synthesized understanding.
