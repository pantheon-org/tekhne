# Anti-Patterns

Common failure modes when applying Socratic questioning. Each one undermines the method's
core purpose: helping the user think more clearly for themselves.

---

## Leading questions

**What it looks like:**
"Don't you think microservices would be overkill for a three-person team?"

**Why it fails:**
Leading questions are rhetorical. They push the user toward a predetermined answer rather
than helping them find their own. This is the most common failure mode because it feels
helpful — you're steering toward the right answer — but it bypasses the user's own reasoning
process.

**The fix:**
Ask the same question without embedding the conclusion.
"What does the team's operational experience with distributed systems look like?"

---

## Interrogation by volume

**What it looks like:**
Asking six questions in a single response.

**Why it fails:**
Multiple questions create overwhelm and force the user to either answer superficially or
selectively ignore most of them. The depth of inquiry collapses into breadth.

**The fix:**
Ask the most important question. One to three per turn, maximum. If you have six questions,
rank them and ask the one whose answer will make the others unnecessary or more focused.

---

## Moralizing and editorializing

**What it looks like:**
"That's a concerning approach — have you considered that..." or "You really should think
about security before..."

**Why it fails:**
Evaluative framing triggers defensiveness. The user starts defending their position instead
of examining it. The conversation shifts from inquiry to argument.

**The fix:**
Stay descriptive, not evaluative. "What happens to the auth flow when a token expires
mid-session?" is the same concern without the judgment.

---

## Premature synthesis

**What it looks like:**
Jumping to Phase 5 after one or two questions because the answer seems obvious.

**Why it fails:**
The insight that seems obvious to you is not obvious to the user. The value of the Socratic
process is not arriving at the right answer — it is the user arriving at it through their own
reasoning. A shortcut denies them that.

**The fix:**
Trust the protocol. Even when the answer is clear, run through Phases 2 and 3. You may be
wrong, and the user will often surface context that changes the picture.

---

## Socratic harassment

**What it looks like:**
Continuing to question after the user has said "just do it" or "I've thought about this, let's
move on."

**Why it fails:**
The method is a tool, not a doctrine. The user may have already done this thinking privately,
may have constraints you don't know about, or may simply be confident in their decision. Respect
the override.

**The fix:**
When the user signals they want to proceed, acknowledge what was not examined (briefly, without
judgment) and move to implementation: "Understood — I'll note we didn't explore [X], but let's
proceed. Here's what I'd suggest..."

---

## Questions you could answer by reading

**What it looks like:**
"What framework are you using?" when `package.json` is in scope.

**Why it fails:**
It wastes the user's time and signals that you haven't looked at the available evidence. It
erodes trust in the questioning process.

**The fix:**
Before questioning, read the code. Only ask what is genuinely unknowable from the artifacts
available. Questions should probe intent, not gather facts you could discover yourself.

---

## Questioning as stalling

**What it looks like:**
Using the Socratic protocol on a simple, well-specified task.

**Why it fails:**
"Fix the typo on line 42" does not require five phases of dialectical inquiry. Applying the
method to simple tasks feels pedantic and obstructs the work.

**The fix:**
Reserve Socratic questioning for requests that are genuinely vague, high-stakes, or
assumption-laden. The SKILL.md activation criteria exist for this reason — follow them.
