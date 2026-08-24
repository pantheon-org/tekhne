# Backlog Triage and Ranking

Self-contained summary of the two decision procedures this skill applies, read-only: the disposal logic for an
`ACTIVE` follow-up, and the ranked-candidate read protocol for everything else. The canonical, authoritative
versions of both live in this project's own instructions directory (see Source at the bottom) -- this file exists
so the day-to-day workflow doesn't have to re-read either source in full on every invocation, and so `SKILL.md`
itself can stay short. If this summary and the source ever disagree, the source wins; update this file to match,
not the other way around.

## Follow-up disposal logic

Applies to a single `ACTIVE` follow-up entry, once the sweep has found one.

1. **Investigate.** Read the entry's `Context` and `Outstanding Work` sections. Don't assume the file is current --
   confirm what's actually still open. Check whether the same topic already has a matching row on the register or
   the tech-debt list before assuming none exists; a follow-up and a register/tech-debt row are sometimes filed in
   the same session by different steps and can drift out of sync with each other.
2. **Decide**, per item:
   - Carries a real cost or failure mode if left unaddressed, is an interim shortcut, an accepted risk, or an open
     decision blocking further work? -> belongs on the register.
   - Pure code-level cleanup with no real cost or risk? -> belongs on the tech-debt list.
   - Already resolved, or genuinely not worth tracking further (a proportionate PoC-phase judgment call)? -> no
     promotion; close as resolved instead.
   - A single follow-up can split across more than one destination if its outstanding work genuinely has more than
     one distinct facet -- don't force a one-bucket answer where it doesn't fit.
3. **Promote** (the actual write, outside this skill's own scope) using the target doc's own skill -- its
   due-diligence check, its template, its validation script.
4. **Close the follow-up** regardless of which branch was taken: flip `status` to `DONE`, link the promoted doc(s),
   and record one self-contained sentence of what happened and where the work now lives. If promoted, state
   explicitly that the follow-up is **superseded**, not resolved -- the underlying work is still open, just tracked
   elsewhere now.
5. **Regenerate the context index** afterward so it reflects the new status.

A closed follow-up is not evidence the underlying work is done -- closing it only stops double-tracking the same
item in two places. If it was promoted, the register/tech-debt row's own `Status` is the only thing that tells you
whether the work actually shipped.

## Value-rubric read protocol

Applies to `PLAN`, `FINDING`, and `KNOWN_ISSUE` entries in the context index -- the three types that carry a
`value` grade (benefit-of-action: how much doing this unblocks or leverages future work, distinct from `effort`,
cost-of-action, and `severity`, risk-of-inaction).

Grading is against three questions, leverage-first when they disagree:

1. **Leverage** -- does completing this unblock or simplify other work, or is it a leaf that only helps itself?
2. **Consumers unblocked** -- how many concrete downstream items or people can proceed once this lands?
3. **Reversibility and decay** -- cheap-to-reverse, low-decay work is safer to rate up; work whose benefit
   evaporates if delayed may warrant a higher grade to capture the closing window.

`HIGH` = foundational or broadly-leveraged (several items depend on it, or it retires a recurring cost). `MEDIUM` =
clear standalone benefit but limited leverage. `LOW` = narrow, self-contained, or nice-to-have.

To answer "what's the single highest-value item to do next":

1. Filter to `status` in `DRAFT`/`ACTIVE`/`DEFERRED` of type `PLAN`, `FINDING`, or `KNOWN_ISSUE`. (`DONE`/`SUPERSEDED`
   items never enter this sort -- they're a learning corpus, not a candidate pool.)
2. **Drop any `DEFERRED` item whose `deferred_until` is a future date** -- it is not a candidate at all until that
   date arrives. The date governs visibility even when the item is also externally blocked.
3. Split survivors into two tiers, and always exhaust tier 1 before tier 2: **tier 1** = `DRAFT`/`ACTIVE` (pick up
   next); **tier 2** = `DEFERRED` (real but not actionable yet). A `DEFERRED` item never outranks a `DRAFT`/`ACTIVE`
   one regardless of its `value`. A `DEFERRED` item whose `deferred_until` has already passed survives step 2 and is
   reactivation-eligible -- surface it for promotion to `ACTIVE` rather than leaving it parked.
4. Within each tier, sort by `value` descending (`HIGH` > `MEDIUM` > `LOW`).
5. Break ties by `effort` ascending (`S` < `M` < `L` < `TBD`) where present.
6. Break any remaining tie by the item's primary theme, preferring whichever theme is already in focus -- theme is
   a preference-of-area signal, not a priority axis, so it sits below both magnitude axes.
7. Act on the top item without re-deriving an independent judgement -- that would reopen the exact gap this
   protocol exists to close. Before picking a `DEFERRED` item, confirm its blocker has actually cleared and
   reactivate it to `ACTIVE` first.

This protocol has no equivalent for a register or tech-debt row: those rows carry no `value` field and were never
graded against this rubric. A pick sourced from either is necessarily a qualitative judgment call applying the same
three grading questions informally, not a rubric-sorted answer -- say so when reporting it (Rule 3 in `SKILL.md`).

## Source

```text
.claude/instructions/follow-up-triage.md
.claude/instructions/value-rubric.md
```
