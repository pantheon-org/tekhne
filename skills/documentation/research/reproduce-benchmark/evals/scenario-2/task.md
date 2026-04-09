# Reproduce Benchmark — codebase-memory-mcp (harness failure)

## Task

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
