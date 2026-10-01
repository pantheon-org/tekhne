# Scenario 4: Vendor and Promote a Pending Tool With a Repro File

## User Prompt

You are working in the research repository. `REVIEWED.md` has this entry for a tool triaged earlier:

```markdown
| 2026-03-02 | versatly-clawvault | tool | pending | Markdown memory vault with session lifecycle |
```

`references/versatly-clawvault.md` exists. The user writes: "Yes, vendor versatly/clawvault and promote it to a full analysis, including the repro file."

The repository URL is https://github.com/versatly/clawvault. After vendoring, the clone contains `benchmarks/` and `scripts/` directories, and `benchmarks/` holds a runnable harness.
