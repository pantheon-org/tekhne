# Task: Create a wave plan for a research consolidation

Create a wave execution plan for the following research project.
Write the wave document to `.context/plans/research-consolidation.md`.

## Requirements

The team needs to:

1. **Triage 6 new tools** — each tool gets its own branch and uses the `triage-tool` skill
   to produce a structured `references/{slug}.md`. These are independent of each other.

2. **Write a cross-cutting synthesis** — after triage is done, one agent reads all 6
   `references/{slug}.md` files and extracts common patterns into an `ANALYSIS.md`
   Key Themes section. This is high-judgment work requiring deep comparison.

3. **Run a consolidation pass** — flip PUNCHLIST markers from `[ ]` to `[x]`,
   add REVIEWED.md rows for all new tools, run `python scripts/build_reference_index.py`.
   Mechanical; script-driven.

4. **Cleanup audit** — run lint, check consistency (no orphaned files, no stale markers),
   commit the result. Entirely mechanical.

## Constraints

- Steps 1 (triage) can all run in parallel.
- Steps 2 (synthesis) and 3 (consolidation) both depend on step 1 but are independent of each other.
- Step 4 (cleanup) depends on steps 2 and 3.
- Use one branch per parallel workstream.
