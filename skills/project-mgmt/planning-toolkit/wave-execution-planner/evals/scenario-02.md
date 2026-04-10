# Scenario 02: Create a Wave Execution Plan from an Existing Flat Task List

## User Prompt

Create a wave execution plan grouping these tasks into dependency-ordered waves.

## Existing task list

| ID | Description | Depends on |
|----|-------------|------------|
| T0.1 | Verify current behaviour and document baseline | none |
| T1.1 | Remove unused cloud infrastructure resource | T0.1 |
| T1.2 | Update consumers of that infrastructure resource | T0.1 |
| T2.1 | Remove business logic referencing the resource | T1.1, T1.2 |
| T2.2 | Remove tests referencing the resource | T1.1, T1.2 |
| T3.1 | Remove unused library dependencies | T2.1, T2.2 |
| T3.2 | Final verification and PR preparation | T2.1, T2.2 |

## Constraints

- Output to `.context/plans/infrastructure-cleanup.md`
- T1.1 and T1.2 touch independent files and can run in parallel
- T2.1 and T2.2 touch independent files and can run in parallel
- T3.1 and T3.2 can run in parallel
- Include an ASCII dependency graph
- Include a branch strategy with one branch per parallel phase
- Each wave must have a verification checklist that can be confirmed by running commands

## Expected Behavior

1. Write the wave document to `.context/plans/infrastructure-cleanup.md`
2. The dependency graph correctly reflects stated dependencies: T0.1 blocks T1.x; T1.x blocks T2.x; T2.x blocks T3.x
3. T0.1 is in Wave 1 alone; T1.x in Wave 2; T2.x in Wave 3; T3.x in Wave 4 — no task is grouped with a task it depends on
4. All 7 tasks (T0.1, T1.1, T1.2, T2.1, T2.2, T3.1, T3.2) appear in the document and each appears in exactly one wave
5. Waves 2, 3, and 4 are labelled "parallel" because each contains 2 independent tasks
6. Wave 1 (T0.1 alone) is labelled "sequential"
7. Each of the 4 waves includes a Verification checklist with at least one concrete runnable command
8. A Branch Strategy section exists with a table assigning one branch per parallel task
9. Write the document to exactly `.context/plans/infrastructure-cleanup.md`
10. Do not add dependencies that were not in the task list (e.g. do not make T3.1 depend on T3.2)
11. Every wave heading specifies its execution mode: (sequential) or (parallel)

## Success Criteria

- **wave-document-created**: A wave document exists at `.context/plans/infrastructure-cleanup.md`
- **dependency-dag-matches-stated-deps**: The dependency graph correctly reflects stated dependencies: T0.1 blocks T1.x; T1.x blocks T2.x; T2.x blocks T3.x
- **no-dependent-tasks-in-same-wave**: T0.1 is in Wave 1 alone; T1.x in Wave 2; T2.x in Wave 3; T3.x in Wave 4
- **all-tasks-appear-exactly-once**: All 7 tasks appear in the document and each appears in exactly one wave
- **parallel-waves-labelled-correctly**: Waves 2, 3, and 4 are labelled "parallel"
- **sequential-wave-labelled-correctly**: Wave 1 (T0.1 alone) is labelled "sequential"
- **verification-per-wave**: Each of the 4 waves includes a Verification checklist with at least one concrete runnable command
- **branch-strategy-present**: A Branch Strategy section exists with a table assigning one branch per parallel task
- **output-path-correct**: The wave document was written to exactly `.context/plans/infrastructure-cleanup.md`
- **no-invented-dependencies**: The agent did not add dependencies not in the task list
- **wave-headings-include-mode**: Every wave heading specifies its execution mode: (sequential) or (parallel)

## Failure Conditions

- Writes the document to a path other than `.context/plans/infrastructure-cleanup.md`
- The dependency graph does not match the stated task dependencies
- A task is grouped with a task it depends on (e.g. T0.1 in the same wave as T1.1)
- One or more tasks are omitted or appear in multiple waves
- Parallel waves (2, 3, 4) are not labelled "parallel"
- Wave 1 is not labelled "sequential"
- A wave is missing its Verification checklist
- No Branch Strategy table is included
- The agent invents new dependencies (e.g. makes T3.1 depend on T3.2)
- Wave headings omit execution mode
