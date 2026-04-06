# Task: Create a wave execution plan from an existing flat task list

You have been given a flat list of tasks with explicit dependencies. Create a wave execution plan grouping these tasks into dependency-ordered waves.

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
