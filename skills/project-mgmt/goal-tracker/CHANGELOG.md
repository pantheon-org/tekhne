# Changelog

All notable changes to the goal-tracker skill.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this skill adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.0] - 2026-09-30

### Added

- Projects that require a goal for every conversation: draft the goal from the first
  request and start work only once the user confirms or corrects it.
- Persist each change to the goal file in the same turn, wherever the next session reads
  it, so an unfinished goal survives a removed worktree.
- When the user wraps up with items open, keep the goal active and write a linked
  handover with `handover-document-creator`.
- Anti-pattern: never start work on an unconfirmed goal.
- `evals/`: three instructions and scenarios 5 and 6 for the rules above; coverage stays
  at 100% (21 of 21).

### Changed

- `SKILL.md` tightened to raise the skill audit from 124/140 (B+) to 128/140 (A): the body
  is under 1200 tokens, more sentences lead with the action, References is a bullet list
  that keeps each load condition, and the duplicated `check` output now lives only in
  `references/file-shape.md`.

## [1.2.0] - 2026-09-14

### Changed

- Template is now `assets/templates/goal.yaml`, a declarative schema, matching the
  convention used across the estate. The 1.1.0 `assets/goal-template.md.tmpl` broke it
  on both format and location.
- `scripts/goal.sh new` emits the goal structure as printf arguments instead of
  substituting into a template file. No replacement mechanism is involved, so no
  character is reserved in the title.

### Removed

- `assets/goal-template.md.tmpl`.
- The `substitute()` helper, no longer needed once nothing is substituted.

## [1.1.0] - 2026-09-14

### Added

- `evals/` with an 18-instruction inventory, 100% coverage, and four scenarios
  covering clarity-test routing, verbatim summary reporting, the outside-reach
  confirmation rule, and promotion versus completion.
- Anti-Patterns section with NEVER statements, WHY lines, and a BAD/GOOD table
  comparison.
- Mindset section: an unevidenced status is worse than no status, and trust is
  asymmetric by reach.
- `references/file-shape.md`, holding the frontmatter contract, the items table,
  and a worked example moved out of SKILL.md.
- Category and priority frontmatter on every reference, plus lazy-load
  conditions in the References table.

### Changed

- SKILL.md trimmed from 197 to 131 lines and made a navigation hub.
- Description rewritten to drop keyword-stuffing conjunctions.
- Template renamed to `goal-template.md.tmpl` so markdownlint stops reading a
  template's placeholder heading as a second H1. Superseded in 1.2.0: that rename
  was the wrong reading of the lint failure.

### Fixed

- Repaired the evidence-table row damaged by an in-place substitution.

### Quality

- Audit score 82/140 (F) to 126/140 (A), zero errors, zero warnings.

## [1.0.0] - 2026-09-14

### Added

- Initial release. Records a session goal as a `.context/goals/` file and
  answers "what's left" with a summary capped at 100 words.
- Clarity test routing an unclear goal to `socratic-method` or
  `guided-interview`, and a clear one straight to work.
- Evidence model splitting items by reach. A `local` item needs a re-checkable
  pointer; an `outside` item needs the user's express confirmation and may
  never be self-attested by an agent.
- `awaiting` item state for outside work that is performed but unconfirmed, so
  it can be surfaced for confirmation instead of being closed or forgotten.
- Parking, which files a `.context/follow-ups/` entry in the same turn and
  removes the item from the goal.
- Promotion to a plan on any of three triggers: more than five items, a second
  worktree, or wave structure.
- `scripts/goal.sh` with `new`, `status`, `check`, and `park`.
