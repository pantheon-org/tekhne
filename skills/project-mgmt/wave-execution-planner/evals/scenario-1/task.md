# Task: Create a wave execution plan for a CLI refactor

You have been given the following requirements. Create a wave execution plan that groups tasks into dependency-ordered waves for parallel subagent execution.

## Requirements

Refactor a scripts directory into a unified Cliffy CLI. The work breaks down as follows:

**Track 1 — Shared library extraction** (no prerequisites):
- Extract `scripts/lib/http.ts` (HTTP utilities)
- Extract `scripts/lib/dates.ts` (date formatting helpers)
- Extract `scripts/lib/wikidata.ts` (Wikidata query helpers)
- Extract `scripts/lib/paths.ts` (path resolution helpers)

These four files are completely independent of each other.

**Track 2 — CLI scaffold** (requires all Track 1 files to exist):
- Install Cliffy and scaffold `scripts/cli.ts` as the entry point

**Track 3 — Command implementations** (requires CLI scaffold):
- Implement `scripts/commands/snapshot.ts`
- Implement `scripts/commands/party-meta.ts`
- Implement `scripts/commands/timeline.ts`

These three commands are independent of each other but all import from the shared lib.

**Track 4 — Cleanup** (requires all Track 3 commands to exist):
- Remove old scripts, update `package.json`, delete legacy entry points

## Constraints

- Output the wave document to `.context/plans/scripts-cli-refactor.md`
- Each parallel task should get its own branch using the pattern `refactor/<task-slug>`
- Include a dependency graph, branch strategy table, and per-wave verification checklist
- The verification for the shared-lib wave should check that the TypeScript compiles and no old import paths remain
