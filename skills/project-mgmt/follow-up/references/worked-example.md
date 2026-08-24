# Worked Example

An illustrative, fully filled-in report, followed by the blank, copy-pasteable template. Load this when drafting a
report for the first time in a session, or when the shape of a section is unclear from `SKILL.md` alone. This repo
doesn't have `docs/RISK_REGISTER.md` or `docs/TECH_DEBT.md` yet, so the counts and row references below are
hypothetical -- a real report always computes its counts fresh against whatever those files actually contain when
they exist.

## Illustrative run

Suppose every filed follow-up is `DONE` -- 15 of 15. That is the expected steady state once a project's follow-up
funnel has been running for a while, not an error condition, so the report falls straight through to the wider
backlog rather than stopping at "nothing to report" (the first Anti-Pattern in `SKILL.md` exists precisely to head
off that shortcut).

```markdown
## Follow-up status

- 0 ACTIVE follow-ups (all 15 filed ones are DONE).

## Wider backlog

- Value-rubric top pick: [illustrative -- re-run the read protocol against the current context index; the top
  tier-1 PLAN/FINDING/KNOWN_ISSUE changes as items land and new ones are filed, so no single title stays accurate
  here].
- Reactivation-eligible DEFERRED item(s): none found in this run.
- Open RISK_REGISTER rows: 6 (of 9 total). Highest-leverage judgment call: row 3 ("Directions API retry budget
  under-documented", Accepted Risk) -- it's recurring friction (every future retry-tuning change risks
  rediscovering the undocumented cap the hard way) rather than a one-off fix, so closing it removes a repeating
  cost instead of a single instance of it.
- Open TECH_DEBT rows: 4 (rows 2-5, all Open). Quickest win: row 3 ("Two helper functions in `streetview.rs` doing
  the same dedupe check", Effort S) -- a mechanical single-file consolidation with no design decision attached.

## Recommended next item

**RISK_REGISTER row 3 -- Directions API retry budget under-documented.** Judgment call, not rubric-sorted (register
rows carry no `value` field -- see the Backlog Triage and Ranking reference). It outranks TECH_DEBT row 3 despite
row 3 being the cheaper, faster fix: row 3 only ever saves the next person who touches that one file, while row 3
of the register's cost compounds on every future change to retry/backoff tuning -- exactly the leverage-over-effort
tiebreak the value-rubric's grading criteria would apply if this row carried a formal grade. Next step: run
`risk-register` to check whether documenting the current retry cap resolves it outright, or whether it needs an
actual behaviour change first.
```

Two things worth noting about worked examples like this one: the value-rubric top pick is deliberately left as a
placeholder above, not a fabricated title -- the actual top item changes as work lands, and printing a fixed example
title here would go stale the moment it's read after a plan closes. A real report always computes it fresh. Second,
row counts like "6 of 9" and "4" should always be grep-verified counts of `Open` rows against the real files, not an
impression from skimming or a number carried over from an example.

## Blank template

```markdown
## Follow-up status

- N ACTIVE follow-up(s) found[, or "none -- all M are DONE"].
- For each ACTIVE one: {path} -- {title} -- {assessed: ready to close-as-superseded | needs promotion to the
  register | needs promotion to the tech-debt list | still genuinely open, no action yet}.

## Wider backlog

- Value-rubric top pick (if any): {path} -- {title} ({value}, tier 1).
- Reactivation-eligible DEFERRED item(s) (if any): {path} -- deferred_until {date} has passed.
- Open RISK_REGISTER rows: N (highest-leverage: row {#} -- {item}).
- Open TECH_DEBT rows: N (quickest win: row {#} -- {item}, Effort {S/M/L}).

## Recommended next item

**{title / row reference}** -- {one-paragraph rationale: why this over the runner-up, and which source it came
from}. Next step: {handoff, e.g. "apply follow-up-triage's flow to {path}" / "run risk-register to promote" /
"run plan-create to scope this"}.
```
