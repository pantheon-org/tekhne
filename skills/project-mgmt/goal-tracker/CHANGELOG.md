# Changelog

All notable changes to the goal-tracker skill.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this skill adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
