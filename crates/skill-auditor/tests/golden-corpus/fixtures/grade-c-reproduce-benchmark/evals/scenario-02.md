# Scenario 02: Reproduce Benchmark — codebase-memory-mcp (Harness Failure)

## User Prompt

Reproduce the benchmark for `codebase-memory-mcp` (slug: `deusdata-codebase-memory-mcp`).

- The tool is vendored at `tools/deusdata-codebase-memory-mcp/` (pinned commit `d33b9e4`)
- The reference file is at `references/deusdata-codebase-memory-mcp.md`
- The analysis file is at `analysis/ANALYSIS-deusdata-codebase-memory-mcp.md`

When you attempt to run the benchmark harness, it fails with the following error:

```
Error: Cannot find module 'better-sqlite3'
Require stack:
- /tools/deusdata-codebase-memory-mcp/src/graph/db.js
```

Document this failure and produce `benchmarks/sources/deusdata-codebase-memory-mcp-repro.md` with outcome `failed`.

## Expected Behavior

1. Create `benchmarks/sources/deusdata-codebase-memory-mcp-repro.md` with valid YAML frontmatter containing all required fields
2. Set `outcome` to exactly `failed` — not `unverified` or `partially-verified`
3. Quote the `Cannot find module 'better-sqlite3'` error verbatim in the Reproduction attempt section — do not paraphrase
4. Refrain from inventing or copying verified figures from the README; state clearly that no metrics were obtained
5. Record self-reported figures from the README, clearly marked `(as reported)`, in a separate section
6. Document in the How to reproduce section that `npm install` or equivalent is required before the harness can run
7. Capture stderr output (do not suppress it with `2>/dev/null` or equivalent)
8. Update `analysis/ANALYSIS-deusdata-codebase-memory-mcp.md` Stage 2.2 with a reference to the repro file and note `outcome: failed`
9. Leave no angle-bracket placeholders in the produced file

## Success Criteria

- **benchmarks/sources/deusdata-codebase-memory-mcp-repro.md is created with valid YAML frontmatter**: The file is created at the correct path and opens with a YAML frontmatter block containing all required fields.
- **outcome is set to failed**: The frontmatter outcome field is exactly 'failed', not 'unverified' or 'partially-verified'.
- **Error message is quoted verbatim in Reproduction attempt section**: The Cannot find module 'better-sqlite3' error is reproduced verbatim in the repro file — not paraphrased.
- **No verified figures are fabricated**: The agent does not invent or copy verified figures from the README when the harness did not run. The Reproduction attempt section clearly states no metrics were obtained.
- **Reported figures section present with (as reported) markers**: Self-reported figures from the README are still recorded, clearly marked (as reported), separate from the failed run.
- **How to reproduce section documents the fix needed**: The section notes that npm install or equivalent is required before the harness can run.
- **stderr is not suppressed**: The agent does not use 2>/dev/null or equivalent when running the harness. The error output is captured and quoted.
- **ANALYSIS Stage 2.2 updated with failure pointer**: analysis/ANALYSIS-deusdata-codebase-memory-mcp.md Stage 2.2 references the repro file and notes outcome: failed.
- **No unfilled placeholders**: The produced file contains no angle-bracket placeholders.

## Failure Conditions

- Agent creates the repro file at the wrong path or with missing frontmatter fields
- Agent sets `outcome` to `unverified` or `partially-verified` instead of `failed`
- Agent paraphrases rather than quoting the error message verbatim
- Agent fabricates or copies verified metric figures when the harness did not run successfully
- Agent omits the reported figures section or fails to mark figures `(as reported)`
- Agent omits the `npm install` fix from the How to reproduce section
- Agent suppresses stderr output, losing the error message
- Agent fails to update Stage 2.2 in the ANALYSIS file
- Agent leaves angle-bracket placeholders in the produced file
