# Scenario 01: Task Tracker Base

## User Prompt

Create an Obsidian Bases file named `tasks.base` for a task-tracking vault.

**Requirements**

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

Provide the complete, valid YAML content for `tasks.base`.

## Expected Behavior

1. Include `file.hasTag("task")` (or equivalent) in global or view-level filters to scope the base to task-tagged notes
2. Use `file.basename` (not `file.name`) for the file name column so the `.md` extension is not shown
3. Define a days-until-due formula under the top-level `formulas` key
4. Guard the formula against missing due dates using `if(due, ..., "")` or equivalent
5. Reference the formula correctly as `formula.<name>` in the view's order list, matching the key in `formulas`
6. Apply a view-level filter on the "Incomplete" view to exclude done tasks (e.g., `status != "done"`)
7. Apply an Average summary to the days-until-due formula column in the "All Tasks" view
8. Produce syntactically valid YAML with consistent indentation

## Success Criteria

- **Tag filter present**: The global or view-level filters include `file.hasTag("task")` (or equivalent) to scope the base to task-tagged notes.
- **file.basename used for display**: The file name column uses `file.basename` (not `file.name`) so the `.md` extension is not shown.
- **days_until_due formula defined in formulas section**: A formula that computes days until due is defined under the top-level `formulas` key.
- **days_until_due formula guards against empty due date**: The formula wraps the date arithmetic in `if(due, ..., "")` or equivalent to avoid errors when the due property is absent.
- **Formula referenced correctly in order**: The days-until-due formula is referenced as `formula.<name>` in the view's order list, and this name matches the key defined in `formulas`.
- **Incomplete view uses view-level filter**: The "Incomplete" view has a `filters` clause that excludes done tasks at the view level, not via a global filter that would affect all views.
- **Summary on days-until-due formula**: The "All Tasks" view includes a `summaries` entry that applies Average (or another numeric summary) to the days-until-due formula column.
- **Valid YAML structure**: The output is syntactically valid YAML. All strings containing colons or special characters are quoted. Indentation is consistent.

## Failure Conditions

- No `file.hasTag("task")` filter present at global or view level
- File name column uses `file.name` instead of `file.basename`, showing the `.md` extension
- No formula for days-until-due is defined under the `formulas` key
- The formula does not guard against missing or empty due date values
- The formula is referenced by a name in the order list that does not match the key in `formulas`
- The "Incomplete" view's `status != "done"` filter is placed globally, affecting other views
- No `summaries` entry applies to the days-until-due formula in the "All Tasks" view
- Output YAML contains syntax errors or inconsistent indentation
