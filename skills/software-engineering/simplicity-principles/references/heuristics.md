# Simplicity Heuristics

Quick checks for KISS / YAGNI / DRY. Core guidance is in `SKILL.md`.

- KISS: could a junior read this in one pass? If not, simplify.
- YAGNI: is this parameter/hook used by a real caller today? If not, remove it.
- DRY: is the same *decision* encoded in two places? Dedupe knowledge, not incidental character overlap.
- Rule of three: tolerate duplication until the third occurrence reveals the shape.
