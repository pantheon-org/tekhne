---
name: tech-debt
description: "Maintain docs/TECH_DEBT.md, the living list of code-level cleanup that isn't a risk, a shortcut, or an open decision (that's the risk-register skill's job) and isn't a future feature (that's ROADMAP.md's job). Use when a lint/format/style violation slips past an existing gate, a small known cleanup is noticed while working on something else, or a filed item is fixed and needs removing. DO NOT use for anything with a real cost or risk if left unaddressed (use risk-register instead), for aislop baseline findings (already lifecycle-managed in .aislop/baseline.json), or for anything confidential. Triggers: 'add to tech debt', 'log this as cleanup', 'track this lint violation', 'mark tech-debt item #N fixed', 'what's on the tech debt list'."
---

# Tech Debt

A single committed table (`docs/TECH_DEBT.md`) of code-level cleanup that isn't a risk, a shortcut, or an open
decision (`docs/RISK_REGISTER.md`'s job) and isn't a future feature (`docs/ROADMAP.md`'s job) -- just things that
should get tidied up when there's spare time. Sibling to the `risk-register` skill, sharing its scaffolding pattern
(dedicated skill, YAML row template, JSON schema, validation script) but not its append-only invariant: a tech-debt
row is deleted once fixed, not archived, because there is no decision to keep evidence of.

## Rules

1. **A row is deleted once fixed, not archived.** Unlike `risk-register`'s `docs/RISK_REGISTER.md`, there is no
   append-only guard here and no technical backstop preventing deletion -- that is the deliberate design, not a gap.
   A concurrent-edit merge that drops a still-open row alongside a legitimate deletion is a known, accepted
   limitation of this model; it is not engineered around, because the whole point of choosing this model over
   `risk-register`'s is to avoid that machinery for low-stakes items.
2. **Enforcement is local pre-commit only, not CI-backed, not GitHub-server-enforced.** `scripts/validate-tech-debt-schema.sh`,
   wired into `pre-commit` via lefthook, blocks a locally-committed malformed row. A commit made through the GitHub
   web UI or API, `git commit --no-verify`, or from a machine without lefthook installed is not stopped by this.
3. **Due diligence runs in both directions.** Before filing a row, confirm the item is genuinely just cleanup --not a
   risk, shortcut, or open decision that belongs in `docs/RISK_REGISTER.md` instead. Before deleting a row (marking
   it fixed), confirm the fix didn't reveal a real cost or risk; if it did, promote it to `docs/RISK_REGISTER.md`
   first (see Workflow) rather than letting the row vanish with the discovery unrecorded.

## Prerequisites

- `docs/TECH_DEBT.md` exists. If this is the first entry in a project and the file doesn't exist yet, create it with
  a `# Tech Debt` title, an intro paragraph drawing the boundary against `RISK_REGISTER.md`/`ROADMAP.md`/`aislop`'s
  baseline, the enforcement-scope caveat from Rule 2, and an empty table using the columns in
  `assets/templates/tech-debt-row-template.yaml`.
- `assets/templates/tech-debt-row-template.yaml` -- the shape of a single row, illustrated
- `assets/schemas/tech-debt-row.schema.json` -- the same shape, as enforceable constraints
- `scripts/validate-tech-debt-schema.sh`, wired into `pre-commit` (lefthook), so a malformed row can never be
  committed. No append-only companion (Rule 1).
- `jq` (schema constraints are read at runtime, never hardcoded -- see Scripts) and `shellcheck` (CI and pre-commit
  both gate the validator script), both pinned in `mise.toml`.

## When to Use

- A lint, format, or style violation is noticed that an existing gate doesn't fail the build on (a warning, not an
  error; or a path a gate doesn't cover) -- the exact situation the seed row in `docs/TECH_DEBT.md` documents.
- A small, low-stakes cleanup item is noticed while working on something unrelated, and isn't worth fixing in the
  same change.
- A filed item is actually fixed and its row needs deleting.
- Someone asks what code-level cleanup is outstanding.

## When NOT to Use

- **Anything with a real cost or risk if left unaddressed** -- that's `docs/RISK_REGISTER.md` (`risk-register`
  skill), not this doc. If a tech-debt item turns out to carry one, promote it (see Workflow).
- **`aislop` baseline findings** (`.aislop/baseline.json`) -- already a scored, regression-tracked inventory of
  code-quality findings with its own mechanism. Don't duplicate a finding aislop already tracks.
- **Anything confidential** -- this file is committed and visible to everyone with repo access.

## Area Taxonomy and Row Schema

`area` is a small, tech-debt-specific set of values (currently `Code quality` and `Testing`), not a closed JSON
Schema `enum` and not the `.context/` themes vocabulary. RECOMMENDED reading before filing or resolving a row: see
[Area Taxonomy and Row Schema](references/area-taxonomy-and-row-schema.md) for the full value list, the revisit
trigger for splitting an overloaded area, and the complete column-by-column row schema.

## Workflow

### Adding an entry

1. **Confirm this is genuinely just cleanup, not a risk, shortcut, or open decision** (Rule 3). If it carries a real
   cost or failure mode if left unaddressed, it belongs in `docs/RISK_REGISTER.md` instead.
2. Read the current last row's `#` in `docs/TECH_DEBT.md`; the new row's `#` is that value + 1 (rows are freely
   reused after deletion, so don't assume the max `#` ever seen -- just the max currently present).
3. Write a self-contained `Description` -- don't assume the reader has whatever prompted this open in front of them.
   **Never cite a `.context/` file as where the extra detail lives** -- `.context/` is gitignored local scratch, so a
   reference to it may be dead for every reader but the one who filed it. Inline the essential facts directly, or
   point at a committed doc (an ADR, an MR description, another `docs/*.md` page) instead.
4. Append the row, matching `assets/templates/tech-debt-row-template.yaml`'s field shapes (`Area` from the current
   taxonomy, `Effort` from its enum, `Since` as `YYYY-MM-DD`, `Status` starting `Open`).
5. Run `scripts/validate-tech-debt-schema.sh` before committing (the pre-commit hook runs it too, but check early to
   avoid a bounced commit).

### Resolving an entry

1. Find the row by its `#` or Item name.
2. **Confirm the fix didn't reveal a real cost or risk** (Rule 3). If it did, add a row to `docs/RISK_REGISTER.md`
   first, with the new row's `Description` referencing this tech-debt item by name -- since this row is about to be
   deleted and would otherwise leave no trace of where the risk was first noticed.
3. Delete the row entirely. Unlike `risk-register`, there is no `Resolved` state to flip to -- deletion is the
   resolution.

### Relationship to `risk-register`

The boundary between the two docs is exactly the kind of judgment call people get wrong -- that's the reason this
skill's grounding finding (a 2026-07-31 governance-model analysis) exists. If a
`risk-register` row turns out, on inspection, to actually be cleanup rather than a real risk, that's a correction to
make via the `risk-register` skill (amend or, if genuinely mis-filed, raise it there), not a silent move into this
doc.

## Mindset

- **Rule 1 is a deliberate tradeoff, not an oversight.** `risk-register`'s append-only guard exists to preserve a
  decision audit trail; tech debt carries no decision to preserve evidence of. Don't add append-only machinery here
  "for consistency" -- that would defeat the reason this skill exists as a separate, lighter sibling.
  See the tech-debt-vs-risk-register governance rationale that grounds this skill for the full reasoning.
- This is a PoC-phase project: proportionate is fine. TYPICALLY a `docs/TECH_DEBT.md` with one honest row is more
  useful than an empty one or no doc at all -- RECOMMENDED to ship with whatever real rows exist rather than waiting
  for a "complete" sweep.
- BY DEFAULT, resolve the risk-vs-cleanup judgment call in Rule 3 by asking "does this carry a cost if nobody ever
  touches it again?" PREFER filing to `docs/RISK_REGISTER.md` UNLESS the answer is a clear no.
- **Kill criterion**: if zero rows beyond the one seeded on launch have been added to `docs/TECH_DEBT.md` by
  **2026-10-31** (90 days from creation), delete `docs/TECH_DEBT.md` and this skill -- the doc isn't earning its
  keep if nobody is using it. The 90-day window is tied to this skill's 2026-07-31 seeding date, the same audit
  referenced below.

## Anti-Patterns

**NEVER** add an append-only guard or a `Resolved` status value to this doc.
**WHY:** Both exist in `risk-register` to preserve a decision audit trail. Tech debt carries no decision -- adding
either here just re-creates the machinery this skill was deliberately built to avoid.
**BAD:** "For consistency with risk-register, let's also block row deletion here."
**GOOD:** Delete the row. That's the resolution mechanism, not a shortcut around one.

**NEVER** file a real risk or open decision here because it's less ceremony than `risk-register`.
**WHY:** `docs/TECH_DEBT.md` has no append-only guard and no audit trail -- a real risk filed here can be silently
lost the moment its row is deleted, with nothing to notice.
**BAD:** Filing "Directions API retries could silently exhaust the free quota" as tech debt to skip risk-register's due-diligence step.
**GOOD:** File it in `docs/RISK_REGISTER.md`, where deletion is blocked and the item can't quietly disappear.

**NEVER** delete a row without checking whether the fix revealed a real cost or risk (Rule 3).
**WHY:** Deletion is irreversible in practice here -- there's no technical backstop, so a real risk "resolved away"
by deletion instead of promoted leaves no trace and no failing check to catch the mistake.
**BAD:** Deleting the lint-violation row the moment the lint is fixed, without checking whether the underlying test
gap it was masking is itself a bigger problem.
**GOOD:** Confirm the fix is complete and clean, then delete -- or, if it wasn't, promote to `risk-register` first.

**NEVER** cite a `.context/` file as where a row's extra detail lives.
**WHY:** `.context/` is gitignored local scratch, not a committed, durable record -- a reference to it may point at
something no other reader, machine, or fresh clone can ever open, the moment the row is committed.
**BAD:** `Description: ... (.context/findings/2026-07-31-foo.md, finding M2)`
**GOOD:** Inline the essential fact in the row itself, or reference a committed doc (ADR, MR, another `docs/*.md`
page) if it doesn't fit.

**NEVER** invent placeholder rows to make the doc look more populated than it is.
**WHY:** A fabricated row is worse than an honest gap -- it looks like real signal to the next reader and wastes
their time chasing cleanup that doesn't exist. The 2026-07-31 launch audit found exactly one genuine candidate and
shipped with that rather than padding the count.
**BAD:** Adding a vague "clean up some old code somewhere" row just to reach a target count.
**GOOD:** Ship with however many real, verified rows exist -- one is more useful than five fabricated ones.

## Scripts

One check -- no append-only companion (Rule 1), unlike `risk-register`'s two:

```bash
scripts/validate-tech-debt-schema.sh   # Content: does every row match tech-debt-row.schema.json?
```

## Troubleshooting

```text
Problem                                             | Solution
Pre-commit blocks: a field doesn't match            | validate-tech-debt-schema.sh caught a malformed field --
                                                     | fix it to match tech-debt-row.schema.json's constraints.
Pre-commit blocks: schema required fields mismatch  | The script's column order is out of sync with a schema edit --
                                                     | update validate-tech-debt-schema.sh's expected_fields.
Not sure what the next # should be                  | Read the last row of the table; next # = its # + 1 (# is
                                                     | reused after deletion, so this is the max currently present,
                                                     | not the max ever assigned).
Unsure whether something belongs here or in         | Does it carry a real cost/risk if left unaddressed? -> register
docs/RISK_REGISTER.md                               | row. Is it just cleanup for spare time? -> tech-debt row.
```

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Area values, the revisit/split trigger, full row schema, a worked example row | [Area Taxonomy and Row Schema](references/area-taxonomy-and-row-schema.md) | Filing a new row or unsure which `Area` value or column format applies |
| Row shape as illustrated YAML | `assets/templates/tech-debt-row-template.yaml` | Drafting a new row before appending it |
| Row shape as enforceable constraints | `assets/schemas/tech-debt-row.schema.json` | Checking what the validator script actually enforces |
| Deciding whether an item is cleanup or a real risk | `risk-register` skill | Rule 3's due-diligence check, in either direction |
