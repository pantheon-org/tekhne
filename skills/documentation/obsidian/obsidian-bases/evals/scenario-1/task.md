# Scenario 1: Task Tracker Base

Create an Obsidian Bases file named `tasks.base` for a task-tracking vault.

## Requirements

- **Scope**: Show all notes tagged `#task`.
- **Columns** (in order): file basename, status, due date, and a formula column showing
  the number of days until the task is due.
- **Formula**: The days-until-due formula must handle notes where the due date property
  is missing or empty (not every task has a deadline).
- **Views**:
  1. A default table view called "All Tasks" that shows every task.
  2. A second table view called "Incomplete" that shows only tasks where
     `status` is not equal to `"done"`.
- **Sorting**: In the "All Tasks" view, sort by the days-until-due formula in ascending
  order (most urgent first).
- **Summary**: Show the average days-until-due in the "All Tasks" view footer.

## Deliverable

Provide the complete, valid YAML content for `tasks.base`.
