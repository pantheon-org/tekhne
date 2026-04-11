# Scenario 03: Reading List Base with Cards View

## User Prompt

Create an Obsidian Bases file named `reading-list.base` for a book-tracking vault.

**Requirements**

- **Scope**: Show all notes tagged `#book`.
- **View type**: A `cards` view (not a table). Name it "Library".
- **Columns** (in order in the cards view): file basename, author, status, and an
  estimated reading time formula.
- **Reading time formula**: Compute estimated reading time in minutes from a `pages`
  frontmatter property. Assume 2 minutes per page. The formula must safely handle notes
  where `pages` is not set.
- **Grouping**: Group the cards view by the `status` property in ascending order.
  Expected status values are `"to-read"`, `"reading"`, and `"done"`.
- **Second view**: Add a simple table view called "Stats" that shows file basename,
  pages, and the reading time formula, with a Sum summary on the pages column.

Provide the complete, valid YAML content for `reading-list.base`.

## Expected Behavior

1. Include `file.hasTag("book")` in global filters to scope the base to book-tagged notes
2. Set the primary view `type` to `cards`, not `table` or `list`
3. Define a reading-time formula under the top-level `formulas` key
4. Guard the formula against missing `pages` values using `if(pages, ..., "")` or equivalent
5. Include a `groupBy` clause in the cards view that groups by the `status` property
6. Use `file.basename` (not `file.name`) in both views' order lists
7. Add a second view of type `table` named "Stats" that includes the reading-time formula and applies a Sum summary to the pages column
8. Produce syntactically valid YAML

## Success Criteria

- **Book tag filter present**: The global filters include `file.hasTag("book")` to scope the base to book-tagged notes.
- **Cards view used**: The primary view has `type: cards`, not `type: table` or `type: list`.
- **reading_time formula defined in formulas section**: A formula that computes estimated reading time from pages is defined under the top-level `formulas` key.
- **reading_time formula guards against missing pages**: The formula wraps the pages arithmetic in `if(pages, ..., "")` or equivalent to handle notes without a pages property.
- **Cards view groups by status**: The cards view includes a `groupBy` clause that groups by the `status` property.
- **file.basename used in order (not file.name)**: The order list in both views uses `file.basename` rather than `file.name` for displaying note titles.
- **Stats table view present with Sum summary**: A second view of type `table` named "Stats" (or similar) is present, includes the reading-time formula in its order list, and applies a Sum summary to the pages column.
- **Valid YAML structure**: The output is syntactically valid YAML. All special characters in strings are quoted. Indentation is consistent.

## Failure Conditions

- No `file.hasTag("book")` filter present in global filters
- Primary view type is `table` or `list` instead of `cards`
- No reading-time formula defined under the `formulas` key
- Formula does not guard against missing `pages` values
- Cards view has no `groupBy` clause for `status`
- Any view uses `file.name` instead of `file.basename`
- No second "Stats" table view present, or it lacks a Sum summary on the pages column
- Output YAML contains syntax errors
