# Scenario 01: Create a Wave Execution Plan for a CLI Refactor

## User Prompt

Create a wave execution plan that groups tasks into dependency-ordered waves for parallel subagent execution.

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

## Expected Behavior

1. Write the wave document to `.context/plans/scripts-cli-refactor.md`
2. Include a Dependency Graph section with an ASCII DAG showing which waves unlock which
3. Assign all 9 tasks to waves with each task appearing in exactly one wave
4. Ensure no wave contains tasks that depend on each other (CLI scaffold and lib extraction are NOT in the same wave; cleanup is NOT in the same wave as commands)
5. Label waves containing multiple independent tasks (lib extractions Wave 1, command implementations Wave 3) as "parallel"
6. Label waves with a single task (CLI scaffold Wave 2, cleanup Wave 4) as "sequential"
7. Include a Verification checklist per wave with at least one concrete runnable check
8. Include a Branch Strategy section with a table assigning one branch per parallel task using the pattern `refactor/<task-slug>`
9. Write the document to exactly `.context/plans/scripts-cli-refactor.md`
10. Structure the document with exactly 4 waves matching the 4 dependency levels
11. Include a Goal section describing the purpose of the refactor in one paragraph
12. Include the execution mode in every wave heading (e.g. `### Wave 1 — Extract shared lib (parallel)`)

## Success Criteria

- **wave-document-created**: A wave document exists at `.context/plans/scripts-cli-refactor.md`
- **dependency-graph-section-present**: The document contains a Dependency Graph section with an ASCII DAG showing which waves unlock which
- **all-tasks-assigned-to-exactly-one-wave**: All 9 tasks (4 lib extractions, 1 CLI scaffold, 3 commands, 1 cleanup) appear in the document and each appears in exactly one wave
- **no-dependent-tasks-in-same-wave**: No wave contains tasks that depend on each other
- **parallel-waves-labelled-correctly**: Waves containing multiple independent tasks are labelled "parallel"
- **sequential-waves-labelled-correctly**: Waves with a single task are labelled "sequential"
- **verification-checklist-per-wave**: Each wave section includes a Verification checklist with at least one concrete runnable check
- **branch-strategy-table-present**: The document includes a Branch Strategy section with a table assigning one branch per parallel task using the pattern `refactor/<task-slug>`
- **output-path-correct**: The wave document was written to exactly `.context/plans/scripts-cli-refactor.md`
- **wave-count-matches-dag**: The document contains exactly 4 waves matching the 4 dependency levels
- **goal-section-present**: The document includes a Goal section describing the purpose of the refactor in one paragraph
- **wave-headings-include-execution-mode**: Every wave heading includes its execution mode in parentheses

## Failure Conditions

- Writes the document to a path other than `.context/plans/scripts-cli-refactor.md`
- Omits the Dependency Graph or ASCII DAG
- Assigns a task to multiple waves or omits a task entirely
- Places CLI scaffold in the same wave as lib extraction (dependency violation)
- Places cleanup in the same wave as command implementations (dependency violation)
- Labels parallel waves as "sequential" or vice versa
- Omits the Verification checklist for any wave
- Does not include a Branch Strategy table
- Creates more or fewer than 4 waves
- Omits the Goal section
- Omits execution mode from wave headings
