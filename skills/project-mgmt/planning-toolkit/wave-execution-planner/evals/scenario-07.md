# Scenario 07: Create a Wave Plan for a Research Consolidation

## User Prompt

Create a wave execution plan for the following research project. Write the wave document to `.context/plans/research-consolidation.md`.

## Requirements

The team needs to:

1. **Triage 6 new tools** — each tool gets its own branch and uses the `triage-tool` skill to produce a structured `references/{slug}.md`. These are independent of each other.

2. **Write a cross-cutting synthesis** — after triage is done, one agent reads all 6 `references/{slug}.md` files and extracts common patterns into an `ANALYSIS.md` Key Themes section. This is high-judgment work requiring deep comparison.

3. **Run a consolidation pass** — flip PUNCHLIST markers from `[ ]` to `[x]`, add REVIEWED.md rows for all new tools, run `python scripts/build_reference_index.py`. Mechanical; script-driven.

4. **Cleanup audit** — run lint, check consistency (no orphaned files, no stale markers), commit the result. Entirely mechanical.

## Constraints

- Steps 1 (triage) can all run in parallel.
- Steps 2 (synthesis) and 3 (consolidation) both depend on step 1 but are independent of each other.
- Step 4 (cleanup) depends on steps 2 and 3.
- Use one branch per parallel workstream.

## Expected Behavior

1. Include a Model column in every wave table in the document
2. Assign all 6 triage phases (skill-driven, template output) a Model value of "standard"
3. Assign the synthesis phase (cross-document analysis, high judgment) a Model value of "smart"
4. Assign the consolidation pass (PUNCHLIST flip, script run) a Model value of "fast"
5. Assign the cleanup audit phase (lint, consistency check) a Model value of "fast"
6. Write the wave document to `.context/plans/research-consolidation.md`

## Success Criteria

- **model-column-present-in-all-wave-tables**: Every wave table in the document includes a Model column
- **triage-phases-assigned-standard**: All 6 triage phases (skill-driven, template output) are assigned "standard" in the Model column
- **synthesis-phase-assigned-smart**: The synthesis phase (cross-document analysis, high judgment) is assigned "smart" in the Model column
- **consolidation-phase-assigned-fast**: The consolidation pass (PUNCHLIST flip, script run) is assigned "fast" in the Model column
- **cleanup-phase-assigned-fast**: The cleanup audit phase (lint, consistency check) is assigned "fast" in the Model column
- **document-written-to-correct-path**: The wave document is written to `.context/plans/research-consolidation.md`

## Failure Conditions

- Any wave table is missing the Model column
- Triage phases are assigned "smart" or "fast" instead of "standard"
- The synthesis phase is assigned "standard" or "fast" instead of "smart"
- The consolidation pass is assigned "standard" or "smart" instead of "fast"
- The cleanup audit is assigned "standard" or "smart" instead of "fast"
- The document is written to a path other than `.context/plans/research-consolidation.md`
