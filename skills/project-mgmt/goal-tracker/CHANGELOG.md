# Changelog

All notable changes to the goal-tracker skill.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this skill adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
  template's placeholder heading as a second H1.

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
