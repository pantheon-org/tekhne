---
name: follow-up
description: >
  Read-only status check across the follow-up backlog: lists every ACTIVE
  follow-up entry, assesses whether it's still genuinely open, and
  recommends the single best next item to work on -- pulling in open
  RISK_REGISTER and TECH_DEBT rows and the value-rubric's ranked
  PLAN/FINDING/KNOWN_ISSUE candidates when the follow-up list itself is
  empty, since that's where promoted follow-up work actually ends up
  living. Triggers: '/follow-up', 'check follow-ups', 'what's open', 'what
  should I work on next', 'follow-up status check', 'next best item',
  'what's still open'. Does NOT promote, close, or otherwise edit anything
  -- it reports and recommends, then applies follow-up-triage's own flow
  directly or hands off to the risk-register / tech-debt / plan-create
  skills for the actual write.
---

# Follow-up Status Check

The read-only front door onto this project's backlog. follow-up-triage's flow is the *action* step once you
already know a follow-up needs promoting or closing; this skill runs first to find out whether one does, and --
because the follow-up list itself is often empty once things have been promoted -- to say what to work on next
even when it is. See [Backlog Triage and Ranking](references/backlog-triage-and-ranking.md) for the full,
self-contained decision logic this skill applies.

## Mindset

A follow-up sitting `ACTIVE` is a weak signal by design: someone flagged it and moved on, which is not the same as
it still being the right thing to do next. TYPICALLY the useful answer to "what's open" spans three sources at
once (the follow-up list, the register, the tech-debt list), not just the one the command name suggests. PREFER
checking all three and saying which source the recommendation came from, over answering only the literal
follow-up list and calling it done -- an empty follow-up list is a completely normal outcome (every follow-up
promoted and closed) and never grounds for stopping the check early.

## Rules

1. **This skill never mutates the register, the tech-debt list, or a follow-up's status.** Report and recommend
   only. Apply follow-up-triage's flow directly to promote or close a follow-up (it's an instructions doc, not a
   separate Skill), or hand off to the `risk-register` / `tech-debt` skills to file or resolve a row, or
   `plan-create` to scaffold work on the recommended item -- and only after the user says to proceed.
2. **Zero `ACTIVE` follow-ups is not "nothing to do."** It usually means the funnel is empty because everything in
   it was already promoted or closed -- the real backlog then lives in the register/tech-debt `Open` rows and the
   context index's `DRAFT`/`ACTIVE` `PLAN`/`FINDING`/`KNOWN_ISSUE` entries. Always fall through to those sources
   rather than reporting an empty follow-up list as the whole answer.
3. **Ground the "next best item" pick in the value-rubric's read protocol wherever it applies** (`PLAN`/`FINDING`/
   `KNOWN_ISSUE` entries carry a `value` grade and a defined sort). For a pick sourced from the register or the
   tech-debt list instead, say explicitly that it's a qualitative judgment call -- those rows carry no `value`
   field, so there is no rubric-sorted answer to defer to.
4. **If the context index looks stale, regenerate it (`context-index` skill) before trusting it.** A quick sanity
   check: its header entry count should roughly match the actual file count under the context directory (see
   Prerequisites for the exact path). A mismatch means new or removed files haven't been indexed yet, and the
   candidate-pool step of the Workflow would be working from stale data.
5. **An `ACTIVE` follow-up outranks a silently-pending backlog row.** If the follow-up sweep finds any `ACTIVE`
   entry, lead the recommendation with it rather than the value-rubric or register/tech-debt pools -- a follow-up
   is something a person already flagged as worth acting on, which is a stronger signal than an unflagged row
   sitting in a list.

## Prerequisites

```text
.context/follow-ups/*.md                    the follow-up list
.context/index.yaml                         the context index (value-rubric candidates)
docs/RISK_REGISTER.md                       the register
docs/TECH_DEBT.md                           the tech-debt list
.claude/instructions/follow-up-triage.md    follow-up-triage's flow (canonical source; summarized in references/)
.claude/instructions/value-rubric.md        the value-rubric protocol (canonical source; summarized in references/)
```

## When to Use

- The user types `/follow-up`, or asks "what's open", "what should I work on next", "check the follow-up backlog",
  "anything still outstanding", "next best item to work on".
- At the start of a session, as a cheap alternative/complement to reading the tech-debt list's Progress-section
  prompt by hand.

## When NOT to Use

- **To actually promote or close a follow-up** -- that's follow-up-triage's flow, applied directly (this skill
  only tells you which follow-ups need it).
- **To file a new follow-up, finding, or plan** -- that's `context-file` / `plan-create`.
- **To add or resolve a register or tech-debt row** -- that's `risk-register` / `tech-debt` directly, once this
  skill (or anything else) has told you which row.

## Workflow

1. **Freshness check (Rule 4).** Compare the context index's header entry count against the actual file count
   under the context directory:

   ```bash
   find .context -name '*.md' | wc -l
   ```

   If the result diverges meaningfully from the index's own header comment, run `context-index` first.
2. **Follow-up sweep.** Filter the follow-up list to `status: ACTIVE`:

   ```bash
   grep -l 'status: ACTIVE' .context/follow-ups/*.md
   ```

   `DONE` entries are closed; ignore them here. For each `ACTIVE` entry, read its `Context` + `Outstanding Work`,
   check whether its topic already has a matching `Open` row on the register or the tech-debt list, and classify
   per the disposal logic in [Backlog Triage and Ranking](references/backlog-triage-and-ranking.md) -- ready to
   close-as-superseded, needs promotion to the register, needs promotion to the tech-debt list, or still genuinely
   open. File nothing; that's the handoff, not this skill's job. If none are `ACTIVE`, say so explicitly and
   continue anyway (Rule 2).
3. **Build the wider candidate pool.** Count the committed-backlog side directly:

   ```bash
   grep -c '| Open ' docs/RISK_REGISTER.md docs/TECH_DEBT.md
   ```

   Apply the value-rubric read protocol (same reference doc) over the context
   index's `PLAN`/`FINDING`/`KNOWN_ISSUE` entries; separately collect `Open` rows from the register and the
   tech-debt list.
4. **Synthesize one recommendation** (Rule 5 ordering): any `ACTIVE` follow-up first; otherwise the value-rubric
   pool's top tier-1 item; otherwise a qualitative judgment call across the register/tech-debt pool (Rule 3).
   Always state why the pick beats the runner-up, not just that it won.
5. **Report** using the format in [Worked Example](references/worked-example.md), then stop -- offer the relevant
   handoff rather than acting on it.

## Anti-Patterns

**NEVER** report "no open follow-ups" as the complete answer to `/follow-up`.
**WHY:** A follow-up funnel emptying out because everything in it was promoted or closed is the *expected* steady
state once a project matures past its early weeks -- exactly the case in this repo, where all 15 filed follow-ups
are currently `DONE`. Stopping there answers the literal words of the command while ignoring the actual open
backlog sitting in the register and the tech-debt list.
**BAD:** "No ACTIVE follow-ups. Nothing to report."
**GOOD:** "No ACTIVE follow-ups (all 15 filed ones are DONE). Falling through to the committed backlog: 38 open
register rows, 7 open tech-debt rows -- recommended next item is register row 5 (CI runner allowlist opacity,
recurring friction)."
**Consequence:** A genuinely large, actionable backlog goes completely unreported, and the person who ran
`/follow-up` walks away believing there is nothing to do when there are dozens of open items one directory over.

**NEVER** invent a `value` ranking for a register or tech-debt row.
**WHY:** Those rows have no `value` field and were never graded against the value-rubric protocol -- presenting a
pick from them with the same confidence as a rubric-sorted `PLAN`/`FINDING`/`KNOWN_ISSUE` pick overstates how
principled the ordering is.
**BAD:** "Value: HIGH -- row 6 (collector failure logging)." (no such field exists on that row)
**GOOD:** "Judgment call, not rubric-sorted: row 6 touches the RUNBOOK's symptom index directly, which is why it
ranks above rows 7-11's mechanical single-export-per-file cleanups."
**Consequence:** The reader trusts an ordering that was never actually graded against a standard, and a
differently-reasonable ranking (another person's judgment call) looks like a disagreement with the rubric instead
of what it actually is -- two informal opinions.

**NEVER** promote, close, or edit a row or follow-up from inside this skill.
**WHY:** This skill's entire value is being safe to run at any time with no side effects -- the moment it starts
writing to a committed doc, running it "just to check" carries the same risk as applying follow-up-triage's own
promotion/close step itself.
**BAD:** Silently flipping a follow-up's `status` to `DONE` because its topic was found already covered by an
existing register row.
**GOOD:** "This follow-up is ACTIVE but its topic is already covered by register row 16 -- recommend closing it as
superseded per follow-up-triage's flow. Want me to do that now?"
**Consequence:** A user who ran this skill expecting a safe, side-effect-free check instead finds a doc already
mutated without a chance to review the change first -- exactly the trust `/follow-up` is supposed to earn by never
needing that caution in the first place.

**NEVER** skip the freshness check and trust a possibly-stale context index.
**WHY:** A newly filed follow-up, or one just closed in a separate change, may not be reflected yet -- the sweep
and the candidate pool are both built directly from the index's current contents.
**BAD:** Reading the index once at the start of a long session and reusing it for every later `/follow-up` call
without re-checking.
**GOOD:** Compare the index's header entry count against the actual file count before every sweep; regenerate via
`context-index` on a mismatch, per Rule 4.
**Consequence:** The skill can recommend an item someone already closed in a separate change, or miss a brand-new
`ACTIVE` follow-up entirely -- silently, since a stale-but-present index never errors the way a missing file would.

**NEVER** present a register/tech-debt-sourced pick with the same confidence as a value-rubric-sorted pick.
**WHY:** Only `PLAN`/`FINDING`/`KNOWN_ISSUE` entries carry a `value` grade and a defined sort; a register or
tech-debt row has neither, so any ordering among them is this skill's own judgment call applying the rubric's
grading questions informally, not the project's own rubric output.
**BAD:** Reporting a register-sourced recommendation in the exact same sentence structure as a rubric-sorted one,
with no signal that one is graded and the other isn't.
**GOOD:** "Judgment call, not rubric-sorted: register row 5 outranks tech-debt row 7 because it retires a
*recurring* cost (every future CI job risks the same rediscovery), where row 7 only ever saves the one file it
touches."
**Consequence:** Two different people (or the same person on two different days) can reasonably rank the same
register/tech-debt pool differently, and presenting one ordering as if it were rubric output hides that
disagreement instead of surfacing it.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Full follow-up disposal logic and the value-rubric's ranked-candidate read protocol, self-contained | [Backlog Triage and Ranking](references/backlog-triage-and-ranking.md) | Workflow steps 2-4, or when Rule 3's qualitative-judgment call needs justifying |
| A fully filled-in sample report against this project's real state, plus the blank output template | [Worked Example](references/worked-example.md) | Workflow step 5, or when drafting the first report of a session |
