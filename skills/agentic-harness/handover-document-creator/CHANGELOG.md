# Changelog

All notable changes to the handover-document-creator skill.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this skill adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-09-28

### Added

- Write a structured session handover under `.context/handovers/`, with a JSON Schema for its frontmatter, a YAML template of required sections, and a shell validator.
- Added `evals/` (instructions.json plus scenario-1..3, each with task.md/criteria.json/capability.txt, and a summary.json) covering evidence-backed Completed items, linking rather than duplicating follow-ups in Outstanding, writing a new superseding handover instead of editing an old one in place, never using a bare `git stash`, and flipping status to `done` on pickup.
- Added `references/document-structure.md` and `references/typology-and-lifecycle.md`, moving detailed section-by-section and typology-boundary explanations out of `SKILL.md` so it stays a navigation hub.
